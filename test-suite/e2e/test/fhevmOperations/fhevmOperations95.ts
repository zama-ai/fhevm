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

describe('FHEVM operations 95', function () {
  before(async function () {
    this.signer = await getSigner(95);

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

  it('test operator "eq" overload (euint64, euint16) => ebool test 1 (18442604745060579793, 19421)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add64(18442604745060579793n);
    input.add16(19421n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint64, euint16) => ebool test 2 (19417, 19421)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add64(19417n);
    input.add16(19421n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint64, euint16) => ebool test 3 (19421, 19421)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add64(19421n);
    input.add16(19421n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint64, euint16) => ebool test 4 (19421, 19417)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add64(19421n);
    input.add16(19417n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 1 (4232162754, 7156)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add32(4232162754n);
    input.add16(7156n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4232164918n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 2 (7152, 7156)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add32(7152n);
    input.add16(7156n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 3 (7156, 7156)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add32(7156n);
    input.add16(7156n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 4 (7156, 7152)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add32(7156n);
    input.add16(7152n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (uint128, euint128) => euint128 test 1 (340282366920938463463367945823449544157, 340282366920938463463374302927749160189)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);

    input.add128(340282366920938463463374302927749160189n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_uint128_euint128(
      340282366920938463463367945823449544157n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463367923552693523677n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (uint128, euint128) => euint128 test 2 (340282366920938463463365634551935052359, 340282366920938463463365634551935052363)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);

    input.add128(340282366920938463463365634551935052363n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_uint128_euint128(
      340282366920938463463365634551935052359n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463365634551935052355n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (uint128, euint128) => euint128 test 3 (340282366920938463463365634551935052363, 340282366920938463463365634551935052363)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);

    input.add128(340282366920938463463365634551935052363n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_uint128_euint128(
      340282366920938463463365634551935052363n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463365634551935052363n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (uint128, euint128) => euint128 test 4 (340282366920938463463365634551935052363, 340282366920938463463365634551935052359)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);

    input.add128(340282366920938463463365634551935052359n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_uint128_euint128(
      340282366920938463463365634551935052363n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463365634551935052355n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (uint128, euint128) => ebool test 1 (340282366920938463463371583364305066843, 340282366920938463463368589084811487681)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);

    input.add128(340282366920938463463368589084811487681n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_uint128_euint128(
      340282366920938463463371583364305066843n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (uint128, euint128) => ebool test 2 (340282366920938463463368227465192969235, 340282366920938463463368227465192969239)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);

    input.add128(340282366920938463463368227465192969239n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_uint128_euint128(
      340282366920938463463368227465192969235n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (uint128, euint128) => ebool test 3 (340282366920938463463368227465192969239, 340282366920938463463368227465192969239)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);

    input.add128(340282366920938463463368227465192969239n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_uint128_euint128(
      340282366920938463463368227465192969239n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (uint128, euint128) => ebool test 4 (340282366920938463463368227465192969239, 340282366920938463463368227465192969235)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);

    input.add128(340282366920938463463368227465192969235n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_uint128_euint128(
      340282366920938463463368227465192969239n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint128, euint8) => ebool test 1 (340282366920938463463370380368381772675, 57)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add128(340282366920938463463370380368381772675n);
    input.add8(57n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint128, euint8) => ebool test 2 (53, 57)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add128(53n);
    input.add8(57n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint128, euint8) => ebool test 3 (57, 57)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add128(57n);
    input.add8(57n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint128, euint8) => ebool test 4 (57, 53)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add128(57n);
    input.add8(53n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint64, euint16) => euint64 test 1 (18439917834512730913, 906)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add64(18439917834512730913n);
    input.add16(906n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18439917834512730283n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint64, euint16) => euint64 test 2 (902, 906)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add64(902n);
    input.add16(906n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 12n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint64, euint16) => euint64 test 3 (906, 906)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add64(906n);
    input.add16(906n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint64, euint16) => euint64 test 4 (906, 902)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add64(906n);
    input.add16(902n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 12n,
    };
    assert.deepEqual(res, expectedRes);
  });
});
