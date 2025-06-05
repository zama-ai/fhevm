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

  it('test operator "ge" overload (euint128, euint128) => ebool test 1 (340282366920938463463369962972097513525, 340282366920938463463366074307494086589)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463369962972097513525n);
    input.add128(340282366920938463463366074307494086589n);
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

  it('test operator "ge" overload (euint128, euint128) => ebool test 2 (340282366920938463463366074307494086585, 340282366920938463463366074307494086589)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463366074307494086585n);
    input.add128(340282366920938463463366074307494086589n);
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

  it('test operator "ge" overload (euint128, euint128) => ebool test 3 (340282366920938463463366074307494086589, 340282366920938463463366074307494086589)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463366074307494086589n);
    input.add128(340282366920938463463366074307494086589n);
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

  it('test operator "ge" overload (euint128, euint128) => ebool test 4 (340282366920938463463366074307494086589, 340282366920938463463366074307494086585)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463366074307494086589n);
    input.add128(340282366920938463463366074307494086585n);
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

  it('test operator "gt" overload (euint128, euint128) => ebool test 1 (340282366920938463463369851219538864853, 340282366920938463463368291735971449829)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463369851219538864853n);
    input.add128(340282366920938463463368291735971449829n);
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

  it('test operator "gt" overload (euint128, euint128) => ebool test 2 (340282366920938463463368291735971449825, 340282366920938463463368291735971449829)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463368291735971449825n);
    input.add128(340282366920938463463368291735971449829n);
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

  it('test operator "gt" overload (euint128, euint128) => ebool test 3 (340282366920938463463368291735971449829, 340282366920938463463368291735971449829)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463368291735971449829n);
    input.add128(340282366920938463463368291735971449829n);
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

  it('test operator "gt" overload (euint128, euint128) => ebool test 4 (340282366920938463463368291735971449829, 340282366920938463463368291735971449825)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463368291735971449829n);
    input.add128(340282366920938463463368291735971449825n);
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

  it('test operator "le" overload (euint128, euint128) => ebool test 1 (340282366920938463463367536769307757855, 340282366920938463463369435843482462693)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463367536769307757855n);
    input.add128(340282366920938463463369435843482462693n);
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

  it('test operator "le" overload (euint128, euint128) => ebool test 2 (340282366920938463463367536769307757851, 340282366920938463463367536769307757855)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463367536769307757851n);
    input.add128(340282366920938463463367536769307757855n);
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

  it('test operator "le" overload (euint128, euint128) => ebool test 3 (340282366920938463463367536769307757855, 340282366920938463463367536769307757855)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463367536769307757855n);
    input.add128(340282366920938463463367536769307757855n);
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

  it('test operator "le" overload (euint128, euint128) => ebool test 4 (340282366920938463463367536769307757855, 340282366920938463463367536769307757851)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463367536769307757855n);
    input.add128(340282366920938463463367536769307757851n);
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

  it('test operator "lt" overload (euint128, euint128) => ebool test 1 (340282366920938463463369388763646972637, 340282366920938463463371378478597461095)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463369388763646972637n);
    input.add128(340282366920938463463371378478597461095n);
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

  it('test operator "lt" overload (euint128, euint128) => ebool test 2 (340282366920938463463369388763646972633, 340282366920938463463369388763646972637)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463369388763646972633n);
    input.add128(340282366920938463463369388763646972637n);
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

  it('test operator "lt" overload (euint128, euint128) => ebool test 3 (340282366920938463463369388763646972637, 340282366920938463463369388763646972637)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463369388763646972637n);
    input.add128(340282366920938463463369388763646972637n);
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

  it('test operator "lt" overload (euint128, euint128) => ebool test 4 (340282366920938463463369388763646972637, 340282366920938463463369388763646972633)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463369388763646972637n);
    input.add128(340282366920938463463369388763646972633n);
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

  it('test operator "min" overload (euint128, euint128) => euint128 test 1 (340282366920938463463373730454927869005, 340282366920938463463369490483031078365)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463373730454927869005n);
    input.add128(340282366920938463463369490483031078365n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.resEuint128());
    expect(res).to.equal(340282366920938463463369490483031078365n);
  });

  it('test operator "min" overload (euint128, euint128) => euint128 test 2 (340282366920938463463369490483031078361, 340282366920938463463369490483031078365)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463369490483031078361n);
    input.add128(340282366920938463463369490483031078365n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.resEuint128());
    expect(res).to.equal(340282366920938463463369490483031078361n);
  });

  it('test operator "min" overload (euint128, euint128) => euint128 test 3 (340282366920938463463369490483031078365, 340282366920938463463369490483031078365)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463369490483031078365n);
    input.add128(340282366920938463463369490483031078365n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.resEuint128());
    expect(res).to.equal(340282366920938463463369490483031078365n);
  });

  it('test operator "min" overload (euint128, euint128) => euint128 test 4 (340282366920938463463369490483031078365, 340282366920938463463369490483031078361)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463369490483031078365n);
    input.add128(340282366920938463463369490483031078361n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.resEuint128());
    expect(res).to.equal(340282366920938463463369490483031078361n);
  });

  it('test operator "max" overload (euint128, euint128) => euint128 test 1 (340282366920938463463366861512587033531, 340282366920938463463369746270736880557)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463366861512587033531n);
    input.add128(340282366920938463463369746270736880557n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.resEuint128());
    expect(res).to.equal(340282366920938463463369746270736880557n);
  });

  it('test operator "max" overload (euint128, euint128) => euint128 test 2 (340282366920938463463366861512587033527, 340282366920938463463366861512587033531)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463366861512587033527n);
    input.add128(340282366920938463463366861512587033531n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.resEuint128());
    expect(res).to.equal(340282366920938463463366861512587033531n);
  });

  it('test operator "max" overload (euint128, euint128) => euint128 test 3 (340282366920938463463366861512587033531, 340282366920938463463366861512587033531)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463366861512587033531n);
    input.add128(340282366920938463463366861512587033531n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.resEuint128());
    expect(res).to.equal(340282366920938463463366861512587033531n);
  });

  it('test operator "max" overload (euint128, euint128) => euint128 test 4 (340282366920938463463366861512587033531, 340282366920938463463366861512587033527)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463366861512587033531n);
    input.add128(340282366920938463463366861512587033527n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.resEuint128());
    expect(res).to.equal(340282366920938463463366861512587033531n);
  });

  it('test operator "and" overload (euint128, euint256) => euint256 test 1 (340282366920938463463370448760626316511, 115792089237316195423570985008687907853269984665640564039457575095067301402135)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463370448760626316511n);
    input.add256(115792089237316195423570985008687907853269984665640564039457575095067301402135n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(340282366920938463463365619543933715479n);
  });

  it('test operator "and" overload (euint128, euint256) => euint256 test 2 (340282366920938463463370448760626316507, 340282366920938463463370448760626316511)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463370448760626316507n);
    input.add256(340282366920938463463370448760626316511n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(340282366920938463463370448760626316507n);
  });

  it('test operator "and" overload (euint128, euint256) => euint256 test 3 (340282366920938463463370448760626316511, 340282366920938463463370448760626316511)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463370448760626316511n);
    input.add256(340282366920938463463370448760626316511n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(340282366920938463463370448760626316511n);
  });

  it('test operator "and" overload (euint128, euint256) => euint256 test 4 (340282366920938463463370448760626316511, 340282366920938463463370448760626316507)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463370448760626316511n);
    input.add256(340282366920938463463370448760626316507n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(340282366920938463463370448760626316507n);
  });

  it('test operator "or" overload (euint128, euint256) => euint256 test 1 (340282366920938463463373893210242750101, 115792089237316195423570985008687907853269984665640564039457575860037635222631)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463373893210242750101n);
    input.add256(115792089237316195423570985008687907853269984665640564039457575860037635222631n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457583867106787540727n);
  });

  it('test operator "or" overload (euint128, euint256) => euint256 test 2 (340282366920938463463373893210242750097, 340282366920938463463373893210242750101)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463373893210242750097n);
    input.add256(340282366920938463463373893210242750101n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(340282366920938463463373893210242750101n);
  });

  it('test operator "or" overload (euint128, euint256) => euint256 test 3 (340282366920938463463373893210242750101, 340282366920938463463373893210242750101)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463373893210242750101n);
    input.add256(340282366920938463463373893210242750101n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(340282366920938463463373893210242750101n);
  });

  it('test operator "or" overload (euint128, euint256) => euint256 test 4 (340282366920938463463373893210242750101, 340282366920938463463373893210242750097)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463373893210242750101n);
    input.add256(340282366920938463463373893210242750097n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(340282366920938463463373893210242750101n);
  });

  it('test operator "xor" overload (euint128, euint256) => euint256 test 1 (340282366920938463463372013735284554989, 115792089237316195423570985008687907853269984665640564039457579013933040973961)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463372013735284554989n);
    input.add256(115792089237316195423570985008687907853269984665640564039457579013933040973961n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907852929702298719625575994216308762791314532n);
  });

  it('test operator "xor" overload (euint128, euint256) => euint256 test 2 (340282366920938463463372013735284554985, 340282366920938463463372013735284554989)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463372013735284554985n);
    input.add256(340282366920938463463372013735284554989n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint128, euint256) => euint256 test 3 (340282366920938463463372013735284554989, 340282366920938463463372013735284554989)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463372013735284554989n);
    input.add256(340282366920938463463372013735284554989n);
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

  it('test operator "xor" overload (euint128, euint256) => euint256 test 4 (340282366920938463463372013735284554989, 340282366920938463463372013735284554985)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463372013735284554989n);
    input.add256(340282366920938463463372013735284554985n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint128, euint256) => ebool test 1 (340282366920938463463368906817226338137, 115792089237316195423570985008687907853269984665640564039457581183637033741583)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463368906817226338137n);
    input.add256(115792089237316195423570985008687907853269984665640564039457581183637033741583n);
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

  it('test operator "eq" overload (euint128, euint256) => ebool test 2 (340282366920938463463368906817226338133, 340282366920938463463368906817226338137)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463368906817226338133n);
    input.add256(340282366920938463463368906817226338137n);
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

  it('test operator "eq" overload (euint128, euint256) => ebool test 3 (340282366920938463463368906817226338137, 340282366920938463463368906817226338137)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463368906817226338137n);
    input.add256(340282366920938463463368906817226338137n);
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

  it('test operator "eq" overload (euint128, euint256) => ebool test 4 (340282366920938463463368906817226338137, 340282366920938463463368906817226338133)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463368906817226338137n);
    input.add256(340282366920938463463368906817226338133n);
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

  it('test operator "ne" overload (euint128, euint256) => ebool test 1 (340282366920938463463371861077614366459, 115792089237316195423570985008687907853269984665640564039457581898105835324453)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463371861077614366459n);
    input.add256(115792089237316195423570985008687907853269984665640564039457581898105835324453n);
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

  it('test operator "ne" overload (euint128, euint256) => ebool test 2 (340282366920938463463371861077614366455, 340282366920938463463371861077614366459)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463371861077614366455n);
    input.add256(340282366920938463463371861077614366459n);
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

  it('test operator "ne" overload (euint128, euint256) => ebool test 3 (340282366920938463463371861077614366459, 340282366920938463463371861077614366459)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463371861077614366459n);
    input.add256(340282366920938463463371861077614366459n);
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

  it('test operator "ne" overload (euint128, euint256) => ebool test 4 (340282366920938463463371861077614366459, 340282366920938463463371861077614366455)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463371861077614366459n);
    input.add256(340282366920938463463371861077614366455n);
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

  it('test operator "and" overload (euint256, euint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457578173524132205795, 170)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578173524132205795n);
    input.add8(170n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(162n);
  });

  it('test operator "and" overload (euint256, euint8) => euint256 test 2 (166, 170)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(166n);
    input.add8(170n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(162n);
  });

  it('test operator "and" overload (euint256, euint8) => euint256 test 3 (170, 170)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(170n);
    input.add8(170n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(170n);
  });

  it('test operator "and" overload (euint256, euint8) => euint256 test 4 (170, 166)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(170n);
    input.add8(166n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(162n);
  });

  it('test operator "or" overload (euint256, euint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457578191938637208771, 22)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578191938637208771n);
    input.add8(22n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457578191938637208791n);
  });

  it('test operator "or" overload (euint256, euint8) => euint256 test 2 (18, 22)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(18n);
    input.add8(22n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(22n);
  });

  it('test operator "or" overload (euint256, euint8) => euint256 test 3 (22, 22)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(22n);
    input.add8(22n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(22n);
  });

  it('test operator "or" overload (euint256, euint8) => euint256 test 4 (22, 18)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(22n);
    input.add8(18n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(22n);
  });

  it('test operator "xor" overload (euint256, euint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457575771173905464021, 147)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575771173905464021n);
    input.add8(147n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457575771173905463878n);
  });

  it('test operator "xor" overload (euint256, euint8) => euint256 test 2 (143, 147)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(143n);
    input.add8(147n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(28n);
  });

  it('test operator "xor" overload (euint256, euint8) => euint256 test 3 (147, 147)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(147n);
    input.add8(147n);
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

  it('test operator "xor" overload (euint256, euint8) => euint256 test 4 (147, 143)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(147n);
    input.add8(143n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(28n);
  });

  it('test operator "eq" overload (euint256, euint8) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457575226370333238975, 28)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575226370333238975n);
    input.add8(28n);
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

  it('test operator "eq" overload (euint256, euint8) => ebool test 2 (24, 28)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(24n);
    input.add8(28n);
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

  it('test operator "eq" overload (euint256, euint8) => ebool test 3 (28, 28)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(28n);
    input.add8(28n);
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

  it('test operator "eq" overload (euint256, euint8) => ebool test 4 (28, 24)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(28n);
    input.add8(24n);
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

  it('test operator "ne" overload (euint256, euint8) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457582583095991626297, 45)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582583095991626297n);
    input.add8(45n);
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

  it('test operator "ne" overload (euint256, euint8) => ebool test 2 (41, 45)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(41n);
    input.add8(45n);
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

  it('test operator "ne" overload (euint256, euint8) => ebool test 3 (45, 45)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(45n);
    input.add8(45n);
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

  it('test operator "ne" overload (euint256, euint8) => ebool test 4 (45, 41)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(45n);
    input.add8(41n);
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

  it('test operator "and" overload (euint256, euint16) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457576445786599284593, 42760)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576445786599284593n);
    input.add16(42760n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(1792n);
  });

  it('test operator "and" overload (euint256, euint16) => euint256 test 2 (42756, 42760)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(42756n);
    input.add16(42760n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(42752n);
  });

  it('test operator "and" overload (euint256, euint16) => euint256 test 3 (42760, 42760)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(42760n);
    input.add16(42760n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(42760n);
  });

  it('test operator "and" overload (euint256, euint16) => euint256 test 4 (42760, 42756)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(42760n);
    input.add16(42756n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(42752n);
  });

  it('test operator "or" overload (euint256, euint16) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457579106619107908235, 55992)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579106619107908235n);
    input.add16(55992n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457579106619107957435n);
  });

  it('test operator "or" overload (euint256, euint16) => euint256 test 2 (55988, 55992)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(55988n);
    input.add16(55992n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(55996n);
  });

  it('test operator "or" overload (euint256, euint16) => euint256 test 3 (55992, 55992)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(55992n);
    input.add16(55992n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(55992n);
  });

  it('test operator "or" overload (euint256, euint16) => euint256 test 4 (55992, 55988)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(55992n);
    input.add16(55988n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(55996n);
  });

  it('test operator "xor" overload (euint256, euint16) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457581243742205072859, 2875)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457581243742205072859n);
    input.add16(2875n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457581243742205075168n);
  });

  it('test operator "xor" overload (euint256, euint16) => euint256 test 2 (2871, 2875)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(2871n);
    input.add16(2875n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint256, euint16) => euint256 test 3 (2875, 2875)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(2875n);
    input.add16(2875n);
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

  it('test operator "xor" overload (euint256, euint16) => euint256 test 4 (2875, 2871)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(2875n);
    input.add16(2871n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint256, euint16) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457583270529480237689, 17028)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457583270529480237689n);
    input.add16(17028n);
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

  it('test operator "eq" overload (euint256, euint16) => ebool test 2 (17024, 17028)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(17024n);
    input.add16(17028n);
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

  it('test operator "eq" overload (euint256, euint16) => ebool test 3 (17028, 17028)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(17028n);
    input.add16(17028n);
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

  it('test operator "eq" overload (euint256, euint16) => ebool test 4 (17028, 17024)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(17028n);
    input.add16(17024n);
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

  it('test operator "ne" overload (euint256, euint16) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457582216258092142095, 62094)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582216258092142095n);
    input.add16(62094n);
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

  it('test operator "ne" overload (euint256, euint16) => ebool test 2 (62090, 62094)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(62090n);
    input.add16(62094n);
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

  it('test operator "ne" overload (euint256, euint16) => ebool test 3 (62094, 62094)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(62094n);
    input.add16(62094n);
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

  it('test operator "ne" overload (euint256, euint16) => ebool test 4 (62094, 62090)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(62094n);
    input.add16(62090n);
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

  it('test operator "and" overload (euint256, euint32) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457577427869945856733, 2077954391)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577427869945856733n);
    input.add32(2077954391n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(1263009877n);
  });

  it('test operator "and" overload (euint256, euint32) => euint256 test 2 (2077954387, 2077954391)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(2077954387n);
    input.add32(2077954391n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(2077954387n);
  });

  it('test operator "and" overload (euint256, euint32) => euint256 test 3 (2077954391, 2077954391)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(2077954391n);
    input.add32(2077954391n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(2077954391n);
  });

  it('test operator "and" overload (euint256, euint32) => euint256 test 4 (2077954391, 2077954387)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(2077954391n);
    input.add32(2077954387n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(2077954387n);
  });

  it('test operator "or" overload (euint256, euint32) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457576532309436497475, 2061981464)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576532309436497475n);
    input.add32(2061981464n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457576532309881356123n);
  });

  it('test operator "or" overload (euint256, euint32) => euint256 test 2 (2061981460, 2061981464)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(2061981460n);
    input.add32(2061981464n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(2061981468n);
  });

  it('test operator "or" overload (euint256, euint32) => euint256 test 3 (2061981464, 2061981464)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(2061981464n);
    input.add32(2061981464n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(2061981464n);
  });

  it('test operator "or" overload (euint256, euint32) => euint256 test 4 (2061981464, 2061981460)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(2061981464n);
    input.add32(2061981460n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(2061981468n);
  });

  it('test operator "xor" overload (euint256, euint32) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457577817517618981823, 3140792768)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577817517618981823n);
    input.add32(3140792768n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457577817518811503231n);
  });

  it('test operator "xor" overload (euint256, euint32) => euint256 test 2 (3140792764, 3140792768)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(3140792764n);
    input.add32(3140792768n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(124n);
  });

  it('test operator "xor" overload (euint256, euint32) => euint256 test 3 (3140792768, 3140792768)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(3140792768n);
    input.add32(3140792768n);
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

  it('test operator "xor" overload (euint256, euint32) => euint256 test 4 (3140792768, 3140792764)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(3140792768n);
    input.add32(3140792764n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(124n);
  });

  it('test operator "eq" overload (euint256, euint32) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457581783374988831535, 2459220002)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457581783374988831535n);
    input.add32(2459220002n);
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

  it('test operator "eq" overload (euint256, euint32) => ebool test 2 (2459219998, 2459220002)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(2459219998n);
    input.add32(2459220002n);
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

  it('test operator "eq" overload (euint256, euint32) => ebool test 3 (2459220002, 2459220002)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(2459220002n);
    input.add32(2459220002n);
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

  it('test operator "eq" overload (euint256, euint32) => ebool test 4 (2459220002, 2459219998)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(2459220002n);
    input.add32(2459219998n);
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

  it('test operator "ne" overload (euint256, euint32) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457578635710426210511, 3252431528)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578635710426210511n);
    input.add32(3252431528n);
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

  it('test operator "ne" overload (euint256, euint32) => ebool test 2 (3252431524, 3252431528)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(3252431524n);
    input.add32(3252431528n);
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

  it('test operator "ne" overload (euint256, euint32) => ebool test 3 (3252431528, 3252431528)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(3252431528n);
    input.add32(3252431528n);
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

  it('test operator "ne" overload (euint256, euint32) => ebool test 4 (3252431528, 3252431524)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(3252431528n);
    input.add32(3252431524n);
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

  it('test operator "and" overload (euint256, euint64) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457582997686206485323, 18445677503915709591)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582997686206485323n);
    input.add64(18445677503915709591n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(18445654681394499587n);
  });

  it('test operator "and" overload (euint256, euint64) => euint256 test 2 (18445677503915709587, 18445677503915709591)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(18445677503915709587n);
    input.add64(18445677503915709591n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(18445677503915709587n);
  });

  it('test operator "and" overload (euint256, euint64) => euint256 test 3 (18445677503915709591, 18445677503915709591)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(18445677503915709591n);
    input.add64(18445677503915709591n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(18445677503915709591n);
  });

  it('test operator "and" overload (euint256, euint64) => euint256 test 4 (18445677503915709591, 18445677503915709587)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(18445677503915709591n);
    input.add64(18445677503915709587n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(18445677503915709587n);
  });

  it('test operator "or" overload (euint256, euint64) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457581281327112025429, 18441417511778818287)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457581281327112025429n);
    input.add64(18441417511778818287n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457583818450193546751n);
  });

  it('test operator "or" overload (euint256, euint64) => euint256 test 2 (18441417511778818283, 18441417511778818287)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(18441417511778818283n);
    input.add64(18441417511778818287n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(18441417511778818287n);
  });

  it('test operator "or" overload (euint256, euint64) => euint256 test 3 (18441417511778818287, 18441417511778818287)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(18441417511778818287n);
    input.add64(18441417511778818287n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(18441417511778818287n);
  });

  it('test operator "or" overload (euint256, euint64) => euint256 test 4 (18441417511778818287, 18441417511778818283)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(18441417511778818287n);
    input.add64(18441417511778818283n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(18441417511778818287n);
  });

  it('test operator "xor" overload (euint256, euint64) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457581065005059042503, 18445034011911955801)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457581065005059042503n);
    input.add64(18445034011911955801n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039439140755638836709790n);
  });

  it('test operator "xor" overload (euint256, euint64) => euint256 test 2 (18445034011911955797, 18445034011911955801)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(18445034011911955797n);
    input.add64(18445034011911955801n);
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

  it('test operator "xor" overload (euint256, euint64) => euint256 test 3 (18445034011911955801, 18445034011911955801)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(18445034011911955801n);
    input.add64(18445034011911955801n);
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

  it('test operator "xor" overload (euint256, euint64) => euint256 test 4 (18445034011911955801, 18445034011911955797)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(18445034011911955801n);
    input.add64(18445034011911955797n);
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

  it('test operator "eq" overload (euint256, euint64) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457575739630011328085, 18437988490284926361)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575739630011328085n);
    input.add64(18437988490284926361n);
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

  it('test operator "eq" overload (euint256, euint64) => ebool test 2 (18437988490284926357, 18437988490284926361)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(18437988490284926357n);
    input.add64(18437988490284926361n);
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

  it('test operator "eq" overload (euint256, euint64) => ebool test 3 (18437988490284926361, 18437988490284926361)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(18437988490284926361n);
    input.add64(18437988490284926361n);
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

  it('test operator "eq" overload (euint256, euint64) => ebool test 4 (18437988490284926361, 18437988490284926357)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(18437988490284926361n);
    input.add64(18437988490284926357n);
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

  it('test operator "ne" overload (euint256, euint64) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457577317589855847465, 18444250274203077585)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577317589855847465n);
    input.add64(18444250274203077585n);
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

  it('test operator "ne" overload (euint256, euint64) => ebool test 2 (18444250274203077581, 18444250274203077585)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(18444250274203077581n);
    input.add64(18444250274203077585n);
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

  it('test operator "ne" overload (euint256, euint64) => ebool test 3 (18444250274203077585, 18444250274203077585)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(18444250274203077585n);
    input.add64(18444250274203077585n);
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

  it('test operator "ne" overload (euint256, euint64) => ebool test 4 (18444250274203077585, 18444250274203077581)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(18444250274203077585n);
    input.add64(18444250274203077581n);
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

  it('test operator "and" overload (euint256, euint128) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457580019389117327691, 340282366920938463463368249048518698897)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580019389117327691n);
    input.add128(340282366920938463463368249048518698897n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(340282366920938463463365952166446571777n);
  });

  it('test operator "and" overload (euint256, euint128) => euint256 test 2 (340282366920938463463368249048518698893, 340282366920938463463368249048518698897)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(340282366920938463463368249048518698893n);
    input.add128(340282366920938463463368249048518698897n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(340282366920938463463368249048518698881n);
  });

  it('test operator "and" overload (euint256, euint128) => euint256 test 3 (340282366920938463463368249048518698897, 340282366920938463463368249048518698897)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(340282366920938463463368249048518698897n);
    input.add128(340282366920938463463368249048518698897n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(340282366920938463463368249048518698897n);
  });

  it('test operator "and" overload (euint256, euint128) => euint256 test 4 (340282366920938463463368249048518698897, 340282366920938463463368249048518698893)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(340282366920938463463368249048518698897n);
    input.add128(340282366920938463463368249048518698893n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(340282366920938463463368249048518698881n);
  });

  it('test operator "or" overload (euint256, euint128) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457577650797352793637, 340282366920938463463367280105944308591)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577650797352793637n);
    input.add128(340282366920938463463367280105944308591n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457578941357766999919n);
  });

  it('test operator "or" overload (euint256, euint128) => euint256 test 2 (340282366920938463463367280105944308587, 340282366920938463463367280105944308591)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(340282366920938463463367280105944308587n);
    input.add128(340282366920938463463367280105944308591n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(340282366920938463463367280105944308591n);
  });

  it('test operator "or" overload (euint256, euint128) => euint256 test 3 (340282366920938463463367280105944308591, 340282366920938463463367280105944308591)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(340282366920938463463367280105944308591n);
    input.add128(340282366920938463463367280105944308591n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(340282366920938463463367280105944308591n);
  });

  it('test operator "or" overload (euint256, euint128) => euint256 test 4 (340282366920938463463367280105944308591, 340282366920938463463367280105944308587)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(340282366920938463463367280105944308591n);
    input.add128(340282366920938463463367280105944308587n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(340282366920938463463367280105944308591n);
  });

  it('test operator "xor" overload (euint256, euint128) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457576859149813939715, 340282366920938463463371440957858593557)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576859149813939715n);
    input.add128(340282366920938463463371440957858593557n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907852929702298719625575994214566154535879958n);
  });

  it('test operator "xor" overload (euint256, euint128) => euint256 test 2 (340282366920938463463371440957858593553, 340282366920938463463371440957858593557)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(340282366920938463463371440957858593553n);
    input.add128(340282366920938463463371440957858593557n);
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

  it('test operator "xor" overload (euint256, euint128) => euint256 test 3 (340282366920938463463371440957858593557, 340282366920938463463371440957858593557)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(340282366920938463463371440957858593557n);
    input.add128(340282366920938463463371440957858593557n);
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

  it('test operator "xor" overload (euint256, euint128) => euint256 test 4 (340282366920938463463371440957858593557, 340282366920938463463371440957858593553)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(340282366920938463463371440957858593557n);
    input.add128(340282366920938463463371440957858593553n);
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

  it('test operator "eq" overload (euint256, euint128) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457578775108021398551, 340282366920938463463366018360898790733)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578775108021398551n);
    input.add128(340282366920938463463366018360898790733n);
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

  it('test operator "eq" overload (euint256, euint128) => ebool test 2 (340282366920938463463366018360898790729, 340282366920938463463366018360898790733)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(340282366920938463463366018360898790729n);
    input.add128(340282366920938463463366018360898790733n);
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

  it('test operator "eq" overload (euint256, euint128) => ebool test 3 (340282366920938463463366018360898790733, 340282366920938463463366018360898790733)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(340282366920938463463366018360898790733n);
    input.add128(340282366920938463463366018360898790733n);
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

  it('test operator "eq" overload (euint256, euint128) => ebool test 4 (340282366920938463463366018360898790733, 340282366920938463463366018360898790729)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(340282366920938463463366018360898790733n);
    input.add128(340282366920938463463366018360898790729n);
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

  it('test operator "ne" overload (euint256, euint128) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457579931552643066571, 340282366920938463463367922418751300671)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579931552643066571n);
    input.add128(340282366920938463463367922418751300671n);
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

  it('test operator "ne" overload (euint256, euint128) => ebool test 2 (340282366920938463463367922418751300667, 340282366920938463463367922418751300671)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(340282366920938463463367922418751300667n);
    input.add128(340282366920938463463367922418751300671n);
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

  it('test operator "ne" overload (euint256, euint128) => ebool test 3 (340282366920938463463367922418751300671, 340282366920938463463367922418751300671)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(340282366920938463463367922418751300671n);
    input.add128(340282366920938463463367922418751300671n);
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

  it('test operator "ne" overload (euint256, euint128) => ebool test 4 (340282366920938463463367922418751300671, 340282366920938463463367922418751300667)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(340282366920938463463367922418751300671n);
    input.add128(340282366920938463463367922418751300667n);
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

  it('test operator "and" overload (euint256, euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457578158911356822319, 115792089237316195423570985008687907853269984665640564039457577433832543175215)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578158911356822319n);
    input.add256(115792089237316195423570985008687907853269984665640564039457577433832543175215n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457577288007382876719n);
  });

  it('test operator "and" overload (euint256, euint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457577433832543175211, 115792089237316195423570985008687907853269984665640564039457577433832543175215)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577433832543175211n);
    input.add256(115792089237316195423570985008687907853269984665640564039457577433832543175215n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457577433832543175211n);
  });

  it('test operator "and" overload (euint256, euint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457577433832543175215, 115792089237316195423570985008687907853269984665640564039457577433832543175215)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577433832543175215n);
    input.add256(115792089237316195423570985008687907853269984665640564039457577433832543175215n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457577433832543175215n);
  });

  it('test operator "and" overload (euint256, euint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457577433832543175215, 115792089237316195423570985008687907853269984665640564039457577433832543175211)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577433832543175215n);
    input.add256(115792089237316195423570985008687907853269984665640564039457577433832543175211n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457577433832543175211n);
  });

  it('test operator "or" overload (euint256, euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457579426915075813205, 115792089237316195423570985008687907853269984665640564039457579104746415170017)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579426915075813205n);
    input.add256(115792089237316195423570985008687907853269984665640564039457579104746415170017n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457579431315432665077n);
  });

  it('test operator "or" overload (euint256, euint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457579104746415170013, 115792089237316195423570985008687907853269984665640564039457579104746415170017)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579104746415170013n);
    input.add256(115792089237316195423570985008687907853269984665640564039457579104746415170017n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457579104746415170045n);
  });

  it('test operator "or" overload (euint256, euint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457579104746415170017, 115792089237316195423570985008687907853269984665640564039457579104746415170017)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579104746415170017n);
    input.add256(115792089237316195423570985008687907853269984665640564039457579104746415170017n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457579104746415170017n);
  });

  it('test operator "or" overload (euint256, euint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457579104746415170017, 115792089237316195423570985008687907853269984665640564039457579104746415170013)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579104746415170017n);
    input.add256(115792089237316195423570985008687907853269984665640564039457579104746415170013n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457579104746415170045n);
  });

  it('test operator "xor" overload (euint256, euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457580818936652567875, 115792089237316195423570985008687907853269984665640564039457582450487997264309)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580818936652567875n);
    input.add256(115792089237316195423570985008687907853269984665640564039457582450487997264309n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(4182693526594806n);
  });

  it('test operator "xor" overload (euint256, euint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457580818936652567871, 115792089237316195423570985008687907853269984665640564039457580818936652567875)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580818936652567871n);
    input.add256(115792089237316195423570985008687907853269984665640564039457580818936652567875n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(124n);
  });

  it('test operator "xor" overload (euint256, euint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457580818936652567875, 115792089237316195423570985008687907853269984665640564039457580818936652567875)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580818936652567875n);
    input.add256(115792089237316195423570985008687907853269984665640564039457580818936652567875n);
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

  it('test operator "xor" overload (euint256, euint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457580818936652567875, 115792089237316195423570985008687907853269984665640564039457580818936652567871)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580818936652567875n);
    input.add256(115792089237316195423570985008687907853269984665640564039457580818936652567871n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(124n);
  });

  it('test operator "eq" overload (euint256, euint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457576242795079425591, 115792089237316195423570985008687907853269984665640564039457579695974999614303)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576242795079425591n);
    input.add256(115792089237316195423570985008687907853269984665640564039457579695974999614303n);
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

  it('test operator "eq" overload (euint256, euint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457576242795079425587, 115792089237316195423570985008687907853269984665640564039457576242795079425591)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576242795079425587n);
    input.add256(115792089237316195423570985008687907853269984665640564039457576242795079425591n);
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

  it('test operator "eq" overload (euint256, euint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457576242795079425591, 115792089237316195423570985008687907853269984665640564039457576242795079425591)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576242795079425591n);
    input.add256(115792089237316195423570985008687907853269984665640564039457576242795079425591n);
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

  it('test operator "eq" overload (euint256, euint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457576242795079425591, 115792089237316195423570985008687907853269984665640564039457576242795079425587)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576242795079425591n);
    input.add256(115792089237316195423570985008687907853269984665640564039457576242795079425587n);
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

  it('test operator "ne" overload (euint256, euint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457582911719561182299, 115792089237316195423570985008687907853269984665640564039457582299413180208323)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582911719561182299n);
    input.add256(115792089237316195423570985008687907853269984665640564039457582299413180208323n);
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

  it('test operator "ne" overload (euint256, euint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457582299413180208319, 115792089237316195423570985008687907853269984665640564039457582299413180208323)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582299413180208319n);
    input.add256(115792089237316195423570985008687907853269984665640564039457582299413180208323n);
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

  it('test operator "ne" overload (euint256, euint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457582299413180208323, 115792089237316195423570985008687907853269984665640564039457582299413180208323)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582299413180208323n);
    input.add256(115792089237316195423570985008687907853269984665640564039457582299413180208323n);
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

  it('test operator "ne" overload (euint256, euint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457582299413180208323, 115792089237316195423570985008687907853269984665640564039457582299413180208319)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582299413180208323n);
    input.add256(115792089237316195423570985008687907853269984665640564039457582299413180208319n);
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

  it('test operator "add" overload (euint8, uint8) => euint8 test 1 (40, 103)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(40n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint8_uint8(encryptedAmount.handles[0], 103n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(143n);
  });

  it('test operator "add" overload (euint8, uint8) => euint8 test 2 (36, 40)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(36n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint8_uint8(encryptedAmount.handles[0], 40n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(76n);
  });

  it('test operator "add" overload (euint8, uint8) => euint8 test 3 (40, 40)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(40n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint8_uint8(encryptedAmount.handles[0], 40n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(80n);
  });

  it('test operator "add" overload (euint8, uint8) => euint8 test 4 (40, 36)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(40n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint8_uint8(encryptedAmount.handles[0], 36n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(76n);
  });

  it('test operator "add" overload (uint8, euint8) => euint8 test 1 (54, 103)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(103n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_uint8_euint8(54n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(157n);
  });

  it('test operator "add" overload (uint8, euint8) => euint8 test 2 (36, 40)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(40n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_uint8_euint8(36n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(76n);
  });

  it('test operator "add" overload (uint8, euint8) => euint8 test 3 (40, 40)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(40n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_uint8_euint8(40n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(80n);
  });

  it('test operator "add" overload (uint8, euint8) => euint8 test 4 (40, 36)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(36n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_uint8_euint8(40n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(76n);
  });

  it('test operator "sub" overload (euint8, uint8) => euint8 test 1 (99, 99)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(99n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_euint8_uint8(encryptedAmount.handles[0], 99n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint8, uint8) => euint8 test 2 (99, 95)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(99n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_euint8_uint8(encryptedAmount.handles[0], 95n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(4n);
  });

  it('test operator "sub" overload (uint8, euint8) => euint8 test 1 (99, 99)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(99n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_uint8_euint8(99n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (uint8, euint8) => euint8 test 2 (99, 95)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(95n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_uint8_euint8(99n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint8, uint8) => euint8 test 1 (12, 15)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(12n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint8_uint8(encryptedAmount.handles[0], 15n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(180n);
  });

  it('test operator "mul" overload (euint8, uint8) => euint8 test 2 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(9n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint8_uint8(encryptedAmount.handles[0], 9n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(81n);
  });

  it('test operator "mul" overload (euint8, uint8) => euint8 test 3 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(9n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint8_uint8(encryptedAmount.handles[0], 9n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(81n);
  });

  it('test operator "mul" overload (euint8, uint8) => euint8 test 4 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(9n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint8_uint8(encryptedAmount.handles[0], 9n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(81n);
  });

  it('test operator "mul" overload (uint8, euint8) => euint8 test 1 (17, 15)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(15n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_uint8_euint8(17n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(255n);
  });

  it('test operator "mul" overload (uint8, euint8) => euint8 test 2 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_uint8_euint8(9n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(81n);
  });

  it('test operator "mul" overload (uint8, euint8) => euint8 test 3 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_uint8_euint8(9n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(81n);
  });

  it('test operator "mul" overload (uint8, euint8) => euint8 test 4 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_uint8_euint8(9n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(81n);
  });

  it('test operator "div" overload (euint8, uint8) => euint8 test 1 (87, 132)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(87n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.div_euint8_uint8(encryptedAmount.handles[0], 132n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint8, uint8) => euint8 test 2 (83, 87)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(83n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.div_euint8_uint8(encryptedAmount.handles[0], 87n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint8, uint8) => euint8 test 3 (87, 87)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(87n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.div_euint8_uint8(encryptedAmount.handles[0], 87n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint8, uint8) => euint8 test 4 (87, 83)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(87n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.div_euint8_uint8(encryptedAmount.handles[0], 83n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(1n);
  });

  it('test operator "rem" overload (euint8, uint8) => euint8 test 1 (16, 130)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(16n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rem_euint8_uint8(encryptedAmount.handles[0], 130n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(16n);
  });

  it('test operator "rem" overload (euint8, uint8) => euint8 test 2 (12, 16)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(12n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rem_euint8_uint8(encryptedAmount.handles[0], 16n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(12n);
  });

  it('test operator "rem" overload (euint8, uint8) => euint8 test 3 (16, 16)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(16n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rem_euint8_uint8(encryptedAmount.handles[0], 16n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(0n);
  });

  it('test operator "rem" overload (euint8, uint8) => euint8 test 4 (16, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(16n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rem_euint8_uint8(encryptedAmount.handles[0], 12n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(4n);
  });

  it('test operator "and" overload (euint8, uint8) => euint8 test 1 (73, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(73n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint8_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(1n);
  });

  it('test operator "and" overload (euint8, uint8) => euint8 test 2 (66, 70)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(66n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint8_uint8(encryptedAmount.handles[0], 70n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(66n);
  });

  it('test operator "and" overload (euint8, uint8) => euint8 test 3 (70, 70)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(70n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint8_uint8(encryptedAmount.handles[0], 70n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(70n);
  });

  it('test operator "and" overload (euint8, uint8) => euint8 test 4 (70, 66)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(70n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint8_uint8(encryptedAmount.handles[0], 66n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(66n);
  });

  it('test operator "and" overload (uint8, euint8) => euint8 test 1 (55, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_uint8_euint8(55n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(7n);
  });

  it('test operator "and" overload (uint8, euint8) => euint8 test 2 (66, 70)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(70n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_uint8_euint8(66n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(66n);
  });

  it('test operator "and" overload (uint8, euint8) => euint8 test 3 (70, 70)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(70n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_uint8_euint8(70n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(70n);
  });

  it('test operator "and" overload (uint8, euint8) => euint8 test 4 (70, 66)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(66n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_uint8_euint8(70n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(66n);
  });

  it('test operator "or" overload (euint8, uint8) => euint8 test 1 (38, 54)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(38n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint8_uint8(encryptedAmount.handles[0], 54n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(54n);
  });

  it('test operator "or" overload (euint8, uint8) => euint8 test 2 (34, 38)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(34n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint8_uint8(encryptedAmount.handles[0], 38n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(38n);
  });

  it('test operator "or" overload (euint8, uint8) => euint8 test 3 (38, 38)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(38n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint8_uint8(encryptedAmount.handles[0], 38n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(38n);
  });

  it('test operator "or" overload (euint8, uint8) => euint8 test 4 (38, 34)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(38n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint8_uint8(encryptedAmount.handles[0], 34n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(38n);
  });
});
