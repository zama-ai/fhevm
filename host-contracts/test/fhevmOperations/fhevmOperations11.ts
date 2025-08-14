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

  it('test operator "lt" overload (uint64, euint64) => ebool test 1 (18441280626911569051, 18440132501482413511)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18440132501482413511n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint64_euint64(
      18441280626911569051n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint64, euint64) => ebool test 2 (18440347090926676087, 18440347090926676091)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18440347090926676091n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint64_euint64(
      18440347090926676087n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint64, euint64) => ebool test 3 (18440347090926676091, 18440347090926676091)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18440347090926676091n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint64_euint64(
      18440347090926676091n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint64, euint64) => ebool test 4 (18440347090926676091, 18440347090926676087)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18440347090926676087n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint64_euint64(
      18440347090926676091n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, uint64) => euint64 test 1 (18443402349954406303, 18440601998930672295)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18443402349954406303n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint64_uint64(
      encryptedAmount.handles[0],
      18440601998930672295n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18440601998930672295n);
  });

  it('test operator "min" overload (euint64, uint64) => euint64 test 2 (18443402349954406299, 18443402349954406303)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18443402349954406299n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint64_uint64(
      encryptedAmount.handles[0],
      18443402349954406303n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18443402349954406299n);
  });

  it('test operator "min" overload (euint64, uint64) => euint64 test 3 (18443402349954406303, 18443402349954406303)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18443402349954406303n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint64_uint64(
      encryptedAmount.handles[0],
      18443402349954406303n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18443402349954406303n);
  });

  it('test operator "min" overload (euint64, uint64) => euint64 test 4 (18443402349954406303, 18443402349954406299)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18443402349954406303n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint64_uint64(
      encryptedAmount.handles[0],
      18443402349954406299n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18443402349954406299n);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 1 (18442140157362967135, 18440601998930672295)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18440601998930672295n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint64_euint64(
      18442140157362967135n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18440601998930672295n);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 2 (18443402349954406299, 18443402349954406303)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18443402349954406303n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint64_euint64(
      18443402349954406299n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18443402349954406299n);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 3 (18443402349954406303, 18443402349954406303)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18443402349954406303n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint64_euint64(
      18443402349954406303n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18443402349954406303n);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 4 (18443402349954406303, 18443402349954406299)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18443402349954406299n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint64_euint64(
      18443402349954406303n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18443402349954406299n);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 1 (18441966199340663907, 18440116110580220957)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18441966199340663907n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint64_uint64(
      encryptedAmount.handles[0],
      18440116110580220957n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18441966199340663907n);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 2 (18441535310484629161, 18441535310484629165)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18441535310484629161n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint64_uint64(
      encryptedAmount.handles[0],
      18441535310484629165n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18441535310484629165n);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 3 (18441535310484629165, 18441535310484629165)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18441535310484629165n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint64_uint64(
      encryptedAmount.handles[0],
      18441535310484629165n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18441535310484629165n);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 4 (18441535310484629165, 18441535310484629161)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18441535310484629165n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint64_uint64(
      encryptedAmount.handles[0],
      18441535310484629161n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18441535310484629165n);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 1 (18444080979892586495, 18440116110580220957)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18440116110580220957n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint64_euint64(
      18444080979892586495n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18444080979892586495n);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 2 (18441535310484629161, 18441535310484629165)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18441535310484629165n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint64_euint64(
      18441535310484629161n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18441535310484629165n);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 3 (18441535310484629165, 18441535310484629165)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18441535310484629165n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint64_euint64(
      18441535310484629165n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18441535310484629165n);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 4 (18441535310484629165, 18441535310484629161)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18441535310484629161n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint64_euint64(
      18441535310484629165n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18441535310484629165n);
  });

  it('test operator "add" overload (euint128, uint128) => euint128 test 1 (170141183460469231731684105186990084698, 170141183460469231731684963517882830842)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(170141183460469231731684105186990084698n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint128_uint128(
      encryptedAmount.handles[0],
      170141183460469231731684963517882830842n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463369068704872915540n);
  });

  it('test operator "add" overload (euint128, uint128) => euint128 test 2 (170141183460469231731684105186990084696, 170141183460469231731684105186990084698)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(170141183460469231731684105186990084696n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint128_uint128(
      encryptedAmount.handles[0],
      170141183460469231731684105186990084698n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463368210373980169394n);
  });

  it('test operator "add" overload (euint128, uint128) => euint128 test 3 (170141183460469231731684105186990084698, 170141183460469231731684105186990084698)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(170141183460469231731684105186990084698n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint128_uint128(
      encryptedAmount.handles[0],
      170141183460469231731684105186990084698n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463368210373980169396n);
  });

  it('test operator "add" overload (euint128, uint128) => euint128 test 4 (170141183460469231731684105186990084698, 170141183460469231731684105186990084696)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(170141183460469231731684105186990084698n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint128_uint128(
      encryptedAmount.handles[0],
      170141183460469231731684105186990084696n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463368210373980169394n);
  });

  it('test operator "add" overload (uint128, euint128) => euint128 test 1 (170141183460469231731685421703267829646, 170141183460469231731684963517882830842)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(170141183460469231731684963517882830842n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_uint128_euint128(
      170141183460469231731685421703267829646n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463370385221150660488n);
  });

  it('test operator "add" overload (uint128, euint128) => euint128 test 2 (170141183460469231731684105186990084696, 170141183460469231731684105186990084698)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(170141183460469231731684105186990084698n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_uint128_euint128(
      170141183460469231731684105186990084696n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463368210373980169394n);
  });

  it('test operator "add" overload (uint128, euint128) => euint128 test 3 (170141183460469231731684105186990084698, 170141183460469231731684105186990084698)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(170141183460469231731684105186990084698n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_uint128_euint128(
      170141183460469231731684105186990084698n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463368210373980169396n);
  });

  it('test operator "add" overload (uint128, euint128) => euint128 test 4 (170141183460469231731684105186990084698, 170141183460469231731684105186990084696)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(170141183460469231731684105186990084696n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_uint128_euint128(
      170141183460469231731684105186990084698n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463368210373980169394n);
  });

  it('test operator "sub" overload (euint128, uint128) => euint128 test 1 (340282366920938463463370774111204197771, 340282366920938463463370774111204197771)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463370774111204197771n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463370774111204197771n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint128, uint128) => euint128 test 2 (340282366920938463463370774111204197771, 340282366920938463463370774111204197767)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463370774111204197771n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463370774111204197767n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "sub" overload (uint128, euint128) => euint128 test 1 (340282366920938463463370774111204197771, 340282366920938463463370774111204197771)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(340282366920938463463370774111204197771n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_uint128_euint128(
      340282366920938463463370774111204197771n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (uint128, euint128) => euint128 test 2 (340282366920938463463370774111204197771, 340282366920938463463370774111204197767)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(340282366920938463463370774111204197767n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_uint128_euint128(
      340282366920938463463370774111204197771n,
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

  it('test operator "div" overload (euint128, uint128) => euint128 test 1 (340282366920938463463369117976452982153, 340282366920938463463370470319835567325)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463369117976452982153n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.div_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463370470319835567325n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint128, uint128) => euint128 test 2 (340282366920938463463369117976452982149, 340282366920938463463369117976452982153)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463369117976452982149n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.div_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463369117976452982153n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint128, uint128) => euint128 test 3 (340282366920938463463369117976452982153, 340282366920938463463369117976452982153)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463369117976452982153n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.div_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463369117976452982153n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint128, uint128) => euint128 test 4 (340282366920938463463369117976452982153, 340282366920938463463369117976452982149)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463369117976452982153n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.div_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463369117976452982149n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(1n);
  });

  it('test operator "rem" overload (euint128, uint128) => euint128 test 1 (340282366920938463463368402451423437941, 340282366920938463463366767492925237721)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463368402451423437941n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rem_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463366767492925237721n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(1634958498200220n);
  });

  it('test operator "rem" overload (euint128, uint128) => euint128 test 2 (340282366920938463463368402451423437937, 340282366920938463463368402451423437941)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463368402451423437937n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rem_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368402451423437941n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463368402451423437937n);
  });

  it('test operator "rem" overload (euint128, uint128) => euint128 test 3 (340282366920938463463368402451423437941, 340282366920938463463368402451423437941)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463368402451423437941n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rem_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368402451423437941n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "rem" overload (euint128, uint128) => euint128 test 4 (340282366920938463463368402451423437941, 340282366920938463463368402451423437937)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463368402451423437941n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rem_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368402451423437937n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "and" overload (euint128, uint128) => euint128 test 1 (340282366920938463463370390251715017337, 340282366920938463463374525640866674839)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463370390251715017337n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463374525640866674839n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463370389825428928529n);
  });

  it('test operator "and" overload (euint128, uint128) => euint128 test 2 (340282366920938463463367745331704529825, 340282366920938463463367745331704529829)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463367745331704529825n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463367745331704529829n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463367745331704529825n);
  });

  it('test operator "and" overload (euint128, uint128) => euint128 test 3 (340282366920938463463367745331704529829, 340282366920938463463367745331704529829)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463367745331704529829n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463367745331704529829n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463367745331704529829n);
  });

  it('test operator "and" overload (euint128, uint128) => euint128 test 4 (340282366920938463463367745331704529829, 340282366920938463463367745331704529825)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463367745331704529829n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463367745331704529825n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463367745331704529825n);
  });

  it('test operator "and" overload (uint128, euint128) => euint128 test 1 (340282366920938463463365612943205062219, 340282366920938463463374525640866674839)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(340282366920938463463374525640866674839n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_uint128_euint128(
      340282366920938463463365612943205062219n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463365601933593153539n);
  });

  it('test operator "and" overload (uint128, euint128) => euint128 test 2 (340282366920938463463367745331704529825, 340282366920938463463367745331704529829)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(340282366920938463463367745331704529829n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_uint128_euint128(
      340282366920938463463367745331704529825n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463367745331704529825n);
  });

  it('test operator "and" overload (uint128, euint128) => euint128 test 3 (340282366920938463463367745331704529829, 340282366920938463463367745331704529829)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(340282366920938463463367745331704529829n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_uint128_euint128(
      340282366920938463463367745331704529829n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463367745331704529829n);
  });

  it('test operator "and" overload (uint128, euint128) => euint128 test 4 (340282366920938463463367745331704529829, 340282366920938463463367745331704529825)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(340282366920938463463367745331704529825n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_uint128_euint128(
      340282366920938463463367745331704529829n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463367745331704529825n);
  });

  it('test operator "or" overload (euint128, uint128) => euint128 test 1 (340282366920938463463366190813548079741, 340282366920938463463365778971860473741)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463366190813548079741n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463365778971860473741n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463366369547052351485n);
  });

  it('test operator "or" overload (euint128, uint128) => euint128 test 2 (340282366920938463463366190813548079737, 340282366920938463463366190813548079741)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463366190813548079737n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463366190813548079741n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463366190813548079741n);
  });

  it('test operator "or" overload (euint128, uint128) => euint128 test 3 (340282366920938463463366190813548079741, 340282366920938463463366190813548079741)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463366190813548079741n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463366190813548079741n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463366190813548079741n);
  });

  it('test operator "or" overload (euint128, uint128) => euint128 test 4 (340282366920938463463366190813548079741, 340282366920938463463366190813548079737)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463366190813548079741n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463366190813548079737n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463366190813548079741n);
  });

  it('test operator "or" overload (uint128, euint128) => euint128 test 1 (340282366920938463463371726488250142633, 340282366920938463463365778971860473741)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(340282366920938463463365778971860473741n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_uint128_euint128(
      340282366920938463463371726488250142633n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463371761689516957613n);
  });

  it('test operator "or" overload (uint128, euint128) => euint128 test 2 (340282366920938463463366190813548079737, 340282366920938463463366190813548079741)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(340282366920938463463366190813548079741n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_uint128_euint128(
      340282366920938463463366190813548079737n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463366190813548079741n);
  });

  it('test operator "or" overload (uint128, euint128) => euint128 test 3 (340282366920938463463366190813548079741, 340282366920938463463366190813548079741)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(340282366920938463463366190813548079741n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_uint128_euint128(
      340282366920938463463366190813548079741n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463366190813548079741n);
  });

  it('test operator "or" overload (uint128, euint128) => euint128 test 4 (340282366920938463463366190813548079741, 340282366920938463463366190813548079737)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(340282366920938463463366190813548079737n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_uint128_euint128(
      340282366920938463463366190813548079741n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463366190813548079741n);
  });

  it('test operator "xor" overload (euint128, uint128) => euint128 test 1 (340282366920938463463368661624087597999, 340282366920938463463370825313681378613)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463368661624087597999n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463370825313681378613n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(6879089258716826n);
  });

  it('test operator "xor" overload (euint128, uint128) => euint128 test 2 (340282366920938463463368661624087597995, 340282366920938463463368661624087597999)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463368661624087597995n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368661624087597999n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint128, uint128) => euint128 test 3 (340282366920938463463368661624087597999, 340282366920938463463368661624087597999)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463368661624087597999n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368661624087597999n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint128, uint128) => euint128 test 4 (340282366920938463463368661624087597999, 340282366920938463463368661624087597995)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463368661624087597999n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368661624087597995n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (uint128, euint128) => euint128 test 1 (340282366920938463463371824732038331069, 340282366920938463463370825313681378613)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(340282366920938463463370825313681378613n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_uint128_euint128(
      340282366920938463463371824732038331069n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(1280970673813384n);
  });

  it('test operator "xor" overload (uint128, euint128) => euint128 test 2 (340282366920938463463368661624087597995, 340282366920938463463368661624087597999)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(340282366920938463463368661624087597999n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_uint128_euint128(
      340282366920938463463368661624087597995n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (uint128, euint128) => euint128 test 3 (340282366920938463463368661624087597999, 340282366920938463463368661624087597999)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(340282366920938463463368661624087597999n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_uint128_euint128(
      340282366920938463463368661624087597999n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (uint128, euint128) => euint128 test 4 (340282366920938463463368661624087597999, 340282366920938463463368661624087597995)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(340282366920938463463368661624087597995n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_uint128_euint128(
      340282366920938463463368661624087597999n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint128, uint128) => ebool test 1 (340282366920938463463370970969341621945, 340282366920938463463372266183726609655)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463370970969341621945n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463372266183726609655n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, uint128) => ebool test 2 (340282366920938463463370970969341621941, 340282366920938463463370970969341621945)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463370970969341621941n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463370970969341621945n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, uint128) => ebool test 3 (340282366920938463463370970969341621945, 340282366920938463463370970969341621945)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463370970969341621945n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463370970969341621945n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint128, uint128) => ebool test 4 (340282366920938463463370970969341621945, 340282366920938463463370970969341621941)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463370970969341621945n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463370970969341621941n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint128, euint128) => ebool test 1 (340282366920938463463366954421098389827, 340282366920938463463372266183726609655)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463372266183726609655n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_uint128_euint128(
      340282366920938463463366954421098389827n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint128, euint128) => ebool test 2 (340282366920938463463370970969341621941, 340282366920938463463370970969341621945)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463370970969341621945n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_uint128_euint128(
      340282366920938463463370970969341621941n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint128, euint128) => ebool test 3 (340282366920938463463370970969341621945, 340282366920938463463370970969341621945)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463370970969341621945n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_uint128_euint128(
      340282366920938463463370970969341621945n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint128, euint128) => ebool test 4 (340282366920938463463370970969341621945, 340282366920938463463370970969341621941)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463370970969341621941n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_uint128_euint128(
      340282366920938463463370970969341621945n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, uint128) => ebool test 1 (340282366920938463463371497876445088367, 340282366920938463463371646757800044601)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463371497876445088367n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463371646757800044601n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, uint128) => ebool test 2 (340282366920938463463369649532840989793, 340282366920938463463369649532840989797)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463369649532840989793n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463369649532840989797n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, uint128) => ebool test 3 (340282366920938463463369649532840989797, 340282366920938463463369649532840989797)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463369649532840989797n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463369649532840989797n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, uint128) => ebool test 4 (340282366920938463463369649532840989797, 340282366920938463463369649532840989793)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463369649532840989797n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463369649532840989793n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint128, euint128) => ebool test 1 (340282366920938463463374308344404434773, 340282366920938463463371646757800044601)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463371646757800044601n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_uint128_euint128(
      340282366920938463463374308344404434773n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint128, euint128) => ebool test 2 (340282366920938463463369649532840989793, 340282366920938463463369649532840989797)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463369649532840989797n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_uint128_euint128(
      340282366920938463463369649532840989793n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint128, euint128) => ebool test 3 (340282366920938463463369649532840989797, 340282366920938463463369649532840989797)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463369649532840989797n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_uint128_euint128(
      340282366920938463463369649532840989797n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint128, euint128) => ebool test 4 (340282366920938463463369649532840989797, 340282366920938463463369649532840989793)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463369649532840989793n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_uint128_euint128(
      340282366920938463463369649532840989797n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, uint128) => ebool test 1 (340282366920938463463368430429229686369, 340282366920938463463371699194007222199)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463368430429229686369n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463371699194007222199n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint128, uint128) => ebool test 2 (340282366920938463463368430429229686365, 340282366920938463463368430429229686369)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463368430429229686365n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368430429229686369n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint128, uint128) => ebool test 3 (340282366920938463463368430429229686369, 340282366920938463463368430429229686369)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463368430429229686369n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368430429229686369n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, uint128) => ebool test 4 (340282366920938463463368430429229686369, 340282366920938463463368430429229686365)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463368430429229686369n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368430429229686365n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint128, euint128) => ebool test 1 (340282366920938463463372394033341836063, 340282366920938463463371699194007222199)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463371699194007222199n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_uint128_euint128(
      340282366920938463463372394033341836063n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint128, euint128) => ebool test 2 (340282366920938463463368430429229686365, 340282366920938463463368430429229686369)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463368430429229686369n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_uint128_euint128(
      340282366920938463463368430429229686365n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint128, euint128) => ebool test 3 (340282366920938463463368430429229686369, 340282366920938463463368430429229686369)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463368430429229686369n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_uint128_euint128(
      340282366920938463463368430429229686369n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint128, euint128) => ebool test 4 (340282366920938463463368430429229686369, 340282366920938463463368430429229686365)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463368430429229686365n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_uint128_euint128(
      340282366920938463463368430429229686369n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, uint128) => ebool test 1 (340282366920938463463374524552952112439, 340282366920938463463370478701130767835)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463374524552952112439n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463370478701130767835n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, uint128) => ebool test 2 (340282366920938463463370277309125353773, 340282366920938463463370277309125353777)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463370277309125353773n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463370277309125353777n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, uint128) => ebool test 3 (340282366920938463463370277309125353777, 340282366920938463463370277309125353777)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463370277309125353777n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463370277309125353777n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, uint128) => ebool test 4 (340282366920938463463370277309125353777, 340282366920938463463370277309125353773)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463370277309125353777n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463370277309125353773n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint128, euint128) => ebool test 1 (340282366920938463463367623818400140331, 340282366920938463463370478701130767835)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463370478701130767835n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_uint128_euint128(
      340282366920938463463367623818400140331n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint128, euint128) => ebool test 2 (340282366920938463463370277309125353773, 340282366920938463463370277309125353777)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463370277309125353777n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_uint128_euint128(
      340282366920938463463370277309125353773n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint128, euint128) => ebool test 3 (340282366920938463463370277309125353777, 340282366920938463463370277309125353777)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463370277309125353777n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_uint128_euint128(
      340282366920938463463370277309125353777n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint128, euint128) => ebool test 4 (340282366920938463463370277309125353777, 340282366920938463463370277309125353773)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463370277309125353773n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_uint128_euint128(
      340282366920938463463370277309125353777n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, uint128) => ebool test 1 (340282366920938463463370613758536238185, 340282366920938463463374416819120259513)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463370613758536238185n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463374416819120259513n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, uint128) => ebool test 2 (340282366920938463463368686873965305355, 340282366920938463463368686873965305359)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463368686873965305355n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368686873965305359n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, uint128) => ebool test 3 (340282366920938463463368686873965305359, 340282366920938463463368686873965305359)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463368686873965305359n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368686873965305359n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, uint128) => ebool test 4 (340282366920938463463368686873965305359, 340282366920938463463368686873965305355)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463368686873965305359n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368686873965305355n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint128, euint128) => ebool test 1 (340282366920938463463373907837703729661, 340282366920938463463374416819120259513)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463374416819120259513n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_uint128_euint128(
      340282366920938463463373907837703729661n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint128, euint128) => ebool test 2 (340282366920938463463368686873965305355, 340282366920938463463368686873965305359)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463368686873965305359n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_uint128_euint128(
      340282366920938463463368686873965305355n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint128, euint128) => ebool test 3 (340282366920938463463368686873965305359, 340282366920938463463368686873965305359)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463368686873965305359n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_uint128_euint128(
      340282366920938463463368686873965305359n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint128, euint128) => ebool test 4 (340282366920938463463368686873965305359, 340282366920938463463368686873965305355)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463368686873965305355n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_uint128_euint128(
      340282366920938463463368686873965305359n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, uint128) => ebool test 1 (340282366920938463463372712265607771809, 340282366920938463463371823291227835029)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463372712265607771809n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463371823291227835029n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, uint128) => ebool test 2 (340282366920938463463369352327753390623, 340282366920938463463369352327753390627)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463369352327753390623n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463369352327753390627n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint128, uint128) => ebool test 3 (340282366920938463463369352327753390627, 340282366920938463463369352327753390627)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463369352327753390627n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463369352327753390627n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, uint128) => ebool test 4 (340282366920938463463369352327753390627, 340282366920938463463369352327753390623)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463369352327753390627n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463369352327753390623n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint128, euint128) => ebool test 1 (340282366920938463463370897047424887325, 340282366920938463463371823291227835029)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463371823291227835029n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_uint128_euint128(
      340282366920938463463370897047424887325n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint128, euint128) => ebool test 2 (340282366920938463463369352327753390623, 340282366920938463463369352327753390627)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463369352327753390627n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_uint128_euint128(
      340282366920938463463369352327753390623n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint128, euint128) => ebool test 3 (340282366920938463463369352327753390627, 340282366920938463463369352327753390627)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463369352327753390627n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_uint128_euint128(
      340282366920938463463369352327753390627n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint128, euint128) => ebool test 4 (340282366920938463463369352327753390627, 340282366920938463463369352327753390623)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463369352327753390623n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_uint128_euint128(
      340282366920938463463369352327753390627n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint128, uint128) => euint128 test 1 (340282366920938463463372146076470617017, 340282366920938463463367059209983720931)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463372146076470617017n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463367059209983720931n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463463367059209983720931n);
  });

  it('test operator "min" overload (euint128, uint128) => euint128 test 2 (340282366920938463463367433331826736857, 340282366920938463463367433331826736861)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463367433331826736857n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463367433331826736861n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463463367433331826736857n);
  });

  it('test operator "min" overload (euint128, uint128) => euint128 test 3 (340282366920938463463367433331826736861, 340282366920938463463367433331826736861)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463367433331826736861n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463367433331826736861n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463463367433331826736861n);
  });

  it('test operator "min" overload (euint128, uint128) => euint128 test 4 (340282366920938463463367433331826736861, 340282366920938463463367433331826736857)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463367433331826736861n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463367433331826736857n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463463367433331826736857n);
  });

  it('test operator "min" overload (uint128, euint128) => euint128 test 1 (340282366920938463463374340150471584997, 340282366920938463463367059209983720931)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463367059209983720931n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_uint128_euint128(
      340282366920938463463374340150471584997n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463463367059209983720931n);
  });

  it('test operator "min" overload (uint128, euint128) => euint128 test 2 (340282366920938463463367433331826736857, 340282366920938463463367433331826736861)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463367433331826736861n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_uint128_euint128(
      340282366920938463463367433331826736857n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463463367433331826736857n);
  });

  it('test operator "min" overload (uint128, euint128) => euint128 test 3 (340282366920938463463367433331826736861, 340282366920938463463367433331826736861)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463367433331826736861n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_uint128_euint128(
      340282366920938463463367433331826736861n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463463367433331826736861n);
  });

  it('test operator "min" overload (uint128, euint128) => euint128 test 4 (340282366920938463463367433331826736861, 340282366920938463463367433331826736857)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463367433331826736857n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_uint128_euint128(
      340282366920938463463367433331826736861n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463463367433331826736857n);
  });

  it('test operator "max" overload (euint128, uint128) => euint128 test 1 (340282366920938463463371807508804912661, 340282366920938463463365877054448462081)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463371807508804912661n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463365877054448462081n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463463371807508804912661n);
  });

  it('test operator "max" overload (euint128, uint128) => euint128 test 2 (340282366920938463463367423518088573067, 340282366920938463463367423518088573071)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463367423518088573067n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463367423518088573071n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463463367423518088573071n);
  });

  it('test operator "max" overload (euint128, uint128) => euint128 test 3 (340282366920938463463367423518088573071, 340282366920938463463367423518088573071)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463367423518088573071n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463367423518088573071n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463463367423518088573071n);
  });

  it('test operator "max" overload (euint128, uint128) => euint128 test 4 (340282366920938463463367423518088573071, 340282366920938463463367423518088573067)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463367423518088573071n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463367423518088573067n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463463367423518088573071n);
  });

  it('test operator "max" overload (uint128, euint128) => euint128 test 1 (340282366920938463463365721618185702103, 340282366920938463463365877054448462081)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463365877054448462081n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_uint128_euint128(
      340282366920938463463365721618185702103n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463463365877054448462081n);
  });

  it('test operator "max" overload (uint128, euint128) => euint128 test 2 (340282366920938463463367423518088573067, 340282366920938463463367423518088573071)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463367423518088573071n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_uint128_euint128(
      340282366920938463463367423518088573067n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463463367423518088573071n);
  });

  it('test operator "max" overload (uint128, euint128) => euint128 test 3 (340282366920938463463367423518088573071, 340282366920938463463367423518088573071)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463367423518088573071n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_uint128_euint128(
      340282366920938463463367423518088573071n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463463367423518088573071n);
  });

  it('test operator "max" overload (uint128, euint128) => euint128 test 4 (340282366920938463463367423518088573071, 340282366920938463463367423518088573067)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463367423518088573067n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_uint128_euint128(
      340282366920938463463367423518088573071n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463463367423518088573071n);
  });

  it('test operator "and" overload (euint256, uint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457581596115014982307, 115792089237316195423570985008687907853269984665640564039457581093096062478513)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457581596115014982307n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457581093096062478513n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457580951791636318369n);
  });

  it('test operator "and" overload (euint256, uint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457575241204937520531, 115792089237316195423570985008687907853269984665640564039457575241204937520535)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575241204937520531n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457575241204937520535n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457575241204937520531n);
  });

  it('test operator "and" overload (euint256, uint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457575241204937520535, 115792089237316195423570985008687907853269984665640564039457575241204937520535)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575241204937520535n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457575241204937520535n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457575241204937520535n);
  });

  it('test operator "and" overload (euint256, uint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457575241204937520535, 115792089237316195423570985008687907853269984665640564039457575241204937520531)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575241204937520535n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457575241204937520531n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457575241204937520531n);
  });

  it('test operator "and" overload (uint256, euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457580197646013937297, 115792089237316195423570985008687907853269984665640564039457581093096062478513)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457581093096062478513n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457580197646013937297n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457579544535938113681n);
  });

  it('test operator "and" overload (uint256, euint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457575241204937520531, 115792089237316195423570985008687907853269984665640564039457575241204937520535)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457575241204937520535n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457575241204937520531n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457575241204937520531n);
  });

  it('test operator "and" overload (uint256, euint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457575241204937520535, 115792089237316195423570985008687907853269984665640564039457575241204937520535)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457575241204937520535n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457575241204937520535n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457575241204937520535n);
  });

  it('test operator "and" overload (uint256, euint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457575241204937520535, 115792089237316195423570985008687907853269984665640564039457575241204937520531)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457575241204937520531n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457575241204937520535n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457575241204937520531n);
  });

  it('test operator "or" overload (euint256, uint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457582881634656085895, 115792089237316195423570985008687907853269984665640564039457577518543938332183)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582881634656085895n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457577518543938332183n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457582881978857514903n);
  });

  it('test operator "or" overload (euint256, uint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457578460657678731511, 115792089237316195423570985008687907853269984665640564039457578460657678731515)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578460657678731511n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457578460657678731515n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457578460657678731519n);
  });

  it('test operator "or" overload (euint256, uint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457578460657678731515, 115792089237316195423570985008687907853269984665640564039457578460657678731515)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578460657678731515n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457578460657678731515n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457578460657678731515n);
  });

  it('test operator "or" overload (euint256, uint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457578460657678731515, 115792089237316195423570985008687907853269984665640564039457578460657678731511)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578460657678731515n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457578460657678731511n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457578460657678731519n);
  });

  it('test operator "or" overload (uint256, euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457582582266325416409, 115792089237316195423570985008687907853269984665640564039457577518543938332183)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457577518543938332183n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457582582266325416409n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457582600486684049375n);
  });

  it('test operator "or" overload (uint256, euint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457578460657678731511, 115792089237316195423570985008687907853269984665640564039457578460657678731515)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457578460657678731515n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457578460657678731511n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457578460657678731519n);
  });

  it('test operator "or" overload (uint256, euint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457578460657678731515, 115792089237316195423570985008687907853269984665640564039457578460657678731515)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457578460657678731515n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457578460657678731515n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457578460657678731515n);
  });

  it('test operator "or" overload (uint256, euint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457578460657678731515, 115792089237316195423570985008687907853269984665640564039457578460657678731511)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457578460657678731511n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457578460657678731515n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457578460657678731519n);
  });

  it('test operator "xor" overload (euint256, uint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457575044259526565797, 115792089237316195423570985008687907853269984665640564039457575436091334676909)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575044259526565797n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457575436091334676909n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(471020302241288n);
  });

  it('test operator "xor" overload (euint256, uint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457575044259526565793, 115792089237316195423570985008687907853269984665640564039457575044259526565797)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575044259526565793n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457575044259526565797n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint256, uint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457575044259526565797, 115792089237316195423570985008687907853269984665640564039457575044259526565797)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575044259526565797n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457575044259526565797n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint256, uint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457575044259526565797, 115792089237316195423570985008687907853269984665640564039457575044259526565793)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575044259526565797n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457575044259526565793n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (uint256, euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457575872436286243567, 115792089237316195423570985008687907853269984665640564039457575436091334676909)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457575436091334676909n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457575872436286243567n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(724760807772994n);
  });

  it('test operator "xor" overload (uint256, euint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457575044259526565793, 115792089237316195423570985008687907853269984665640564039457575044259526565797)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457575044259526565797n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457575044259526565793n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (uint256, euint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457575044259526565797, 115792089237316195423570985008687907853269984665640564039457575044259526565797)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457575044259526565797n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457575044259526565797n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (uint256, euint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457575044259526565797, 115792089237316195423570985008687907853269984665640564039457575044259526565793)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457575044259526565793n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457575044259526565797n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint256, uint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457575466255317790237, 115792089237316195423570985008687907853269984665640564039457580129835927933151)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575466255317790237n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457580129835927933151n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, uint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457575466255317790233, 115792089237316195423570985008687907853269984665640564039457575466255317790237)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575466255317790233n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457575466255317790237n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, uint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457575466255317790237, 115792089237316195423570985008687907853269984665640564039457575466255317790237)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575466255317790237n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457575466255317790237n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint256, uint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457575466255317790237, 115792089237316195423570985008687907853269984665640564039457575466255317790233)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575466255317790237n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457575466255317790233n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint256, euint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457577701107346059093, 115792089237316195423570985008687907853269984665640564039457580129835927933151)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457580129835927933151n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457577701107346059093n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint256, euint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457575466255317790233, 115792089237316195423570985008687907853269984665640564039457575466255317790237)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457575466255317790237n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457575466255317790233n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint256, euint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457575466255317790237, 115792089237316195423570985008687907853269984665640564039457575466255317790237)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457575466255317790237n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457575466255317790237n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint256, euint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457575466255317790237, 115792089237316195423570985008687907853269984665640564039457575466255317790233)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457575466255317790233n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457575466255317790237n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, uint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457575782941318555907, 115792089237316195423570985008687907853269984665640564039457580959519280070871)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575782941318555907n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457580959519280070871n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, uint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457575782941318555903, 115792089237316195423570985008687907853269984665640564039457575782941318555907)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575782941318555903n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457575782941318555907n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, uint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457575782941318555907, 115792089237316195423570985008687907853269984665640564039457575782941318555907)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575782941318555907n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457575782941318555907n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, uint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457575782941318555907, 115792089237316195423570985008687907853269984665640564039457575782941318555903)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575782941318555907n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457575782941318555903n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint256, euint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457583691349465032371, 115792089237316195423570985008687907853269984665640564039457580959519280070871)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457580959519280070871n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457583691349465032371n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint256, euint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457575782941318555903, 115792089237316195423570985008687907853269984665640564039457575782941318555907)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457575782941318555907n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457575782941318555903n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint256, euint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457575782941318555907, 115792089237316195423570985008687907853269984665640564039457575782941318555907)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457575782941318555907n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457575782941318555907n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint256, euint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457575782941318555907, 115792089237316195423570985008687907853269984665640564039457575782941318555903)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457575782941318555903n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457575782941318555907n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "shl" overload (euint8, euint8) => euint8 test 1 (5, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(5n);
    input.add8(7n);
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

  it('test operator "shl" overload (euint8, euint8) => euint8 test 2 (1, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(1n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(32n);
  });

  it('test operator "shl" overload (euint8, euint8) => euint8 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(5n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(160n);
  });

  it('test operator "shl" overload (euint8, euint8) => euint8 test 4 (5, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(5n);
    input.add8(1n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(10n);
  });

  it('test operator "shl" overload (euint8, uint8) => euint8 test 1 (5, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint8_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(128n);
  });

  it('test operator "shl" overload (euint8, uint8) => euint8 test 2 (1, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(1n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint8_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(32n);
  });

  it('test operator "shl" overload (euint8, uint8) => euint8 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint8_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(160n);
  });

  it('test operator "shl" overload (euint8, uint8) => euint8 test 4 (5, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint8_uint8(encryptedAmount.handles[0], 1n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(10n);
  });

  it('test operator "shr" overload (euint8, euint8) => euint8 test 1 (240, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(240n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(7n);
  });

  it('test operator "shr" overload (euint8, euint8) => euint8 test 2 (1, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(1n);
    input.add8(5n);
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

  it('test operator "shr" overload (euint8, euint8) => euint8 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(5n);
    input.add8(5n);
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

  it('test operator "shr" overload (euint8, euint8) => euint8 test 4 (5, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(5n);
    input.add8(1n);
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

  it('test operator "shr" overload (euint8, uint8) => euint8 test 1 (240, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(240n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint8_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(7n);
  });

  it('test operator "shr" overload (euint8, uint8) => euint8 test 2 (1, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(1n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint8_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint8, uint8) => euint8 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint8_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint8, uint8) => euint8 test 4 (5, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint8_uint8(encryptedAmount.handles[0], 1n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(2n);
  });

  it('test operator "rotl" overload (euint8, euint8) => euint8 test 1 (226, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(226n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(184n);
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

  it('test operator "rotl" overload (euint8, uint8) => euint8 test 1 (226, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(226n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint8_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(184n);
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

  it('test operator "rotr" overload (euint8, euint8) => euint8 test 1 (153, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(153n);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(51n);
  });

  it('test operator "rotr" overload (euint8, euint8) => euint8 test 2 (3, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(3n);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(6n);
  });

  it('test operator "rotr" overload (euint8, euint8) => euint8 test 3 (7, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(7n);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(14n);
  });

  it('test operator "rotr" overload (euint8, euint8) => euint8 test 4 (7, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(7n);
    input.add8(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(224n);
  });
});
