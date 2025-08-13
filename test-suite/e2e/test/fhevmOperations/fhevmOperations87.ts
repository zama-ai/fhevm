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

describe('FHEVM operations 87', function () {
  before(async function () {
    this.signer = await getSigner(87);

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

  it('test operator "le" overload (euint64, euint64) => ebool test 1 (18443903610248017297, 18446695778791465943)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add64(18443903610248017297n);
    input.add64(18446695778791465943n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
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

  it('test operator "le" overload (euint64, euint64) => ebool test 2 (18443903610248017293, 18443903610248017297)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add64(18443903610248017293n);
    input.add64(18443903610248017297n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
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

  it('test operator "le" overload (euint64, euint64) => ebool test 3 (18443903610248017297, 18443903610248017297)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add64(18443903610248017297n);
    input.add64(18443903610248017297n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
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

  it('test operator "le" overload (euint64, euint64) => ebool test 4 (18443903610248017297, 18443903610248017293)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add64(18443903610248017297n);
    input.add64(18443903610248017293n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
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

  it('test operator "ne" overload (euint64, euint64) => ebool test 1 (18442684280855507527, 18446661370111082195)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add64(18442684280855507527n);
    input.add64(18446661370111082195n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
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

  it('test operator "ne" overload (euint64, euint64) => ebool test 2 (18442684280855507523, 18442684280855507527)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add64(18442684280855507523n);
    input.add64(18442684280855507527n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
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

  it('test operator "ne" overload (euint64, euint64) => ebool test 3 (18442684280855507527, 18442684280855507527)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add64(18442684280855507527n);
    input.add64(18442684280855507527n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
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

  it('test operator "ne" overload (euint64, euint64) => ebool test 4 (18442684280855507527, 18442684280855507523)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add64(18442684280855507527n);
    input.add64(18442684280855507523n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
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

  it('test operator "or" overload (euint64, euint128) => euint128 test 1 (18444154156152306099, 340282366920938463463370835647881067077)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add64(18444154156152306099n);
    input.add128(340282366920938463463370835647881067077n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463372036773401902071n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint64, euint128) => euint128 test 2 (18444154156152306095, 18444154156152306099)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add64(18444154156152306095n);
    input.add128(18444154156152306099n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18444154156152306111n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint64, euint128) => euint128 test 3 (18444154156152306099, 18444154156152306099)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add64(18444154156152306099n);
    input.add128(18444154156152306099n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18444154156152306099n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint64, euint128) => euint128 test 4 (18444154156152306099, 18444154156152306095)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add64(18444154156152306099n);
    input.add128(18444154156152306095n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18444154156152306111n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint32, euint128) => euint128 test 1 (2, 2147483649)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add32(2n);
    input.add128(2147483649n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2147483651n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint32, euint128) => euint128 test 2 (54151047, 54151051)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add32(54151047n);
    input.add128(54151051n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 108302098n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint32, euint128) => euint128 test 3 (54151051, 54151051)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add32(54151051n);
    input.add128(54151051n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 108302102n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint32, euint128) => euint128 test 4 (54151051, 54151047)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add32(54151051n);
    input.add128(54151047n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 108302098n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint8, euint32) => euint32 test 1 (155, 527859839)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add8(155n);
    input.add32(527859839n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 527859967n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint8, euint32) => euint32 test 2 (151, 155)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add8(151n);
    input.add32(155n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 159n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint8, euint32) => euint32 test 3 (155, 155)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add8(155n);
    input.add32(155n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 155n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint8, euint32) => euint32 test 4 (155, 151)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add8(155n);
    input.add32(151n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 159n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint128, uint128) => ebool test 1 (340282366920938463463370114914581200015, 340282366920938463463368589084811487681)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add128(340282366920938463463370114914581200015n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368589084811487681n,
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

  it('test operator "ge" overload (euint128, uint128) => ebool test 2 (340282366920938463463368227465192969235, 340282366920938463463368227465192969239)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add128(340282366920938463463368227465192969235n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368227465192969239n,
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

  it('test operator "ge" overload (euint128, uint128) => ebool test 3 (340282366920938463463368227465192969239, 340282366920938463463368227465192969239)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add128(340282366920938463463368227465192969239n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368227465192969239n,
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

  it('test operator "ge" overload (euint128, uint128) => ebool test 4 (340282366920938463463368227465192969239, 340282366920938463463368227465192969235)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add128(340282366920938463463368227465192969239n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368227465192969235n,
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
});
