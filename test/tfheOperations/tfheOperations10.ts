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
import { createInstances, decrypt128, decrypt256, decryptBool } from '../instance';
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

    const contract10 = await deployTfheTestFixture10();
    this.contract10Address = await contract10.getAddress();
    this.contract10 = contract10;

    const contract11 = await deployTfheTestFixture11();
    this.contract11Address = await contract11.getAddress();
    this.contract11 = contract11;

    const instances = await createInstances(this.signers);
    this.instances = instances;
  });

  it('test operator "le" overload (euint128, euint128) => ebool test 1 (340282366920938463463373992528465349581, 340282366920938463463372840104721436799)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463373992528465349581n);
    input.add128(340282366920938463463372840104721436799n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.le_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint128, euint128) => ebool test 2 (340282366920938463463372840104721436795, 340282366920938463463372840104721436799)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463372840104721436795n);
    input.add128(340282366920938463463372840104721436799n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.le_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint128) => ebool test 3 (340282366920938463463372840104721436799, 340282366920938463463372840104721436799)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463372840104721436799n);
    input.add128(340282366920938463463372840104721436799n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.le_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint128) => ebool test 4 (340282366920938463463372840104721436799, 340282366920938463463372840104721436795)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463372840104721436799n);
    input.add128(340282366920938463463372840104721436795n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.le_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint128) => ebool test 1 (340282366920938463463368309276517872429, 340282366920938463463372024309642476529)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463368309276517872429n);
    input.add128(340282366920938463463372024309642476529n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.lt_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint128, euint128) => ebool test 2 (340282366920938463463368309276517872425, 340282366920938463463368309276517872429)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463368309276517872425n);
    input.add128(340282366920938463463368309276517872429n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.lt_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint128, euint128) => ebool test 3 (340282366920938463463368309276517872429, 340282366920938463463368309276517872429)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463368309276517872429n);
    input.add128(340282366920938463463368309276517872429n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.lt_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint128) => ebool test 4 (340282366920938463463368309276517872429, 340282366920938463463368309276517872425)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463368309276517872429n);
    input.add128(340282366920938463463368309276517872425n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.lt_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint128, euint128) => euint128 test 1 (340282366920938463463369507025632955721, 340282366920938463463371759125181075731)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463369507025632955721n);
    input.add128(340282366920938463463371759125181075731n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.min_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(340282366920938463463369507025632955721n);
  });

  it('test operator "min" overload (euint128, euint128) => euint128 test 2 (340282366920938463463369507025632955717, 340282366920938463463369507025632955721)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463369507025632955717n);
    input.add128(340282366920938463463369507025632955721n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.min_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(340282366920938463463369507025632955717n);
  });

  it('test operator "min" overload (euint128, euint128) => euint128 test 3 (340282366920938463463369507025632955721, 340282366920938463463369507025632955721)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463369507025632955721n);
    input.add128(340282366920938463463369507025632955721n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.min_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(340282366920938463463369507025632955721n);
  });

  it('test operator "min" overload (euint128, euint128) => euint128 test 4 (340282366920938463463369507025632955721, 340282366920938463463369507025632955717)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463369507025632955721n);
    input.add128(340282366920938463463369507025632955717n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.min_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(340282366920938463463369507025632955717n);
  });

  it('test operator "max" overload (euint128, euint128) => euint128 test 1 (340282366920938463463374311959896511759, 340282366920938463463367499312160836031)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463374311959896511759n);
    input.add128(340282366920938463463367499312160836031n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.max_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(340282366920938463463374311959896511759n);
  });

  it('test operator "max" overload (euint128, euint128) => euint128 test 2 (340282366920938463463367499312160836027, 340282366920938463463367499312160836031)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463367499312160836027n);
    input.add128(340282366920938463463367499312160836031n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.max_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(340282366920938463463367499312160836031n);
  });

  it('test operator "max" overload (euint128, euint128) => euint128 test 3 (340282366920938463463367499312160836031, 340282366920938463463367499312160836031)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463367499312160836031n);
    input.add128(340282366920938463463367499312160836031n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.max_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(340282366920938463463367499312160836031n);
  });

  it('test operator "max" overload (euint128, euint128) => euint128 test 4 (340282366920938463463367499312160836031, 340282366920938463463367499312160836027)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463367499312160836031n);
    input.add128(340282366920938463463367499312160836027n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.max_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(340282366920938463463367499312160836031n);
  });

  it('test operator "add" overload (euint128, euint256) => euint256 test 1 (2, 170141183460469231731687303715884105729)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(2n);
    input.add256(170141183460469231731687303715884105729n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.add_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(170141183460469231731687303715884105731n);
  });

  it('test operator "add" overload (euint128, euint256) => euint256 test 2 (170141183460469231731686007573348351546, 170141183460469231731686007573348351548)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(170141183460469231731686007573348351546n);
    input.add256(170141183460469231731686007573348351548n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.add_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(340282366920938463463372015146696703094n);
  });

  it('test operator "add" overload (euint128, euint256) => euint256 test 3 (170141183460469231731686007573348351548, 170141183460469231731686007573348351548)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(170141183460469231731686007573348351548n);
    input.add256(170141183460469231731686007573348351548n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.add_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(340282366920938463463372015146696703096n);
  });

  it('test operator "add" overload (euint128, euint256) => euint256 test 4 (170141183460469231731686007573348351548, 170141183460469231731686007573348351546)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(170141183460469231731686007573348351548n);
    input.add256(170141183460469231731686007573348351546n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.add_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(340282366920938463463372015146696703094n);
  });

  it('test operator "sub" overload (euint128, euint256) => euint256 test 1 (340282366920938463463369549263218693865, 340282366920938463463369549263218693865)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463369549263218693865n);
    input.add256(340282366920938463463369549263218693865n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.sub_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint128, euint256) => euint256 test 2 (340282366920938463463369549263218693865, 340282366920938463463369549263218693861)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463369549263218693865n);
    input.add256(340282366920938463463369549263218693861n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.sub_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint128, euint256) => euint256 test 1 (2, 85070591730234615865843651857942052865)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(2n);
    input.add256(85070591730234615865843651857942052865n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.mul_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(170141183460469231731687303715884105730n);
  });

  it('test operator "mul" overload (euint128, euint256) => euint256 test 2 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(9223372036854775809n);
    input.add256(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.mul_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "mul" overload (euint128, euint256) => euint256 test 3 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(9223372036854775809n);
    input.add256(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.mul_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "mul" overload (euint128, euint256) => euint256 test 4 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(9223372036854775809n);
    input.add256(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.mul_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "and" overload (euint128, euint256) => euint256 test 1 (340282366920938463463366718158519885511, 115792089237316195423570985008687907853269984665640564039457581748247155679323)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463366718158519885511n);
    input.add256(115792089237316195423570985008687907853269984665640564039457581748247155679323n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.and_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(340282366920938463463366717989234606147n);
  });

  it('test operator "and" overload (euint128, euint256) => euint256 test 2 (340282366920938463463366718158519885507, 340282366920938463463366718158519885511)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463366718158519885507n);
    input.add256(340282366920938463463366718158519885511n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.and_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(340282366920938463463366718158519885507n);
  });

  it('test operator "and" overload (euint128, euint256) => euint256 test 3 (340282366920938463463366718158519885511, 340282366920938463463366718158519885511)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463366718158519885511n);
    input.add256(340282366920938463463366718158519885511n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.and_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(340282366920938463463366718158519885511n);
  });

  it('test operator "and" overload (euint128, euint256) => euint256 test 4 (340282366920938463463366718158519885511, 340282366920938463463366718158519885507)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463366718158519885511n);
    input.add256(340282366920938463463366718158519885507n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.and_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(340282366920938463463366718158519885507n);
  });

  it('test operator "or" overload (euint128, euint256) => euint256 test 1 (340282366920938463463366429103695675815, 115792089237316195423570985008687907853269984665640564039457583383018428109749)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463366429103695675815n);
    input.add256(115792089237316195423570985008687907853269984665640564039457583383018428109749n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.or_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457583999099276579767n);
  });

  it('test operator "or" overload (euint128, euint256) => euint256 test 2 (340282366920938463463366429103695675811, 340282366920938463463366429103695675815)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463366429103695675811n);
    input.add256(340282366920938463463366429103695675815n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.or_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(340282366920938463463366429103695675815n);
  });

  it('test operator "or" overload (euint128, euint256) => euint256 test 3 (340282366920938463463366429103695675815, 340282366920938463463366429103695675815)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463366429103695675815n);
    input.add256(340282366920938463463366429103695675815n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.or_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(340282366920938463463366429103695675815n);
  });

  it('test operator "or" overload (euint128, euint256) => euint256 test 4 (340282366920938463463366429103695675815, 340282366920938463463366429103695675811)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463366429103695675815n);
    input.add256(340282366920938463463366429103695675811n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.or_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(340282366920938463463366429103695675815n);
  });

  it('test operator "xor" overload (euint128, euint256) => euint256 test 1 (340282366920938463463370357504982561043, 115792089237316195423570985008687907853269984665640564039457576209743222065387)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463370357504982561043n);
    input.add256(115792089237316195423570985008687907853269984665640564039457576209743222065387n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.xor_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(115792089237316195423570985008687907852929702298719625575994215220393361046008n);
  });

  it('test operator "xor" overload (euint128, euint256) => euint256 test 2 (340282366920938463463370357504982561039, 340282366920938463463370357504982561043)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463370357504982561039n);
    input.add256(340282366920938463463370357504982561043n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.xor_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(28n);
  });

  it('test operator "xor" overload (euint128, euint256) => euint256 test 3 (340282366920938463463370357504982561043, 340282366920938463463370357504982561043)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463370357504982561043n);
    input.add256(340282366920938463463370357504982561043n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.xor_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint128, euint256) => euint256 test 4 (340282366920938463463370357504982561043, 340282366920938463463370357504982561039)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463370357504982561043n);
    input.add256(340282366920938463463370357504982561039n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.xor_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(28n);
  });

  it('test operator "eq" overload (euint128, euint256) => ebool test 1 (340282366920938463463367588896596491863, 115792089237316195423570985008687907853269984665640564039457578982740222216241)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463367588896596491863n);
    input.add256(115792089237316195423570985008687907853269984665640564039457578982740222216241n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.eq_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint256) => ebool test 2 (340282366920938463463367588896596491859, 340282366920938463463367588896596491863)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463367588896596491859n);
    input.add256(340282366920938463463367588896596491863n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.eq_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint256) => ebool test 3 (340282366920938463463367588896596491863, 340282366920938463463367588896596491863)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463367588896596491863n);
    input.add256(340282366920938463463367588896596491863n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.eq_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint128, euint256) => ebool test 4 (340282366920938463463367588896596491863, 340282366920938463463367588896596491859)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463367588896596491863n);
    input.add256(340282366920938463463367588896596491859n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.eq_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint256) => ebool test 1 (340282366920938463463374144988236257799, 115792089237316195423570985008687907853269984665640564039457582794848891263301)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463374144988236257799n);
    input.add256(115792089237316195423570985008687907853269984665640564039457582794848891263301n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.ne_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint256) => ebool test 2 (340282366920938463463374144988236257795, 340282366920938463463374144988236257799)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463374144988236257795n);
    input.add256(340282366920938463463374144988236257799n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.ne_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint256) => ebool test 3 (340282366920938463463374144988236257799, 340282366920938463463374144988236257799)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463374144988236257799n);
    input.add256(340282366920938463463374144988236257799n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.ne_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint256) => ebool test 4 (340282366920938463463374144988236257799, 340282366920938463463374144988236257795)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463374144988236257799n);
    input.add256(340282366920938463463374144988236257795n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.ne_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint256) => ebool test 1 (340282366920938463463372552646665539131, 115792089237316195423570985008687907853269984665640564039457578321851153049841)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463372552646665539131n);
    input.add256(115792089237316195423570985008687907853269984665640564039457578321851153049841n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.ge_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint128, euint256) => ebool test 2 (340282366920938463463372552646665539127, 340282366920938463463372552646665539131)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463372552646665539127n);
    input.add256(340282366920938463463372552646665539131n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.ge_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint128, euint256) => ebool test 3 (340282366920938463463372552646665539131, 340282366920938463463372552646665539131)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463372552646665539131n);
    input.add256(340282366920938463463372552646665539131n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.ge_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint256) => ebool test 4 (340282366920938463463372552646665539131, 340282366920938463463372552646665539127)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463372552646665539131n);
    input.add256(340282366920938463463372552646665539127n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.ge_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, euint256) => ebool test 1 (340282366920938463463370843114455636527, 115792089237316195423570985008687907853269984665640564039457583283635470770217)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463370843114455636527n);
    input.add256(115792089237316195423570985008687907853269984665640564039457583283635470770217n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.gt_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint256) => ebool test 2 (340282366920938463463370843114455636523, 340282366920938463463370843114455636527)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463370843114455636523n);
    input.add256(340282366920938463463370843114455636527n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.gt_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint256) => ebool test 3 (340282366920938463463370843114455636527, 340282366920938463463370843114455636527)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463370843114455636527n);
    input.add256(340282366920938463463370843114455636527n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.gt_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint256) => ebool test 4 (340282366920938463463370843114455636527, 340282366920938463463370843114455636523)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463370843114455636527n);
    input.add256(340282366920938463463370843114455636523n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.gt_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint256) => ebool test 1 (340282366920938463463370154534182411951, 115792089237316195423570985008687907853269984665640564039457582642838670062225)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463370154534182411951n);
    input.add256(115792089237316195423570985008687907853269984665640564039457582642838670062225n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.le_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint256) => ebool test 2 (340282366920938463463370154534182411947, 340282366920938463463370154534182411951)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463370154534182411947n);
    input.add256(340282366920938463463370154534182411951n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.le_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint256) => ebool test 3 (340282366920938463463370154534182411951, 340282366920938463463370154534182411951)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463370154534182411951n);
    input.add256(340282366920938463463370154534182411951n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.le_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint256) => ebool test 4 (340282366920938463463370154534182411951, 340282366920938463463370154534182411947)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463370154534182411951n);
    input.add256(340282366920938463463370154534182411947n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.le_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint256) => ebool test 1 (340282366920938463463373024758287549459, 115792089237316195423570985008687907853269984665640564039457580686028388504291)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463373024758287549459n);
    input.add256(115792089237316195423570985008687907853269984665640564039457580686028388504291n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.lt_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint128, euint256) => ebool test 2 (340282366920938463463373024758287549455, 340282366920938463463373024758287549459)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463373024758287549455n);
    input.add256(340282366920938463463373024758287549459n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.lt_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint128, euint256) => ebool test 3 (340282366920938463463373024758287549459, 340282366920938463463373024758287549459)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463373024758287549459n);
    input.add256(340282366920938463463373024758287549459n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.lt_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint256) => ebool test 4 (340282366920938463463373024758287549459, 340282366920938463463373024758287549455)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463373024758287549459n);
    input.add256(340282366920938463463373024758287549455n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.lt_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint128, euint256) => euint256 test 1 (340282366920938463463370734691749234713, 115792089237316195423570985008687907853269984665640564039457579316332311519059)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463370734691749234713n);
    input.add256(115792089237316195423570985008687907853269984665640564039457579316332311519059n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.min_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(340282366920938463463370734691749234713n);
  });

  it('test operator "min" overload (euint128, euint256) => euint256 test 2 (340282366920938463463370734691749234709, 340282366920938463463370734691749234713)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463370734691749234709n);
    input.add256(340282366920938463463370734691749234713n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.min_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(340282366920938463463370734691749234709n);
  });

  it('test operator "min" overload (euint128, euint256) => euint256 test 3 (340282366920938463463370734691749234713, 340282366920938463463370734691749234713)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463370734691749234713n);
    input.add256(340282366920938463463370734691749234713n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.min_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(340282366920938463463370734691749234713n);
  });

  it('test operator "min" overload (euint128, euint256) => euint256 test 4 (340282366920938463463370734691749234713, 340282366920938463463370734691749234709)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463370734691749234713n);
    input.add256(340282366920938463463370734691749234709n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.min_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(340282366920938463463370734691749234709n);
  });

  it('test operator "max" overload (euint128, euint256) => euint256 test 1 (340282366920938463463373789765297113341, 115792089237316195423570985008687907853269984665640564039457579894549860743867)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463373789765297113341n);
    input.add256(115792089237316195423570985008687907853269984665640564039457579894549860743867n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.max_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457579894549860743867n);
  });

  it('test operator "max" overload (euint128, euint256) => euint256 test 2 (340282366920938463463373789765297113337, 340282366920938463463373789765297113341)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463373789765297113337n);
    input.add256(340282366920938463463373789765297113341n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.max_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(340282366920938463463373789765297113341n);
  });

  it('test operator "max" overload (euint128, euint256) => euint256 test 3 (340282366920938463463373789765297113341, 340282366920938463463373789765297113341)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463373789765297113341n);
    input.add256(340282366920938463463373789765297113341n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.max_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(340282366920938463463373789765297113341n);
  });

  it('test operator "max" overload (euint128, euint256) => euint256 test 4 (340282366920938463463373789765297113341, 340282366920938463463373789765297113337)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463373789765297113341n);
    input.add256(340282366920938463463373789765297113337n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.max_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(340282366920938463463373789765297113341n);
  });

  it('test operator "add" overload (euint128, uint128) => euint128 test 1 (170141183460469231731685489259224150459, 170141183460469231731685023392215721043)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(170141183460469231731685489259224150459n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.add_euint128_uint128(
      encryptedAmount.handles[0],
      170141183460469231731685023392215721043n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(340282366920938463463370512651439871502n);
  });

  it('test operator "add" overload (euint128, uint128) => euint128 test 2 (170141183460469231731685489259224150457, 170141183460469231731685489259224150459)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(170141183460469231731685489259224150457n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.add_euint128_uint128(
      encryptedAmount.handles[0],
      170141183460469231731685489259224150459n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(340282366920938463463370978518448300916n);
  });

  it('test operator "add" overload (euint128, uint128) => euint128 test 3 (170141183460469231731685489259224150459, 170141183460469231731685489259224150459)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(170141183460469231731685489259224150459n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.add_euint128_uint128(
      encryptedAmount.handles[0],
      170141183460469231731685489259224150459n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(340282366920938463463370978518448300918n);
  });

  it('test operator "add" overload (euint128, uint128) => euint128 test 4 (170141183460469231731685489259224150459, 170141183460469231731685489259224150457)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(170141183460469231731685489259224150459n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.add_euint128_uint128(
      encryptedAmount.handles[0],
      170141183460469231731685489259224150457n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(340282366920938463463370978518448300916n);
  });

  it('test operator "add" overload (uint128, euint128) => euint128 test 1 (170141183460469231731685837916615726593, 170141183460469231731685023392215721043)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);

    input.add128(170141183460469231731685023392215721043n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.add_uint128_euint128(
      170141183460469231731685837916615726593n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(340282366920938463463370861308831447636n);
  });

  it('test operator "add" overload (uint128, euint128) => euint128 test 2 (170141183460469231731685489259224150457, 170141183460469231731685489259224150459)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);

    input.add128(170141183460469231731685489259224150459n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.add_uint128_euint128(
      170141183460469231731685489259224150457n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(340282366920938463463370978518448300916n);
  });

  it('test operator "add" overload (uint128, euint128) => euint128 test 3 (170141183460469231731685489259224150459, 170141183460469231731685489259224150459)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);

    input.add128(170141183460469231731685489259224150459n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.add_uint128_euint128(
      170141183460469231731685489259224150459n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(340282366920938463463370978518448300918n);
  });

  it('test operator "add" overload (uint128, euint128) => euint128 test 4 (170141183460469231731685489259224150459, 170141183460469231731685489259224150457)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);

    input.add128(170141183460469231731685489259224150457n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.add_uint128_euint128(
      170141183460469231731685489259224150459n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(340282366920938463463370978518448300916n);
  });

  it('test operator "sub" overload (euint128, uint128) => euint128 test 1 (340282366920938463463366688248127833841, 340282366920938463463366688248127833841)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463366688248127833841n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.sub_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463366688248127833841n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint128, uint128) => euint128 test 2 (340282366920938463463366688248127833841, 340282366920938463463366688248127833837)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463366688248127833841n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.sub_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463366688248127833837n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(4n);
  });

  it('test operator "sub" overload (uint128, euint128) => euint128 test 1 (340282366920938463463366688248127833841, 340282366920938463463366688248127833841)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);

    input.add128(340282366920938463463366688248127833841n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.sub_uint128_euint128(
      340282366920938463463366688248127833841n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (uint128, euint128) => euint128 test 2 (340282366920938463463366688248127833841, 340282366920938463463366688248127833837)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);

    input.add128(340282366920938463463366688248127833837n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.sub_uint128_euint128(
      340282366920938463463366688248127833841n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint128, uint128) => euint128 test 1 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(9223372036854775809n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.mul_euint128_uint128(
      encryptedAmount.handles[0],
      9223372036854775809n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "mul" overload (euint128, uint128) => euint128 test 2 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(9223372036854775809n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.mul_euint128_uint128(
      encryptedAmount.handles[0],
      9223372036854775809n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "mul" overload (euint128, uint128) => euint128 test 3 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(9223372036854775809n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.mul_euint128_uint128(
      encryptedAmount.handles[0],
      9223372036854775809n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "mul" overload (euint128, uint128) => euint128 test 4 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(9223372036854775809n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.mul_euint128_uint128(
      encryptedAmount.handles[0],
      9223372036854775809n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "mul" overload (uint128, euint128) => euint128 test 1 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);

    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.mul_uint128_euint128(
      9223372036854775809n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "mul" overload (uint128, euint128) => euint128 test 2 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);

    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.mul_uint128_euint128(
      9223372036854775809n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "mul" overload (uint128, euint128) => euint128 test 3 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);

    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.mul_uint128_euint128(
      9223372036854775809n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "mul" overload (uint128, euint128) => euint128 test 4 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);

    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.mul_uint128_euint128(
      9223372036854775809n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "div" overload (euint128, uint128) => euint128 test 1 (340282366920938463463372619792959880891, 340282366920938463463368477899733272407)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463372619792959880891n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.div_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368477899733272407n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint128, uint128) => euint128 test 2 (340282366920938463463372619792959880887, 340282366920938463463372619792959880891)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463372619792959880887n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.div_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463372619792959880891n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint128, uint128) => euint128 test 3 (340282366920938463463372619792959880891, 340282366920938463463372619792959880891)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463372619792959880891n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.div_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463372619792959880891n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint128, uint128) => euint128 test 4 (340282366920938463463372619792959880891, 340282366920938463463372619792959880887)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463372619792959880891n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.div_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463372619792959880887n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(1n);
  });

  it('test operator "rem" overload (euint128, uint128) => euint128 test 1 (340282366920938463463367984231624733459, 340282366920938463463368025855706253793)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463367984231624733459n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.rem_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368025855706253793n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(340282366920938463463367984231624733459n);
  });

  it('test operator "rem" overload (euint128, uint128) => euint128 test 2 (340282366920938463463367984231624733455, 340282366920938463463367984231624733459)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463367984231624733455n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.rem_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463367984231624733459n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(340282366920938463463367984231624733455n);
  });

  it('test operator "rem" overload (euint128, uint128) => euint128 test 3 (340282366920938463463367984231624733459, 340282366920938463463367984231624733459)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463367984231624733459n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.rem_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463367984231624733459n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "rem" overload (euint128, uint128) => euint128 test 4 (340282366920938463463367984231624733459, 340282366920938463463367984231624733455)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463367984231624733459n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.rem_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463367984231624733455n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(4n);
  });

  it('test operator "and" overload (euint128, uint128) => euint128 test 1 (340282366920938463463365754576278416105, 340282366920938463463366410191459861233)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463365754576278416105n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.and_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463366410191459861233n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(340282366920938463463365741001161851617n);
  });

  it('test operator "and" overload (euint128, uint128) => euint128 test 2 (340282366920938463463365754576278416101, 340282366920938463463365754576278416105)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463365754576278416101n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.and_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463365754576278416105n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(340282366920938463463365754576278416097n);
  });

  it('test operator "and" overload (euint128, uint128) => euint128 test 3 (340282366920938463463365754576278416105, 340282366920938463463365754576278416105)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463365754576278416105n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.and_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463365754576278416105n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(340282366920938463463365754576278416105n);
  });

  it('test operator "and" overload (euint128, uint128) => euint128 test 4 (340282366920938463463365754576278416105, 340282366920938463463365754576278416101)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463365754576278416105n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.and_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463365754576278416101n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(340282366920938463463365754576278416097n);
  });

  it('test operator "and" overload (uint128, euint128) => euint128 test 1 (340282366920938463463371836375477224759, 340282366920938463463366410191459861233)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);

    input.add128(340282366920938463463366410191459861233n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.and_uint128_euint128(
      340282366920938463463371836375477224759n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(340282366920938463463366199075542663217n);
  });

  it('test operator "and" overload (uint128, euint128) => euint128 test 2 (340282366920938463463365754576278416101, 340282366920938463463365754576278416105)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);

    input.add128(340282366920938463463365754576278416105n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.and_uint128_euint128(
      340282366920938463463365754576278416101n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(340282366920938463463365754576278416097n);
  });

  it('test operator "and" overload (uint128, euint128) => euint128 test 3 (340282366920938463463365754576278416105, 340282366920938463463365754576278416105)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);

    input.add128(340282366920938463463365754576278416105n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.and_uint128_euint128(
      340282366920938463463365754576278416105n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(340282366920938463463365754576278416105n);
  });

  it('test operator "and" overload (uint128, euint128) => euint128 test 4 (340282366920938463463365754576278416105, 340282366920938463463365754576278416101)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);

    input.add128(340282366920938463463365754576278416101n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.and_uint128_euint128(
      340282366920938463463365754576278416105n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(340282366920938463463365754576278416097n);
  });

  it('test operator "or" overload (euint128, uint128) => euint128 test 1 (340282366920938463463367655063545859643, 340282366920938463463373853783183776929)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463367655063545859643n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.or_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463373853783183776929n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(340282366920938463463374430271410986683n);
  });

  it('test operator "or" overload (euint128, uint128) => euint128 test 2 (340282366920938463463367655063545859639, 340282366920938463463367655063545859643)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463367655063545859639n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.or_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463367655063545859643n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(340282366920938463463367655063545859647n);
  });

  it('test operator "or" overload (euint128, uint128) => euint128 test 3 (340282366920938463463367655063545859643, 340282366920938463463367655063545859643)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463367655063545859643n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.or_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463367655063545859643n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(340282366920938463463367655063545859643n);
  });

  it('test operator "or" overload (euint128, uint128) => euint128 test 4 (340282366920938463463367655063545859643, 340282366920938463463367655063545859639)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463367655063545859643n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.or_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463367655063545859639n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(340282366920938463463367655063545859647n);
  });

  it('test operator "or" overload (uint128, euint128) => euint128 test 1 (340282366920938463463370547981929977503, 340282366920938463463373853783183776929)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);

    input.add128(340282366920938463463373853783183776929n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.or_uint128_euint128(
      340282366920938463463370547981929977503n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(340282366920938463463373996101960916671n);
  });

  it('test operator "or" overload (uint128, euint128) => euint128 test 2 (340282366920938463463367655063545859639, 340282366920938463463367655063545859643)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);

    input.add128(340282366920938463463367655063545859643n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.or_uint128_euint128(
      340282366920938463463367655063545859639n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(340282366920938463463367655063545859647n);
  });

  it('test operator "or" overload (uint128, euint128) => euint128 test 3 (340282366920938463463367655063545859643, 340282366920938463463367655063545859643)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);

    input.add128(340282366920938463463367655063545859643n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.or_uint128_euint128(
      340282366920938463463367655063545859643n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(340282366920938463463367655063545859643n);
  });

  it('test operator "or" overload (uint128, euint128) => euint128 test 4 (340282366920938463463367655063545859643, 340282366920938463463367655063545859639)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);

    input.add128(340282366920938463463367655063545859639n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.or_uint128_euint128(
      340282366920938463463367655063545859643n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(340282366920938463463367655063545859647n);
  });

  it('test operator "xor" overload (euint128, uint128) => euint128 test 1 (340282366920938463463372080691181989981, 340282366920938463463368386031692563719)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463372080691181989981n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.xor_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368386031692563719n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(8690927368893786n);
  });

  it('test operator "xor" overload (euint128, uint128) => euint128 test 2 (340282366920938463463368941059346865325, 340282366920938463463368941059346865329)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463368941059346865325n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.xor_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368941059346865329n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(28n);
  });

  it('test operator "xor" overload (euint128, uint128) => euint128 test 3 (340282366920938463463368941059346865329, 340282366920938463463368941059346865329)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463368941059346865329n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.xor_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368941059346865329n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint128, uint128) => euint128 test 4 (340282366920938463463368941059346865329, 340282366920938463463368941059346865325)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463368941059346865329n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.xor_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368941059346865325n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(28n);
  });

  it('test operator "xor" overload (uint128, euint128) => euint128 test 1 (340282366920938463463374269120313762865, 340282366920938463463368386031692563719)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);

    input.add128(340282366920938463463368386031692563719n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.xor_uint128_euint128(
      340282366920938463463374269120313762865n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(6519991538245942n);
  });

  it('test operator "xor" overload (uint128, euint128) => euint128 test 2 (340282366920938463463368941059346865325, 340282366920938463463368941059346865329)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);

    input.add128(340282366920938463463368941059346865329n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.xor_uint128_euint128(
      340282366920938463463368941059346865325n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(28n);
  });

  it('test operator "xor" overload (uint128, euint128) => euint128 test 3 (340282366920938463463368941059346865329, 340282366920938463463368941059346865329)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);

    input.add128(340282366920938463463368941059346865329n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.xor_uint128_euint128(
      340282366920938463463368941059346865329n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (uint128, euint128) => euint128 test 4 (340282366920938463463368941059346865329, 340282366920938463463368941059346865325)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);

    input.add128(340282366920938463463368941059346865325n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.xor_uint128_euint128(
      340282366920938463463368941059346865329n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(28n);
  });

  it('test operator "eq" overload (euint128, uint128) => ebool test 1 (340282366920938463463371368021216956093, 340282366920938463463373345909663104587)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463371368021216956093n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.eq_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463373345909663104587n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, uint128) => ebool test 2 (340282366920938463463370243874611927701, 340282366920938463463370243874611927705)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463370243874611927701n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.eq_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463370243874611927705n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, uint128) => ebool test 3 (340282366920938463463370243874611927705, 340282366920938463463370243874611927705)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463370243874611927705n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.eq_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463370243874611927705n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint128, uint128) => ebool test 4 (340282366920938463463370243874611927705, 340282366920938463463370243874611927701)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463370243874611927705n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.eq_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463370243874611927701n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint128, euint128) => ebool test 1 (340282366920938463463369712474902223435, 340282366920938463463373345909663104587)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);

    input.add128(340282366920938463463373345909663104587n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.eq_uint128_euint128(
      340282366920938463463369712474902223435n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint128, euint128) => ebool test 2 (340282366920938463463370243874611927701, 340282366920938463463370243874611927705)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);

    input.add128(340282366920938463463370243874611927705n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.eq_uint128_euint128(
      340282366920938463463370243874611927701n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint128, euint128) => ebool test 3 (340282366920938463463370243874611927705, 340282366920938463463370243874611927705)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);

    input.add128(340282366920938463463370243874611927705n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.eq_uint128_euint128(
      340282366920938463463370243874611927705n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint128, euint128) => ebool test 4 (340282366920938463463370243874611927705, 340282366920938463463370243874611927701)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);

    input.add128(340282366920938463463370243874611927701n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.eq_uint128_euint128(
      340282366920938463463370243874611927705n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, uint128) => ebool test 1 (340282366920938463463366690564024700303, 340282366920938463463373212539976330723)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463366690564024700303n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.ne_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463373212539976330723n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, uint128) => ebool test 2 (340282366920938463463366105643215363061, 340282366920938463463366105643215363065)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463366105643215363061n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.ne_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463366105643215363065n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, uint128) => ebool test 3 (340282366920938463463366105643215363065, 340282366920938463463366105643215363065)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463366105643215363065n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.ne_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463366105643215363065n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, uint128) => ebool test 4 (340282366920938463463366105643215363065, 340282366920938463463366105643215363061)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463366105643215363065n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.ne_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463366105643215363061n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint128, euint128) => ebool test 1 (340282366920938463463372802578174510491, 340282366920938463463373212539976330723)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);

    input.add128(340282366920938463463373212539976330723n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.ne_uint128_euint128(
      340282366920938463463372802578174510491n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint128, euint128) => ebool test 2 (340282366920938463463366105643215363061, 340282366920938463463366105643215363065)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);

    input.add128(340282366920938463463366105643215363065n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.ne_uint128_euint128(
      340282366920938463463366105643215363061n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint128, euint128) => ebool test 3 (340282366920938463463366105643215363065, 340282366920938463463366105643215363065)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);

    input.add128(340282366920938463463366105643215363065n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.ne_uint128_euint128(
      340282366920938463463366105643215363065n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint128, euint128) => ebool test 4 (340282366920938463463366105643215363065, 340282366920938463463366105643215363061)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);

    input.add128(340282366920938463463366105643215363061n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.ne_uint128_euint128(
      340282366920938463463366105643215363065n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, uint128) => ebool test 1 (340282366920938463463369991167430665173, 340282366920938463463370096368753149585)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463369991167430665173n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.ge_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463370096368753149585n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint128, uint128) => ebool test 2 (340282366920938463463367243418798341495, 340282366920938463463367243418798341499)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463367243418798341495n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.ge_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463367243418798341499n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint128, uint128) => ebool test 3 (340282366920938463463367243418798341499, 340282366920938463463367243418798341499)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463367243418798341499n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.ge_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463367243418798341499n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, uint128) => ebool test 4 (340282366920938463463367243418798341499, 340282366920938463463367243418798341495)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463367243418798341499n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.ge_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463367243418798341495n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint128, euint128) => ebool test 1 (340282366920938463463370285053807900009, 340282366920938463463370096368753149585)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);

    input.add128(340282366920938463463370096368753149585n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.ge_uint128_euint128(
      340282366920938463463370285053807900009n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint128, euint128) => ebool test 2 (340282366920938463463367243418798341495, 340282366920938463463367243418798341499)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);

    input.add128(340282366920938463463367243418798341499n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.ge_uint128_euint128(
      340282366920938463463367243418798341495n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint128, euint128) => ebool test 3 (340282366920938463463367243418798341499, 340282366920938463463367243418798341499)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);

    input.add128(340282366920938463463367243418798341499n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.ge_uint128_euint128(
      340282366920938463463367243418798341499n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint128, euint128) => ebool test 4 (340282366920938463463367243418798341499, 340282366920938463463367243418798341495)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);

    input.add128(340282366920938463463367243418798341495n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.ge_uint128_euint128(
      340282366920938463463367243418798341499n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, uint128) => ebool test 1 (340282366920938463463367714696589499231, 340282366920938463463369886774124197135)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463367714696589499231n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.gt_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463369886774124197135n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, uint128) => ebool test 2 (340282366920938463463367714696589499227, 340282366920938463463367714696589499231)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463367714696589499227n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.gt_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463367714696589499231n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, uint128) => ebool test 3 (340282366920938463463367714696589499231, 340282366920938463463367714696589499231)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463367714696589499231n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.gt_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463367714696589499231n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, uint128) => ebool test 4 (340282366920938463463367714696589499231, 340282366920938463463367714696589499227)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463367714696589499231n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.gt_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463367714696589499227n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint128, euint128) => ebool test 1 (340282366920938463463371365285946007453, 340282366920938463463369886774124197135)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);

    input.add128(340282366920938463463369886774124197135n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.gt_uint128_euint128(
      340282366920938463463371365285946007453n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint128, euint128) => ebool test 2 (340282366920938463463367714696589499227, 340282366920938463463367714696589499231)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);

    input.add128(340282366920938463463367714696589499231n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.gt_uint128_euint128(
      340282366920938463463367714696589499227n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint128, euint128) => ebool test 3 (340282366920938463463367714696589499231, 340282366920938463463367714696589499231)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);

    input.add128(340282366920938463463367714696589499231n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.gt_uint128_euint128(
      340282366920938463463367714696589499231n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint128, euint128) => ebool test 4 (340282366920938463463367714696589499231, 340282366920938463463367714696589499227)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);

    input.add128(340282366920938463463367714696589499227n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.gt_uint128_euint128(
      340282366920938463463367714696589499231n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, uint128) => ebool test 1 (340282366920938463463373992528465349581, 340282366920938463463373835012809081201)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463373992528465349581n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.le_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463373835012809081201n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint128, uint128) => ebool test 2 (340282366920938463463372840104721436795, 340282366920938463463372840104721436799)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463372840104721436795n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.le_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463372840104721436799n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, uint128) => ebool test 3 (340282366920938463463372840104721436799, 340282366920938463463372840104721436799)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463372840104721436799n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.le_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463372840104721436799n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, uint128) => ebool test 4 (340282366920938463463372840104721436799, 340282366920938463463372840104721436795)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463372840104721436799n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.le_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463372840104721436795n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint128, euint128) => ebool test 1 (340282366920938463463374153561991926979, 340282366920938463463373835012809081201)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);

    input.add128(340282366920938463463373835012809081201n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.le_uint128_euint128(
      340282366920938463463374153561991926979n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint128, euint128) => ebool test 2 (340282366920938463463372840104721436795, 340282366920938463463372840104721436799)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);

    input.add128(340282366920938463463372840104721436799n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.le_uint128_euint128(
      340282366920938463463372840104721436795n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint128, euint128) => ebool test 3 (340282366920938463463372840104721436799, 340282366920938463463372840104721436799)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);

    input.add128(340282366920938463463372840104721436799n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.le_uint128_euint128(
      340282366920938463463372840104721436799n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint128, euint128) => ebool test 4 (340282366920938463463372840104721436799, 340282366920938463463372840104721436795)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);

    input.add128(340282366920938463463372840104721436795n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.le_uint128_euint128(
      340282366920938463463372840104721436799n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, uint128) => ebool test 1 (340282366920938463463368309276517872429, 340282366920938463463366305725915183957)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463368309276517872429n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.lt_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463366305725915183957n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, uint128) => ebool test 2 (340282366920938463463368309276517872425, 340282366920938463463368309276517872429)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463368309276517872425n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.lt_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368309276517872429n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint128, uint128) => ebool test 3 (340282366920938463463368309276517872429, 340282366920938463463368309276517872429)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463368309276517872429n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.lt_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368309276517872429n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, uint128) => ebool test 4 (340282366920938463463368309276517872429, 340282366920938463463368309276517872425)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463368309276517872429n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.lt_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368309276517872425n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint128, euint128) => ebool test 1 (340282366920938463463371881592186712743, 340282366920938463463366305725915183957)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);

    input.add128(340282366920938463463366305725915183957n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.lt_uint128_euint128(
      340282366920938463463371881592186712743n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint128, euint128) => ebool test 2 (340282366920938463463368309276517872425, 340282366920938463463368309276517872429)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);

    input.add128(340282366920938463463368309276517872429n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.lt_uint128_euint128(
      340282366920938463463368309276517872425n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint128, euint128) => ebool test 3 (340282366920938463463368309276517872429, 340282366920938463463368309276517872429)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);

    input.add128(340282366920938463463368309276517872429n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.lt_uint128_euint128(
      340282366920938463463368309276517872429n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint128, euint128) => ebool test 4 (340282366920938463463368309276517872429, 340282366920938463463368309276517872425)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);

    input.add128(340282366920938463463368309276517872425n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.lt_uint128_euint128(
      340282366920938463463368309276517872429n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint128, uint128) => euint128 test 1 (340282366920938463463369507025632955721, 340282366920938463463369479362880511843)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463369507025632955721n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.min_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463369479362880511843n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(340282366920938463463369479362880511843n);
  });

  it('test operator "min" overload (euint128, uint128) => euint128 test 2 (340282366920938463463369507025632955717, 340282366920938463463369507025632955721)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463369507025632955717n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.min_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463369507025632955721n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(340282366920938463463369507025632955717n);
  });

  it('test operator "min" overload (euint128, uint128) => euint128 test 3 (340282366920938463463369507025632955721, 340282366920938463463369507025632955721)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463369507025632955721n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.min_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463369507025632955721n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(340282366920938463463369507025632955721n);
  });

  it('test operator "min" overload (euint128, uint128) => euint128 test 4 (340282366920938463463369507025632955721, 340282366920938463463369507025632955717)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463369507025632955721n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.min_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463369507025632955717n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(340282366920938463463369507025632955717n);
  });

  it('test operator "min" overload (uint128, euint128) => euint128 test 1 (340282366920938463463367460119738968177, 340282366920938463463369479362880511843)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);

    input.add128(340282366920938463463369479362880511843n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.min_uint128_euint128(
      340282366920938463463367460119738968177n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(340282366920938463463367460119738968177n);
  });

  it('test operator "min" overload (uint128, euint128) => euint128 test 2 (340282366920938463463369507025632955717, 340282366920938463463369507025632955721)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);

    input.add128(340282366920938463463369507025632955721n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.min_uint128_euint128(
      340282366920938463463369507025632955717n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(340282366920938463463369507025632955717n);
  });

  it('test operator "min" overload (uint128, euint128) => euint128 test 3 (340282366920938463463369507025632955721, 340282366920938463463369507025632955721)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);

    input.add128(340282366920938463463369507025632955721n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.min_uint128_euint128(
      340282366920938463463369507025632955721n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(340282366920938463463369507025632955721n);
  });

  it('test operator "min" overload (uint128, euint128) => euint128 test 4 (340282366920938463463369507025632955721, 340282366920938463463369507025632955717)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);

    input.add128(340282366920938463463369507025632955717n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.min_uint128_euint128(
      340282366920938463463369507025632955721n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(340282366920938463463369507025632955717n);
  });

  it('test operator "max" overload (euint128, uint128) => euint128 test 1 (340282366920938463463374311959896511759, 340282366920938463463371397781663603615)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463374311959896511759n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.max_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463371397781663603615n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(340282366920938463463374311959896511759n);
  });

  it('test operator "max" overload (euint128, uint128) => euint128 test 2 (340282366920938463463367499312160836027, 340282366920938463463367499312160836031)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463367499312160836027n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.max_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463367499312160836031n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(340282366920938463463367499312160836031n);
  });

  it('test operator "max" overload (euint128, uint128) => euint128 test 3 (340282366920938463463367499312160836031, 340282366920938463463367499312160836031)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463367499312160836031n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.max_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463367499312160836031n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(340282366920938463463367499312160836031n);
  });

  it('test operator "max" overload (euint128, uint128) => euint128 test 4 (340282366920938463463367499312160836031, 340282366920938463463367499312160836027)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463367499312160836031n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.max_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463367499312160836027n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(340282366920938463463367499312160836031n);
  });

  it('test operator "max" overload (uint128, euint128) => euint128 test 1 (340282366920938463463370589299931927619, 340282366920938463463371397781663603615)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);

    input.add128(340282366920938463463371397781663603615n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.max_uint128_euint128(
      340282366920938463463370589299931927619n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(340282366920938463463371397781663603615n);
  });

  it('test operator "max" overload (uint128, euint128) => euint128 test 2 (340282366920938463463367499312160836027, 340282366920938463463367499312160836031)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);

    input.add128(340282366920938463463367499312160836031n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.max_uint128_euint128(
      340282366920938463463367499312160836027n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(340282366920938463463367499312160836031n);
  });

  it('test operator "max" overload (uint128, euint128) => euint128 test 3 (340282366920938463463367499312160836031, 340282366920938463463367499312160836031)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);

    input.add128(340282366920938463463367499312160836031n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.max_uint128_euint128(
      340282366920938463463367499312160836031n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(340282366920938463463367499312160836031n);
  });

  it('test operator "max" overload (uint128, euint128) => euint128 test 4 (340282366920938463463367499312160836031, 340282366920938463463367499312160836027)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);

    input.add128(340282366920938463463367499312160836027n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.max_uint128_euint128(
      340282366920938463463367499312160836031n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(340282366920938463463367499312160836031n);
  });

  it('test operator "add" overload (euint256, euint4) => euint256 test 1 (9, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(9n);
    input.add4(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.add_euint256_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(11n);
  });

  it('test operator "add" overload (euint256, euint4) => euint256 test 2 (6, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(6n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.add_euint256_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(14n);
  });

  it('test operator "add" overload (euint256, euint4) => euint256 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(5n);
    input.add4(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.add_euint256_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint256, euint4) => euint256 test 4 (8, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(8n);
    input.add4(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.add_euint256_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(14n);
  });

  it('test operator "sub" overload (euint256, euint4) => euint256 test 1 (11, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(11n);
    input.add4(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.sub_euint256_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint256, euint4) => euint256 test 2 (11, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(11n);
    input.add4(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.sub_euint256_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint256, euint4) => euint256 test 1 (5, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(5n);
    input.add4(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.mul_euint256_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(10n);
  });

  it('test operator "mul" overload (euint256, euint4) => euint256 test 2 (3, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(3n);
    input.add4(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.mul_euint256_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(15n);
  });

  it('test operator "mul" overload (euint256, euint4) => euint256 test 3 (3, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(3n);
    input.add4(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.mul_euint256_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint256, euint4) => euint256 test 4 (5, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(5n);
    input.add4(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.mul_euint256_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(15n);
  });

  it('test operator "and" overload (euint256, euint4) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457579364828919798977, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579364828919798977n);
    input.add4(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.and_euint256_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(1n);
  });

  it('test operator "and" overload (euint256, euint4) => euint256 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.and_euint256_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint256, euint4) => euint256 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.and_euint256_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint256, euint4) => euint256 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.and_euint256_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "or" overload (euint256, euint4) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457579810763114124559, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579810763114124559n);
    input.add4(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.or_euint256_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457579810763114124559n);
  });

  it('test operator "or" overload (euint256, euint4) => euint256 test 2 (10, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(10n);
    input.add4(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.or_euint256_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(14n);
  });

  it('test operator "or" overload (euint256, euint4) => euint256 test 3 (14, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(14n);
    input.add4(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.or_euint256_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(14n);
  });

  it('test operator "or" overload (euint256, euint4) => euint256 test 4 (14, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(14n);
    input.add4(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.or_euint256_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(14n);
  });

  it('test operator "xor" overload (euint256, euint4) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457582453903191902895, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582453903191902895n);
    input.add4(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.xor_euint256_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457582453903191902883n);
  });

  it('test operator "xor" overload (euint256, euint4) => euint256 test 2 (8, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(8n);
    input.add4(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.xor_euint256_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint256, euint4) => euint256 test 3 (12, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(12n);
    input.add4(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.xor_euint256_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint256, euint4) => euint256 test 4 (12, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(12n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.xor_euint256_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint256, euint4) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457581942293222963611, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457581942293222963611n);
    input.add4(1n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.eq_euint256_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, euint4) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.eq_euint256_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, euint4) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.eq_euint256_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint256, euint4) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.eq_euint256_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, euint4) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457577394215847356385, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577394215847356385n);
    input.add4(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.ne_euint256_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, euint4) => ebool test 2 (8, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(8n);
    input.add4(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.ne_euint256_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, euint4) => ebool test 3 (12, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(12n);
    input.add4(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.ne_euint256_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, euint4) => ebool test 4 (12, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(12n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.ne_euint256_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint256, euint4) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457575496173341010993, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575496173341010993n);
    input.add4(1n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.ge_euint256_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint256, euint4) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.ge_euint256_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint256, euint4) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.ge_euint256_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint256, euint4) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.ge_euint256_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint256, euint4) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457576018520605547063, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576018520605547063n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.gt_euint256_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint256, euint4) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.gt_euint256_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint256, euint4) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.gt_euint256_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint256, euint4) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.gt_euint256_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint256, euint4) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457578089704885195999, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578089704885195999n);
    input.add4(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.le_euint256_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint256, euint4) => ebool test 2 (7, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(7n);
    input.add4(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.le_euint256_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint256, euint4) => ebool test 3 (11, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(11n);
    input.add4(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.le_euint256_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint256, euint4) => ebool test 4 (11, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(11n);
    input.add4(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.le_euint256_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint256, euint4) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457580046200927487349, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580046200927487349n);
    input.add4(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.lt_euint256_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint256, euint4) => ebool test 2 (5, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(5n);
    input.add4(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.lt_euint256_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint256, euint4) => ebool test 3 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(9n);
    input.add4(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.lt_euint256_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint256, euint4) => ebool test 4 (9, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(9n);
    input.add4(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.lt_euint256_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint256, euint4) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457582664799758938101, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582664799758938101n);
    input.add4(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.min_euint256_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(2n);
  });

  it('test operator "min" overload (euint256, euint4) => euint256 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.min_euint256_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(4n);
  });

  it('test operator "min" overload (euint256, euint4) => euint256 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.min_euint256_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (euint256, euint4) => euint256 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.min_euint256_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(4n);
  });

  it('test operator "max" overload (euint256, euint4) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457577303760309863037, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577303760309863037n);
    input.add4(1n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.max_euint256_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457577303760309863037n);
  });

  it('test operator "max" overload (euint256, euint4) => euint256 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.max_euint256_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint256, euint4) => euint256 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.max_euint256_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint256, euint4) => euint256 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.max_euint256_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint256, euint8) => euint256 test 1 (129, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(129n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.add_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(131n);
  });

  it('test operator "add" overload (euint256, euint8) => euint256 test 2 (101, 103)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(101n);
    input.add8(103n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.add_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(204n);
  });

  it('test operator "add" overload (euint256, euint8) => euint256 test 3 (103, 103)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(103n);
    input.add8(103n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.add_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(206n);
  });

  it('test operator "add" overload (euint256, euint8) => euint256 test 4 (103, 101)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(103n);
    input.add8(101n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.add_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(204n);
  });

  it('test operator "sub" overload (euint256, euint8) => euint256 test 1 (234, 234)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(234n);
    input.add8(234n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.sub_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint256, euint8) => euint256 test 2 (234, 230)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(234n);
    input.add8(230n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.sub_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint256, euint8) => euint256 test 1 (65, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(65n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.mul_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(130n);
  });

  it('test operator "mul" overload (euint256, euint8) => euint256 test 2 (13, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(13n);
    input.add8(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.mul_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(182n);
  });

  it('test operator "mul" overload (euint256, euint8) => euint256 test 3 (14, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(14n);
    input.add8(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.mul_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(196n);
  });

  it('test operator "mul" overload (euint256, euint8) => euint256 test 4 (14, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(14n);
    input.add8(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.mul_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(182n);
  });

  it('test operator "and" overload (euint256, euint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457580200311934752389, 160)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580200311934752389n);
    input.add8(160n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.and_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(128n);
  });

  it('test operator "and" overload (euint256, euint8) => euint256 test 2 (156, 160)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(156n);
    input.add8(160n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.and_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(128n);
  });

  it('test operator "and" overload (euint256, euint8) => euint256 test 3 (160, 160)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(160n);
    input.add8(160n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.and_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(160n);
  });

  it('test operator "and" overload (euint256, euint8) => euint256 test 4 (160, 156)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(160n);
    input.add8(156n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.and_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(128n);
  });

  it('test operator "or" overload (euint256, euint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457579600697842114421, 25)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579600697842114421n);
    input.add8(25n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.or_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457579600697842114429n);
  });

  it('test operator "or" overload (euint256, euint8) => euint256 test 2 (21, 25)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(21n);
    input.add8(25n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.or_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(29n);
  });

  it('test operator "or" overload (euint256, euint8) => euint256 test 3 (25, 25)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(25n);
    input.add8(25n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.or_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(25n);
  });

  it('test operator "or" overload (euint256, euint8) => euint256 test 4 (25, 21)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(25n);
    input.add8(21n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.or_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(29n);
  });

  it('test operator "xor" overload (euint256, euint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457576224347045805665, 175)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576224347045805665n);
    input.add8(175n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.xor_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457576224347045805774n);
  });

  it('test operator "xor" overload (euint256, euint8) => euint256 test 2 (171, 175)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(171n);
    input.add8(175n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.xor_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint256, euint8) => euint256 test 3 (175, 175)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(175n);
    input.add8(175n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.xor_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint256, euint8) => euint256 test 4 (175, 171)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(175n);
    input.add8(171n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.xor_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint256, euint8) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457581724603808654415, 213)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457581724603808654415n);
    input.add8(213n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.eq_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, euint8) => ebool test 2 (209, 213)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(209n);
    input.add8(213n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.eq_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, euint8) => ebool test 3 (213, 213)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(213n);
    input.add8(213n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.eq_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint256, euint8) => ebool test 4 (213, 209)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(213n);
    input.add8(209n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.eq_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, euint8) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457577306800788645475, 79)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577306800788645475n);
    input.add8(79n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.ne_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, euint8) => ebool test 2 (75, 79)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(75n);
    input.add8(79n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.ne_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, euint8) => ebool test 3 (79, 79)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(79n);
    input.add8(79n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.ne_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, euint8) => ebool test 4 (79, 75)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(79n);
    input.add8(75n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.ne_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint256, euint8) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457583777768346925767, 250)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457583777768346925767n);
    input.add8(250n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.ge_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint256, euint8) => ebool test 2 (246, 250)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(246n);
    input.add8(250n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.ge_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint256, euint8) => ebool test 3 (250, 250)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(250n);
    input.add8(250n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.ge_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint256, euint8) => ebool test 4 (250, 246)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(250n);
    input.add8(246n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.ge_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint256, euint8) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457583855482126679265, 93)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457583855482126679265n);
    input.add8(93n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.gt_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint256, euint8) => ebool test 2 (89, 93)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(89n);
    input.add8(93n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.gt_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint256, euint8) => ebool test 3 (93, 93)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(93n);
    input.add8(93n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.gt_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint256, euint8) => ebool test 4 (93, 89)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(93n);
    input.add8(89n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.gt_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint256, euint8) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457580743279895693245, 161)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580743279895693245n);
    input.add8(161n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.le_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint256, euint8) => ebool test 2 (157, 161)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(157n);
    input.add8(161n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.le_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint256, euint8) => ebool test 3 (161, 161)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(161n);
    input.add8(161n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.le_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint256, euint8) => ebool test 4 (161, 157)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(161n);
    input.add8(157n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.le_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint256, euint8) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457575356850463874977, 220)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575356850463874977n);
    input.add8(220n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.lt_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint256, euint8) => ebool test 2 (216, 220)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(216n);
    input.add8(220n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.lt_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint256, euint8) => ebool test 3 (220, 220)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(220n);
    input.add8(220n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.lt_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint256, euint8) => ebool test 4 (220, 216)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(220n);
    input.add8(216n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.lt_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint256, euint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457577026640283682165, 103)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577026640283682165n);
    input.add8(103n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.min_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(103n);
  });

  it('test operator "min" overload (euint256, euint8) => euint256 test 2 (99, 103)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(99n);
    input.add8(103n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.min_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(99n);
  });

  it('test operator "min" overload (euint256, euint8) => euint256 test 3 (103, 103)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(103n);
    input.add8(103n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.min_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(103n);
  });

  it('test operator "min" overload (euint256, euint8) => euint256 test 4 (103, 99)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(103n);
    input.add8(99n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.min_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(99n);
  });

  it('test operator "max" overload (euint256, euint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457581923844171729579, 237)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457581923844171729579n);
    input.add8(237n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.max_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457581923844171729579n);
  });

  it('test operator "max" overload (euint256, euint8) => euint256 test 2 (233, 237)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(233n);
    input.add8(237n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.max_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(237n);
  });

  it('test operator "max" overload (euint256, euint8) => euint256 test 3 (237, 237)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(237n);
    input.add8(237n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.max_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(237n);
  });

  it('test operator "max" overload (euint256, euint8) => euint256 test 4 (237, 233)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(237n);
    input.add8(233n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.max_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(237n);
  });

  it('test operator "add" overload (euint256, euint16) => euint256 test 1 (32769, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(32769n);
    input.add16(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.add_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(32771n);
  });

  it('test operator "add" overload (euint256, euint16) => euint256 test 2 (18590, 18592)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(18590n);
    input.add16(18592n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.add_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(37182n);
  });

  it('test operator "add" overload (euint256, euint16) => euint256 test 3 (18592, 18592)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(18592n);
    input.add16(18592n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.add_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(37184n);
  });

  it('test operator "add" overload (euint256, euint16) => euint256 test 4 (18592, 18590)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(18592n);
    input.add16(18590n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.add_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(37182n);
  });

  it('test operator "sub" overload (euint256, euint16) => euint256 test 1 (46280, 46280)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(46280n);
    input.add16(46280n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.sub_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint256, euint16) => euint256 test 2 (46280, 46276)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(46280n);
    input.add16(46276n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.sub_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint256, euint16) => euint256 test 1 (16385, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(16385n);
    input.add16(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.mul_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(32770n);
  });

  it('test operator "mul" overload (euint256, euint16) => euint256 test 2 (233, 233)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(233n);
    input.add16(233n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.mul_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(54289n);
  });

  it('test operator "mul" overload (euint256, euint16) => euint256 test 3 (233, 233)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(233n);
    input.add16(233n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.mul_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(54289n);
  });

  it('test operator "mul" overload (euint256, euint16) => euint256 test 4 (233, 233)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(233n);
    input.add16(233n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.mul_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(54289n);
  });

  it('test operator "and" overload (euint256, euint16) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457581238581042328907, 21842)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457581238581042328907n);
    input.add16(21842n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.and_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(5442n);
  });

  it('test operator "and" overload (euint256, euint16) => euint256 test 2 (21838, 21842)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(21838n);
    input.add16(21842n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.and_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(21826n);
  });

  it('test operator "and" overload (euint256, euint16) => euint256 test 3 (21842, 21842)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(21842n);
    input.add16(21842n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.and_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(21842n);
  });

  it('test operator "and" overload (euint256, euint16) => euint256 test 4 (21842, 21838)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(21842n);
    input.add16(21838n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.and_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(21826n);
  });
});
