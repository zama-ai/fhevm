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

describe('FHEVM operations 11', function () {
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

  it('test operator "lt" overload (uint64, euint64) => ebool test 1 (18442112281788702681, 18445098809648433805)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18445098809648433805n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint64_euint64(
      18442112281788702681n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint64, euint64) => ebool test 2 (18439528997555571215, 18439528997555571219)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18439528997555571219n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint64_euint64(
      18439528997555571215n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint64, euint64) => ebool test 3 (18439528997555571219, 18439528997555571219)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18439528997555571219n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint64_euint64(
      18439528997555571219n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint64, euint64) => ebool test 4 (18439528997555571219, 18439528997555571215)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18439528997555571215n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint64_euint64(
      18439528997555571219n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, uint64) => euint64 test 1 (18441823953720517223, 18445998158017731793)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18441823953720517223n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint64_uint64(
      encryptedAmount.handles[0],
      18445998158017731793n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18441823953720517223n);
  });

  it('test operator "min" overload (euint64, uint64) => euint64 test 2 (18441823953720517219, 18441823953720517223)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18441823953720517219n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint64_uint64(
      encryptedAmount.handles[0],
      18441823953720517223n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18441823953720517219n);
  });

  it('test operator "min" overload (euint64, uint64) => euint64 test 3 (18441823953720517223, 18441823953720517223)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18441823953720517223n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint64_uint64(
      encryptedAmount.handles[0],
      18441823953720517223n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18441823953720517223n);
  });

  it('test operator "min" overload (euint64, uint64) => euint64 test 4 (18441823953720517223, 18441823953720517219)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18441823953720517223n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint64_uint64(
      encryptedAmount.handles[0],
      18441823953720517219n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18441823953720517219n);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 1 (18443190390985963021, 18445998158017731793)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18445998158017731793n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint64_euint64(
      18443190390985963021n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18443190390985963021n);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 2 (18441823953720517219, 18441823953720517223)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18441823953720517223n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint64_euint64(
      18441823953720517219n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18441823953720517219n);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 3 (18441823953720517223, 18441823953720517223)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18441823953720517223n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint64_euint64(
      18441823953720517223n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18441823953720517223n);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 4 (18441823953720517223, 18441823953720517219)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18441823953720517219n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint64_euint64(
      18441823953720517223n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18441823953720517219n);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 1 (18443639926415614305, 18438600297804249647)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18443639926415614305n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint64_uint64(
      encryptedAmount.handles[0],
      18438600297804249647n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18443639926415614305n);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 2 (18438561345275597753, 18438561345275597757)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18438561345275597753n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint64_uint64(
      encryptedAmount.handles[0],
      18438561345275597757n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18438561345275597757n);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 3 (18438561345275597757, 18438561345275597757)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18438561345275597757n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint64_uint64(
      encryptedAmount.handles[0],
      18438561345275597757n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18438561345275597757n);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 4 (18438561345275597757, 18438561345275597753)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18438561345275597757n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint64_uint64(
      encryptedAmount.handles[0],
      18438561345275597753n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18438561345275597757n);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 1 (18442596139812243651, 18438600297804249647)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18438600297804249647n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint64_euint64(
      18442596139812243651n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18442596139812243651n);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 2 (18438561345275597753, 18438561345275597757)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18438561345275597757n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint64_euint64(
      18438561345275597753n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18438561345275597757n);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 3 (18438561345275597757, 18438561345275597757)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18438561345275597757n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint64_euint64(
      18438561345275597757n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18438561345275597757n);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 4 (18438561345275597757, 18438561345275597753)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18438561345275597753n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint64_euint64(
      18438561345275597757n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18438561345275597757n);
  });

  it('test operator "add" overload (euint128, uint128) => euint128 test 1 (170141183460469231731683660477207909513, 170141183460469231731685551579543043490)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(170141183460469231731683660477207909513n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint128_uint128(
      encryptedAmount.handles[0],
      170141183460469231731685551579543043490n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463369212056750953003n);
  });

  it('test operator "add" overload (euint128, uint128) => euint128 test 2 (170141183460469231731683131555950184093, 170141183460469231731683131555950184095)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(170141183460469231731683131555950184093n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint128_uint128(
      encryptedAmount.handles[0],
      170141183460469231731683131555950184095n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463366263111900368188n);
  });

  it('test operator "add" overload (euint128, uint128) => euint128 test 3 (170141183460469231731683131555950184095, 170141183460469231731683131555950184095)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(170141183460469231731683131555950184095n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint128_uint128(
      encryptedAmount.handles[0],
      170141183460469231731683131555950184095n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463366263111900368190n);
  });

  it('test operator "add" overload (euint128, uint128) => euint128 test 4 (170141183460469231731683131555950184095, 170141183460469231731683131555950184093)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(170141183460469231731683131555950184095n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint128_uint128(
      encryptedAmount.handles[0],
      170141183460469231731683131555950184093n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463366263111900368188n);
  });

  it('test operator "add" overload (uint128, euint128) => euint128 test 1 (170141183460469231731687118770020130429, 170141183460469231731685551579543043490)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(170141183460469231731685551579543043490n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_uint128_euint128(
      170141183460469231731687118770020130429n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463372670349563173919n);
  });

  it('test operator "add" overload (uint128, euint128) => euint128 test 2 (170141183460469231731683131555950184093, 170141183460469231731683131555950184095)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(170141183460469231731683131555950184095n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_uint128_euint128(
      170141183460469231731683131555950184093n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463366263111900368188n);
  });

  it('test operator "add" overload (uint128, euint128) => euint128 test 3 (170141183460469231731683131555950184095, 170141183460469231731683131555950184095)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(170141183460469231731683131555950184095n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_uint128_euint128(
      170141183460469231731683131555950184095n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463366263111900368190n);
  });

  it('test operator "add" overload (uint128, euint128) => euint128 test 4 (170141183460469231731683131555950184095, 170141183460469231731683131555950184093)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(170141183460469231731683131555950184093n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_uint128_euint128(
      170141183460469231731683131555950184095n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463366263111900368188n);
  });

  it('test operator "sub" overload (euint128, uint128) => euint128 test 1 (340282366920938463463371992799637379637, 340282366920938463463371992799637379637)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463371992799637379637n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463371992799637379637n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint128, uint128) => euint128 test 2 (340282366920938463463371992799637379637, 340282366920938463463371992799637379633)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463371992799637379637n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463371992799637379633n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "sub" overload (uint128, euint128) => euint128 test 1 (340282366920938463463371992799637379637, 340282366920938463463371992799637379637)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(340282366920938463463371992799637379637n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_uint128_euint128(
      340282366920938463463371992799637379637n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (uint128, euint128) => euint128 test 2 (340282366920938463463371992799637379637, 340282366920938463463371992799637379633)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(340282366920938463463371992799637379633n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_uint128_euint128(
      340282366920938463463371992799637379637n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint128, uint128) => euint128 test 1 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(9223372036854775809n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint128_uint128(
      encryptedAmount.handles[0],
      9223372036854775809n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "mul" overload (euint128, uint128) => euint128 test 2 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(9223372036854775809n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint128_uint128(
      encryptedAmount.handles[0],
      9223372036854775809n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "mul" overload (euint128, uint128) => euint128 test 3 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(9223372036854775809n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint128_uint128(
      encryptedAmount.handles[0],
      9223372036854775809n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "mul" overload (euint128, uint128) => euint128 test 4 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(9223372036854775809n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint128_uint128(
      encryptedAmount.handles[0],
      9223372036854775809n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "mul" overload (uint128, euint128) => euint128 test 1 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_uint128_euint128(
      9223372036854775809n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "mul" overload (uint128, euint128) => euint128 test 2 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_uint128_euint128(
      9223372036854775809n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "mul" overload (uint128, euint128) => euint128 test 3 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_uint128_euint128(
      9223372036854775809n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "mul" overload (uint128, euint128) => euint128 test 4 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_uint128_euint128(
      9223372036854775809n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "div" overload (euint128, uint128) => euint128 test 1 (340282366920938463463369265467522813597, 340282366920938463463371456033369517087)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463369265467522813597n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.div_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463371456033369517087n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint128, uint128) => euint128 test 2 (340282366920938463463368198820666522839, 340282366920938463463368198820666522843)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463368198820666522839n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.div_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368198820666522843n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint128, uint128) => euint128 test 3 (340282366920938463463368198820666522843, 340282366920938463463368198820666522843)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463368198820666522843n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.div_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368198820666522843n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint128, uint128) => euint128 test 4 (340282366920938463463368198820666522843, 340282366920938463463368198820666522839)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463368198820666522843n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.div_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368198820666522839n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(1n);
  });

  it('test operator "rem" overload (euint128, uint128) => euint128 test 1 (340282366920938463463373742720787503951, 340282366920938463463367150249971702261)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463373742720787503951n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rem_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463367150249971702261n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(6592470815801690n);
  });

  it('test operator "rem" overload (euint128, uint128) => euint128 test 2 (340282366920938463463373742720787503947, 340282366920938463463373742720787503951)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463373742720787503947n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rem_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463373742720787503951n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463373742720787503947n);
  });

  it('test operator "rem" overload (euint128, uint128) => euint128 test 3 (340282366920938463463373742720787503951, 340282366920938463463373742720787503951)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463373742720787503951n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rem_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463373742720787503951n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "rem" overload (euint128, uint128) => euint128 test 4 (340282366920938463463373742720787503951, 340282366920938463463373742720787503947)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463373742720787503951n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rem_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463373742720787503947n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "and" overload (euint128, uint128) => euint128 test 1 (340282366920938463463371932248882114205, 340282366920938463463374399520861696199)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463371932248882114205n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463374399520861696199n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463371866243789561989n);
  });

  it('test operator "and" overload (euint128, uint128) => euint128 test 2 (340282366920938463463371545091546498821, 340282366920938463463371545091546498825)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463371545091546498821n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463371545091546498825n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463371545091546498817n);
  });

  it('test operator "and" overload (euint128, uint128) => euint128 test 3 (340282366920938463463371545091546498825, 340282366920938463463371545091546498825)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463371545091546498825n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463371545091546498825n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463371545091546498825n);
  });

  it('test operator "and" overload (euint128, uint128) => euint128 test 4 (340282366920938463463371545091546498825, 340282366920938463463371545091546498821)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463371545091546498825n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463371545091546498821n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463371545091546498817n);
  });

  it('test operator "and" overload (uint128, euint128) => euint128 test 1 (340282366920938463463370605242580756601, 340282366920938463463374399520861696199)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(340282366920938463463374399520861696199n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_uint128_euint128(
      340282366920938463463370605242580756601n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463370455708879298625n);
  });

  it('test operator "and" overload (uint128, euint128) => euint128 test 2 (340282366920938463463371545091546498821, 340282366920938463463371545091546498825)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(340282366920938463463371545091546498825n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_uint128_euint128(
      340282366920938463463371545091546498821n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463371545091546498817n);
  });

  it('test operator "and" overload (uint128, euint128) => euint128 test 3 (340282366920938463463371545091546498825, 340282366920938463463371545091546498825)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(340282366920938463463371545091546498825n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_uint128_euint128(
      340282366920938463463371545091546498825n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463371545091546498825n);
  });

  it('test operator "and" overload (uint128, euint128) => euint128 test 4 (340282366920938463463371545091546498825, 340282366920938463463371545091546498821)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(340282366920938463463371545091546498821n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_uint128_euint128(
      340282366920938463463371545091546498825n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463371545091546498817n);
  });

  it('test operator "or" overload (euint128, uint128) => euint128 test 1 (340282366920938463463372292218849760581, 340282366920938463463370827997210694141)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463372292218849760581n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463370827997210694141n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463372310414505268733n);
  });

  it('test operator "or" overload (euint128, uint128) => euint128 test 2 (340282366920938463463369412692211173585, 340282366920938463463369412692211173589)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463369412692211173585n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463369412692211173589n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463369412692211173589n);
  });

  it('test operator "or" overload (euint128, uint128) => euint128 test 3 (340282366920938463463369412692211173589, 340282366920938463463369412692211173589)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463369412692211173589n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463369412692211173589n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463369412692211173589n);
  });

  it('test operator "or" overload (euint128, uint128) => euint128 test 4 (340282366920938463463369412692211173589, 340282366920938463463369412692211173585)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463369412692211173589n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463369412692211173585n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463369412692211173589n);
  });

  it('test operator "or" overload (uint128, euint128) => euint128 test 1 (340282366920938463463373872637035190483, 340282366920938463463370827997210694141)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(340282366920938463463370827997210694141n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_uint128_euint128(
      340282366920938463463373872637035190483n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463374593962046118399n);
  });

  it('test operator "or" overload (uint128, euint128) => euint128 test 2 (340282366920938463463369412692211173585, 340282366920938463463369412692211173589)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(340282366920938463463369412692211173589n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_uint128_euint128(
      340282366920938463463369412692211173585n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463369412692211173589n);
  });

  it('test operator "or" overload (uint128, euint128) => euint128 test 3 (340282366920938463463369412692211173589, 340282366920938463463369412692211173589)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(340282366920938463463369412692211173589n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_uint128_euint128(
      340282366920938463463369412692211173589n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463369412692211173589n);
  });

  it('test operator "or" overload (uint128, euint128) => euint128 test 4 (340282366920938463463369412692211173589, 340282366920938463463369412692211173585)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(340282366920938463463369412692211173585n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_uint128_euint128(
      340282366920938463463369412692211173589n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463369412692211173589n);
  });

  it('test operator "xor" overload (euint128, uint128) => euint128 test 1 (340282366920938463463374282009991941377, 340282366920938463463367412849758519023)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463374282009991941377n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463367412849758519023n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(6940765961761774n);
  });

  it('test operator "xor" overload (euint128, uint128) => euint128 test 2 (340282366920938463463370992300019806541, 340282366920938463463370992300019806545)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463370992300019806541n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463370992300019806545n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(28n);
  });

  it('test operator "xor" overload (euint128, uint128) => euint128 test 3 (340282366920938463463370992300019806545, 340282366920938463463370992300019806545)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463370992300019806545n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463370992300019806545n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint128, uint128) => euint128 test 4 (340282366920938463463370992300019806545, 340282366920938463463370992300019806541)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463370992300019806545n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463370992300019806541n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(28n);
  });

  it('test operator "xor" overload (uint128, euint128) => euint128 test 1 (340282366920938463463367651154091815725, 340282366920938463463367412849758519023)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(340282366920938463463367412849758519023n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_uint128_euint128(
      340282366920938463463367651154091815725n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(345099332652482n);
  });

  it('test operator "xor" overload (uint128, euint128) => euint128 test 2 (340282366920938463463370992300019806541, 340282366920938463463370992300019806545)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(340282366920938463463370992300019806545n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_uint128_euint128(
      340282366920938463463370992300019806541n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(28n);
  });

  it('test operator "xor" overload (uint128, euint128) => euint128 test 3 (340282366920938463463370992300019806545, 340282366920938463463370992300019806545)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(340282366920938463463370992300019806545n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_uint128_euint128(
      340282366920938463463370992300019806545n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (uint128, euint128) => euint128 test 4 (340282366920938463463370992300019806545, 340282366920938463463370992300019806541)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(340282366920938463463370992300019806541n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_uint128_euint128(
      340282366920938463463370992300019806545n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(28n);
  });

  it('test operator "eq" overload (euint128, uint128) => ebool test 1 (340282366920938463463369660429237688665, 340282366920938463463367360854418161167)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463369660429237688665n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463367360854418161167n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, uint128) => ebool test 2 (340282366920938463463369660429237688661, 340282366920938463463369660429237688665)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463369660429237688661n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463369660429237688665n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, uint128) => ebool test 3 (340282366920938463463369660429237688665, 340282366920938463463369660429237688665)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463369660429237688665n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463369660429237688665n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint128, uint128) => ebool test 4 (340282366920938463463369660429237688665, 340282366920938463463369660429237688661)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463369660429237688665n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463369660429237688661n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint128, euint128) => ebool test 1 (340282366920938463463370684765186274371, 340282366920938463463367360854418161167)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463367360854418161167n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_uint128_euint128(
      340282366920938463463370684765186274371n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint128, euint128) => ebool test 2 (340282366920938463463369660429237688661, 340282366920938463463369660429237688665)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463369660429237688665n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_uint128_euint128(
      340282366920938463463369660429237688661n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint128, euint128) => ebool test 3 (340282366920938463463369660429237688665, 340282366920938463463369660429237688665)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463369660429237688665n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_uint128_euint128(
      340282366920938463463369660429237688665n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint128, euint128) => ebool test 4 (340282366920938463463369660429237688665, 340282366920938463463369660429237688661)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463369660429237688661n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_uint128_euint128(
      340282366920938463463369660429237688665n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, uint128) => ebool test 1 (340282366920938463463373573707769836885, 340282366920938463463373123045235078483)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463373573707769836885n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463373123045235078483n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, uint128) => ebool test 2 (340282366920938463463369848690637762897, 340282366920938463463369848690637762901)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463369848690637762897n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463369848690637762901n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, uint128) => ebool test 3 (340282366920938463463369848690637762901, 340282366920938463463369848690637762901)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463369848690637762901n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463369848690637762901n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, uint128) => ebool test 4 (340282366920938463463369848690637762901, 340282366920938463463369848690637762897)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463369848690637762901n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463369848690637762897n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint128, euint128) => ebool test 1 (340282366920938463463366583772199851309, 340282366920938463463373123045235078483)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463373123045235078483n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_uint128_euint128(
      340282366920938463463366583772199851309n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint128, euint128) => ebool test 2 (340282366920938463463369848690637762897, 340282366920938463463369848690637762901)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463369848690637762901n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_uint128_euint128(
      340282366920938463463369848690637762897n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint128, euint128) => ebool test 3 (340282366920938463463369848690637762901, 340282366920938463463369848690637762901)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463369848690637762901n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_uint128_euint128(
      340282366920938463463369848690637762901n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint128, euint128) => ebool test 4 (340282366920938463463369848690637762901, 340282366920938463463369848690637762897)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463369848690637762897n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_uint128_euint128(
      340282366920938463463369848690637762901n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, uint128) => ebool test 1 (340282366920938463463366979368712504533, 340282366920938463463373444004260042149)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463366979368712504533n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463373444004260042149n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint128, uint128) => ebool test 2 (340282366920938463463366979368712504529, 340282366920938463463366979368712504533)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463366979368712504529n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463366979368712504533n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint128, uint128) => ebool test 3 (340282366920938463463366979368712504533, 340282366920938463463366979368712504533)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463366979368712504533n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463366979368712504533n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, uint128) => ebool test 4 (340282366920938463463366979368712504533, 340282366920938463463366979368712504529)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463366979368712504533n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463366979368712504529n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint128, euint128) => ebool test 1 (340282366920938463463373124388421219561, 340282366920938463463373444004260042149)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463373444004260042149n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_uint128_euint128(
      340282366920938463463373124388421219561n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint128, euint128) => ebool test 2 (340282366920938463463366979368712504529, 340282366920938463463366979368712504533)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463366979368712504533n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_uint128_euint128(
      340282366920938463463366979368712504529n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint128, euint128) => ebool test 3 (340282366920938463463366979368712504533, 340282366920938463463366979368712504533)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463366979368712504533n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_uint128_euint128(
      340282366920938463463366979368712504533n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint128, euint128) => ebool test 4 (340282366920938463463366979368712504533, 340282366920938463463366979368712504529)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463366979368712504529n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_uint128_euint128(
      340282366920938463463366979368712504533n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, uint128) => ebool test 1 (340282366920938463463370995022472569787, 340282366920938463463372128341737580261)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463370995022472569787n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463372128341737580261n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, uint128) => ebool test 2 (340282366920938463463368896639485130251, 340282366920938463463368896639485130255)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463368896639485130251n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368896639485130255n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, uint128) => ebool test 3 (340282366920938463463368896639485130255, 340282366920938463463368896639485130255)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463368896639485130255n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368896639485130255n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, uint128) => ebool test 4 (340282366920938463463368896639485130255, 340282366920938463463368896639485130251)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463368896639485130255n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368896639485130251n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint128, euint128) => ebool test 1 (340282366920938463463372558172017318195, 340282366920938463463372128341737580261)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463372128341737580261n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_uint128_euint128(
      340282366920938463463372558172017318195n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint128, euint128) => ebool test 2 (340282366920938463463368896639485130251, 340282366920938463463368896639485130255)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463368896639485130255n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_uint128_euint128(
      340282366920938463463368896639485130251n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint128, euint128) => ebool test 3 (340282366920938463463368896639485130255, 340282366920938463463368896639485130255)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463368896639485130255n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_uint128_euint128(
      340282366920938463463368896639485130255n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint128, euint128) => ebool test 4 (340282366920938463463368896639485130255, 340282366920938463463368896639485130251)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463368896639485130251n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_uint128_euint128(
      340282366920938463463368896639485130255n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, uint128) => ebool test 1 (340282366920938463463371623229046350155, 340282366920938463463366532608153244445)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463371623229046350155n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463366532608153244445n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint128, uint128) => ebool test 2 (340282366920938463463371623229046350151, 340282366920938463463371623229046350155)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463371623229046350151n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463371623229046350155n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, uint128) => ebool test 3 (340282366920938463463371623229046350155, 340282366920938463463371623229046350155)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463371623229046350155n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463371623229046350155n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, uint128) => ebool test 4 (340282366920938463463371623229046350155, 340282366920938463463371623229046350151)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463371623229046350155n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463371623229046350151n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint128, euint128) => ebool test 1 (340282366920938463463373060319067001073, 340282366920938463463366532608153244445)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463366532608153244445n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_uint128_euint128(
      340282366920938463463373060319067001073n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint128, euint128) => ebool test 2 (340282366920938463463371623229046350151, 340282366920938463463371623229046350155)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463371623229046350155n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_uint128_euint128(
      340282366920938463463371623229046350151n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint128, euint128) => ebool test 3 (340282366920938463463371623229046350155, 340282366920938463463371623229046350155)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463371623229046350155n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_uint128_euint128(
      340282366920938463463371623229046350155n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint128, euint128) => ebool test 4 (340282366920938463463371623229046350155, 340282366920938463463371623229046350151)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463371623229046350151n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_uint128_euint128(
      340282366920938463463371623229046350155n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, uint128) => ebool test 1 (340282366920938463463371362312241676061, 340282366920938463463371136402842576865)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463371362312241676061n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463371136402842576865n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, uint128) => ebool test 2 (340282366920938463463368735450827730501, 340282366920938463463368735450827730505)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463368735450827730501n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368735450827730505n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint128, uint128) => ebool test 3 (340282366920938463463368735450827730505, 340282366920938463463368735450827730505)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463368735450827730505n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368735450827730505n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, uint128) => ebool test 4 (340282366920938463463368735450827730505, 340282366920938463463368735450827730501)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463368735450827730505n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368735450827730501n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint128, euint128) => ebool test 1 (340282366920938463463367571989706462091, 340282366920938463463371136402842576865)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463371136402842576865n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_uint128_euint128(
      340282366920938463463367571989706462091n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint128, euint128) => ebool test 2 (340282366920938463463368735450827730501, 340282366920938463463368735450827730505)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463368735450827730505n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_uint128_euint128(
      340282366920938463463368735450827730501n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint128, euint128) => ebool test 3 (340282366920938463463368735450827730505, 340282366920938463463368735450827730505)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463368735450827730505n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_uint128_euint128(
      340282366920938463463368735450827730505n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint128, euint128) => ebool test 4 (340282366920938463463368735450827730505, 340282366920938463463368735450827730501)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463368735450827730501n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_uint128_euint128(
      340282366920938463463368735450827730505n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint128, uint128) => euint128 test 1 (340282366920938463463367333886528321945, 340282366920938463463374173656349611801)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463367333886528321945n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463374173656349611801n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463463367333886528321945n);
  });

  it('test operator "min" overload (euint128, uint128) => euint128 test 2 (340282366920938463463367333886528321941, 340282366920938463463367333886528321945)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463367333886528321941n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463367333886528321945n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463463367333886528321941n);
  });

  it('test operator "min" overload (euint128, uint128) => euint128 test 3 (340282366920938463463367333886528321945, 340282366920938463463367333886528321945)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463367333886528321945n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463367333886528321945n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463463367333886528321945n);
  });

  it('test operator "min" overload (euint128, uint128) => euint128 test 4 (340282366920938463463367333886528321945, 340282366920938463463367333886528321941)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463367333886528321945n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463367333886528321941n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463463367333886528321941n);
  });

  it('test operator "min" overload (uint128, euint128) => euint128 test 1 (340282366920938463463371219404295456265, 340282366920938463463374173656349611801)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463374173656349611801n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_uint128_euint128(
      340282366920938463463371219404295456265n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463463371219404295456265n);
  });

  it('test operator "min" overload (uint128, euint128) => euint128 test 2 (340282366920938463463367333886528321941, 340282366920938463463367333886528321945)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463367333886528321945n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_uint128_euint128(
      340282366920938463463367333886528321941n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463463367333886528321941n);
  });

  it('test operator "min" overload (uint128, euint128) => euint128 test 3 (340282366920938463463367333886528321945, 340282366920938463463367333886528321945)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463367333886528321945n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_uint128_euint128(
      340282366920938463463367333886528321945n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463463367333886528321945n);
  });

  it('test operator "min" overload (uint128, euint128) => euint128 test 4 (340282366920938463463367333886528321945, 340282366920938463463367333886528321941)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463367333886528321941n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_uint128_euint128(
      340282366920938463463367333886528321945n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463463367333886528321941n);
  });

  it('test operator "max" overload (euint128, uint128) => euint128 test 1 (340282366920938463463368599296840715501, 340282366920938463463367507600221459439)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463368599296840715501n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463367507600221459439n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463463368599296840715501n);
  });

  it('test operator "max" overload (euint128, uint128) => euint128 test 2 (340282366920938463463368599296840715497, 340282366920938463463368599296840715501)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463368599296840715497n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368599296840715501n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463463368599296840715501n);
  });

  it('test operator "max" overload (euint128, uint128) => euint128 test 3 (340282366920938463463368599296840715501, 340282366920938463463368599296840715501)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463368599296840715501n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368599296840715501n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463463368599296840715501n);
  });

  it('test operator "max" overload (euint128, uint128) => euint128 test 4 (340282366920938463463368599296840715501, 340282366920938463463368599296840715497)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463368599296840715501n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368599296840715497n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463463368599296840715501n);
  });

  it('test operator "max" overload (uint128, euint128) => euint128 test 1 (340282366920938463463369067110441266547, 340282366920938463463367507600221459439)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463367507600221459439n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_uint128_euint128(
      340282366920938463463369067110441266547n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463463369067110441266547n);
  });

  it('test operator "max" overload (uint128, euint128) => euint128 test 2 (340282366920938463463368599296840715497, 340282366920938463463368599296840715501)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463368599296840715501n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_uint128_euint128(
      340282366920938463463368599296840715497n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463463368599296840715501n);
  });

  it('test operator "max" overload (uint128, euint128) => euint128 test 3 (340282366920938463463368599296840715501, 340282366920938463463368599296840715501)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463368599296840715501n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_uint128_euint128(
      340282366920938463463368599296840715501n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463463368599296840715501n);
  });

  it('test operator "max" overload (uint128, euint128) => euint128 test 4 (340282366920938463463368599296840715501, 340282366920938463463368599296840715497)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463368599296840715497n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_uint128_euint128(
      340282366920938463463368599296840715501n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463463368599296840715501n);
  });

  it('test operator "and" overload (euint256, uint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457583476569507143393, 115792089237316195423570985008687907853269984665640564039457577848464592316595)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457583476569507143393n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457577848464592316595n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457577846261244004513n);
  });

  it('test operator "and" overload (euint256, uint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457576398624363873275, 115792089237316195423570985008687907853269984665640564039457576398624363873279)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576398624363873275n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457576398624363873279n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457576398624363873275n);
  });

  it('test operator "and" overload (euint256, uint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457576398624363873279, 115792089237316195423570985008687907853269984665640564039457576398624363873279)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576398624363873279n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457576398624363873279n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457576398624363873279n);
  });

  it('test operator "and" overload (euint256, uint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457576398624363873279, 115792089237316195423570985008687907853269984665640564039457576398624363873275)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576398624363873279n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457576398624363873275n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457576398624363873275n);
  });

  it('test operator "and" overload (uint256, euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457582612568110211261, 115792089237316195423570985008687907853269984665640564039457577848464592316595)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457577848464592316595n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457582612568110211261n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457577826461472202929n);
  });

  it('test operator "and" overload (uint256, euint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457576398624363873275, 115792089237316195423570985008687907853269984665640564039457576398624363873279)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457576398624363873279n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457576398624363873275n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457576398624363873275n);
  });

  it('test operator "and" overload (uint256, euint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457576398624363873279, 115792089237316195423570985008687907853269984665640564039457576398624363873279)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457576398624363873279n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457576398624363873279n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457576398624363873279n);
  });

  it('test operator "and" overload (uint256, euint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457576398624363873279, 115792089237316195423570985008687907853269984665640564039457576398624363873275)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457576398624363873275n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457576398624363873279n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457576398624363873275n);
  });

  it('test operator "or" overload (euint256, uint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457576120721729938827, 115792089237316195423570985008687907853269984665640564039457582315989680531493)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576120721729938827n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457582315989680531493n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457582881688950435247n);
  });

  it('test operator "or" overload (euint256, uint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457576120721729938823, 115792089237316195423570985008687907853269984665640564039457576120721729938827)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576120721729938823n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457576120721729938827n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457576120721729938831n);
  });

  it('test operator "or" overload (euint256, uint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457576120721729938827, 115792089237316195423570985008687907853269984665640564039457576120721729938827)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576120721729938827n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457576120721729938827n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457576120721729938827n);
  });

  it('test operator "or" overload (euint256, uint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457576120721729938827, 115792089237316195423570985008687907853269984665640564039457576120721729938823)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576120721729938827n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457576120721729938823n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457576120721729938831n);
  });

  it('test operator "or" overload (uint256, euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457579199021958158311, 115792089237316195423570985008687907853269984665640564039457582315989680531493)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457582315989680531493n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457579199021958158311n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457584007324181594087n);
  });

  it('test operator "or" overload (uint256, euint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457576120721729938823, 115792089237316195423570985008687907853269984665640564039457576120721729938827)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457576120721729938827n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457576120721729938823n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457576120721729938831n);
  });

  it('test operator "or" overload (uint256, euint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457576120721729938827, 115792089237316195423570985008687907853269984665640564039457576120721729938827)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457576120721729938827n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457576120721729938827n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457576120721729938827n);
  });

  it('test operator "or" overload (uint256, euint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457576120721729938827, 115792089237316195423570985008687907853269984665640564039457576120721729938823)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457576120721729938823n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457576120721729938827n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457576120721729938831n);
  });

  it('test operator "xor" overload (euint256, uint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457582474364217937289, 115792089237316195423570985008687907853269984665640564039457578931092473587871)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582474364217937289n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457578931092473587871n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(6609809812645142n);
  });

  it('test operator "xor" overload (euint256, uint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457582474364217937285, 115792089237316195423570985008687907853269984665640564039457582474364217937289)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582474364217937285n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457582474364217937289n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint256, uint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457582474364217937289, 115792089237316195423570985008687907853269984665640564039457582474364217937289)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582474364217937289n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457582474364217937289n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint256, uint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457582474364217937289, 115792089237316195423570985008687907853269984665640564039457582474364217937285)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582474364217937289n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457582474364217937285n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (uint256, euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457580799631751342559, 115792089237316195423570985008687907853269984665640564039457578931092473587871)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457578931092473587871n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457580799631751342559n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(7156434968920384n);
  });

  it('test operator "xor" overload (uint256, euint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457582474364217937285, 115792089237316195423570985008687907853269984665640564039457582474364217937289)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457582474364217937289n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457582474364217937285n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (uint256, euint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457582474364217937289, 115792089237316195423570985008687907853269984665640564039457582474364217937289)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457582474364217937289n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457582474364217937289n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (uint256, euint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457582474364217937289, 115792089237316195423570985008687907853269984665640564039457582474364217937285)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457582474364217937285n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457582474364217937289n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint256, uint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457581509794520384545, 115792089237316195423570985008687907853269984665640564039457581365157143701111)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457581509794520384545n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457581365157143701111n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, uint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457581509794520384541, 115792089237316195423570985008687907853269984665640564039457581509794520384545)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457581509794520384541n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457581509794520384545n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, uint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457581509794520384545, 115792089237316195423570985008687907853269984665640564039457581509794520384545)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457581509794520384545n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457581509794520384545n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint256, uint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457581509794520384545, 115792089237316195423570985008687907853269984665640564039457581509794520384541)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457581509794520384545n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457581509794520384541n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint256, euint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457578915462108280733, 115792089237316195423570985008687907853269984665640564039457581365157143701111)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457581365157143701111n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457578915462108280733n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint256, euint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457581509794520384541, 115792089237316195423570985008687907853269984665640564039457581509794520384545)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457581509794520384545n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457581509794520384541n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint256, euint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457581509794520384545, 115792089237316195423570985008687907853269984665640564039457581509794520384545)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457581509794520384545n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457581509794520384545n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint256, euint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457581509794520384545, 115792089237316195423570985008687907853269984665640564039457581509794520384541)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457581509794520384541n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457581509794520384545n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, uint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457584002695830945561, 115792089237316195423570985008687907853269984665640564039457583776225039918185)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457584002695830945561n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457583776225039918185n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, uint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457580644856625898091, 115792089237316195423570985008687907853269984665640564039457580644856625898095)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580644856625898091n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457580644856625898095n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, uint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457580644856625898095, 115792089237316195423570985008687907853269984665640564039457580644856625898095)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580644856625898095n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457580644856625898095n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, uint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457580644856625898095, 115792089237316195423570985008687907853269984665640564039457580644856625898091)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580644856625898095n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457580644856625898091n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint256, euint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457583032914471443181, 115792089237316195423570985008687907853269984665640564039457583776225039918185)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457583776225039918185n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457583032914471443181n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint256, euint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457580644856625898091, 115792089237316195423570985008687907853269984665640564039457580644856625898095)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457580644856625898095n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457580644856625898091n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint256, euint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457580644856625898095, 115792089237316195423570985008687907853269984665640564039457580644856625898095)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457580644856625898095n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457580644856625898095n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint256, euint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457580644856625898095, 115792089237316195423570985008687907853269984665640564039457580644856625898091)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457580644856625898091n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457580644856625898095n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "shl" overload (euint8, euint8) => euint8 test 1 (103, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(103n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(103n);
  });

  it('test operator "shl" overload (euint8, euint8) => euint8 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(4n);
  });

  it('test operator "shl" overload (euint8, euint8) => euint8 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(8n);
  });

  it('test operator "shl" overload (euint8, euint8) => euint8 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(128n);
  });

  it('test operator "shl" overload (euint8, uint8) => euint8 test 1 (103, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(103n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint8_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(103n);
  });

  it('test operator "shl" overload (euint8, uint8) => euint8 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint8_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(4n);
  });

  it('test operator "shl" overload (euint8, uint8) => euint8 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint8_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(8n);
  });

  it('test operator "shl" overload (euint8, uint8) => euint8 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint8_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(128n);
  });

  it('test operator "shr" overload (euint8, euint8) => euint8 test 1 (170, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(170n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(2n);
  });

  it('test operator "shr" overload (euint8, euint8) => euint8 test 2 (2, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(2n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint8, euint8) => euint8 test 3 (6, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(6n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint8, euint8) => euint8 test 4 (6, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(6n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(1n);
  });

  it('test operator "shr" overload (euint8, uint8) => euint8 test 1 (170, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(170n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint8_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(2n);
  });

  it('test operator "shr" overload (euint8, uint8) => euint8 test 2 (2, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(2n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint8_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint8, uint8) => euint8 test 3 (6, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(6n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint8_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint8, uint8) => euint8 test 4 (6, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(6n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint8_uint8(encryptedAmount.handles[0], 2n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(1n);
  });

  it('test operator "rotl" overload (euint8, euint8) => euint8 test 1 (128, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(128n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(32n);
  });

  it('test operator "rotl" overload (euint8, euint8) => euint8 test 2 (2, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(2n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(128n);
  });

  it('test operator "rotl" overload (euint8, euint8) => euint8 test 3 (6, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(6n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(129n);
  });

  it('test operator "rotl" overload (euint8, euint8) => euint8 test 4 (6, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(6n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(24n);
  });

  it('test operator "rotl" overload (euint8, uint8) => euint8 test 1 (128, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(128n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint8_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(32n);
  });

  it('test operator "rotl" overload (euint8, uint8) => euint8 test 2 (2, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(2n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint8_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(128n);
  });

  it('test operator "rotl" overload (euint8, uint8) => euint8 test 3 (6, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(6n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint8_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(129n);
  });

  it('test operator "rotl" overload (euint8, uint8) => euint8 test 4 (6, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(6n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint8_uint8(encryptedAmount.handles[0], 2n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(24n);
  });

  it('test operator "rotr" overload (euint8, euint8) => euint8 test 1 (43, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(43n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(89n);
  });

  it('test operator "rotr" overload (euint8, euint8) => euint8 test 2 (1, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(1n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(8n);
  });

  it('test operator "rotr" overload (euint8, euint8) => euint8 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(5n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(40n);
  });

  it('test operator "rotr" overload (euint8, euint8) => euint8 test 4 (5, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(5n);
    input.add8(1n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(130n);
  });
});
