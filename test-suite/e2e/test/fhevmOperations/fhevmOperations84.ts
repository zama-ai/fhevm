import { assert } from 'chai';
import { ethers } from 'hardhat';

import type { FHEVMTestSuite1 } from '../../types/contracts/tests/FHEVMTestSuite1';
import type { FHEVMTestSuite2 } from '../../types/contracts/tests/FHEVMTestSuite2';
import type { FHEVMTestSuite3 } from '../../types/contracts/tests/FHEVMTestSuite3';
import type { FHEVMTestSuite4 } from '../../types/contracts/tests/FHEVMTestSuite4';
import type { FHEVMTestSuite5 } from '../../types/contracts/tests/FHEVMTestSuite5';
import type { FHEVMTestSuite6 } from '../../types/contracts/tests/FHEVMTestSuite6';
import type { FHEVMTestSuite7 } from '../../types/contracts/tests/FHEVMTestSuite7';
import { createInstance } from '../instance';
import { getSigner, getSigners, initSigners } from '../signers';

async function deployFHEVMTestFixture1(signer: HardhatEthersSigner): Promise<FHEVMTestSuite1> {
  const admin = signer;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite1');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployFHEVMTestFixture2(signer: HardhatEthersSigner): Promise<FHEVMTestSuite2> {
  const admin = signer;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite2');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployFHEVMTestFixture3(signer: HardhatEthersSigner): Promise<FHEVMTestSuite3> {
  const admin = signer;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite3');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployFHEVMTestFixture4(signer: HardhatEthersSigner): Promise<FHEVMTestSuite4> {
  const admin = signer;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite4');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployFHEVMTestFixture5(signer: HardhatEthersSigner): Promise<FHEVMTestSuite5> {
  const admin = signer;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite5');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployFHEVMTestFixture6(signer: HardhatEthersSigner): Promise<FHEVMTestSuite6> {
  const admin = signer;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite6');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployFHEVMTestFixture7(signer: HardhatEthersSigner): Promise<FHEVMTestSuite7> {
  const admin = signer;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite7');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

describe('FHEVM operations 84', function () {
  before(async function () {
    this.signer = await getSigner(84);

    const contract1 = await deployFHEVMTestFixture1(this.signer);
    this.contract1Address = await contract1.getAddress();
    this.contract1 = contract1;

    const contract2 = await deployFHEVMTestFixture2(this.signer);
    this.contract2Address = await contract2.getAddress();
    this.contract2 = contract2;

    const contract3 = await deployFHEVMTestFixture3(this.signer);
    this.contract3Address = await contract3.getAddress();
    this.contract3 = contract3;

    const contract4 = await deployFHEVMTestFixture4(this.signer);
    this.contract4Address = await contract4.getAddress();
    this.contract4 = contract4;

    const contract5 = await deployFHEVMTestFixture5(this.signer);
    this.contract5Address = await contract5.getAddress();
    this.contract5 = contract5;

    const contract6 = await deployFHEVMTestFixture6(this.signer);
    this.contract6Address = await contract6.getAddress();
    this.contract6 = contract6;

    const contract7 = await deployFHEVMTestFixture7(this.signer);
    this.contract7Address = await contract7.getAddress();
    this.contract7 = contract7;

    const instance = await createInstance();
    this.instance = instance;
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 1 (18437803334447700951, 47708)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add64(18437803334447700951n);
    input.add16(47708n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18437803334447700951n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 2 (47704, 47708)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add64(47704n);
    input.add16(47708n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 47708n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 3 (47708, 47708)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add64(47708n);
    input.add16(47708n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 47708n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 4 (47708, 47704)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add64(47708n);
    input.add16(47704n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 47708n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint128, uint128) => ebool test 1 (340282366920938463463372039300008014685, 340282366920938463463368162728995832595)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add128(340282366920938463463372039300008014685n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368162728995832595n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint128, uint128) => ebool test 2 (340282366920938463463367203559539217349, 340282366920938463463367203559539217353)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add128(340282366920938463463367203559539217349n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463367203559539217353n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint128, uint128) => ebool test 3 (340282366920938463463367203559539217353, 340282366920938463463367203559539217353)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add128(340282366920938463463367203559539217353n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463367203559539217353n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint128, uint128) => ebool test 4 (340282366920938463463367203559539217353, 340282366920938463463367203559539217349)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add128(340282366920938463463367203559539217353n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463367203559539217349n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint128, uint128) => euint128 test 1 (340282366920938463463365634551935052363, 340282366920938463463374302927749160189)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add128(340282366920938463463365634551935052363n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463374302927749160189n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463365612352912001097n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint128, uint128) => euint128 test 2 (340282366920938463463365634551935052359, 340282366920938463463365634551935052363)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add128(340282366920938463463365634551935052359n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463365634551935052363n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463365634551935052355n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint128, uint128) => euint128 test 3 (340282366920938463463365634551935052363, 340282366920938463463365634551935052363)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add128(340282366920938463463365634551935052363n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463365634551935052363n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463365634551935052363n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint128, uint128) => euint128 test 4 (340282366920938463463365634551935052363, 340282366920938463463365634551935052359)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add128(340282366920938463463365634551935052363n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463365634551935052359n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463365634551935052355n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint128, uint128) => euint128 test 1 (170141183460469231731684782079341823794, 170141183460469231731686022757356869593)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add128(170141183460469231731684782079341823794n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint128_uint128(
      encryptedAmount.handles[0],
      170141183460469231731686022757356869593n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463370804836698693387n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint128, uint128) => euint128 test 2 (170141183460469231731683215615520315653, 170141183460469231731683215615520315655)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add128(170141183460469231731683215615520315653n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint128_uint128(
      encryptedAmount.handles[0],
      170141183460469231731683215615520315655n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463366431231040631308n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint128, uint128) => euint128 test 3 (170141183460469231731683215615520315655, 170141183460469231731683215615520315655)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add128(170141183460469231731683215615520315655n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint128_uint128(
      encryptedAmount.handles[0],
      170141183460469231731683215615520315655n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463366431231040631310n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint128, uint128) => euint128 test 4 (170141183460469231731683215615520315655, 170141183460469231731683215615520315653)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add128(170141183460469231731683215615520315655n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint128_uint128(
      encryptedAmount.handles[0],
      170141183460469231731683215615520315653n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463366431231040631308n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint8, euint32) => euint32 test 1 (2, 77)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add8(2n);
    input.add32(77n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 154n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint8, euint32) => euint32 test 2 (9, 9)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add8(9n);
    input.add32(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 81n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint8, euint32) => euint32 test 3 (9, 9)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add8(9n);
    input.add32(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 81n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint8, euint32) => euint32 test 4 (9, 9)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add8(9n);
    input.add32(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 81n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint128, euint32) => euint128 test 1 (340282366920938463463371740885402340955, 875603075)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add128(340282366920938463463371740885402340955n);
    input.add32(875603075n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463371740885738974939n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint128, euint32) => euint128 test 2 (875603071, 875603075)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add128(875603071n);
    input.add32(875603075n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 875603199n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint128, euint32) => euint128 test 3 (875603075, 875603075)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add128(875603075n);
    input.add32(875603075n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 875603075n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint128, euint32) => euint128 test 4 (875603075, 875603071)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add128(875603075n);
    input.add32(875603071n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 875603199n,
    };
    assert.deepEqual(res, expectedRes);
  });
});
