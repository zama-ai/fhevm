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

describe('FHEVM operations 64', function () {
  before(async function () {
    this.signer = await getSigner(64);

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

  it('test operator "or" overload (euint16, euint256) => euint256 test 1 (45255, 115792089237316195423570985008687907853269984665640564039457580032423081744359)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add16(45255n);
    input.add256(115792089237316195423570985008687907853269984665640564039457580032423081744359n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457580032423081785319n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint16, euint256) => euint256 test 2 (45251, 45255)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add16(45251n);
    input.add256(45255n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 45255n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint16, euint256) => euint256 test 3 (45255, 45255)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add16(45255n);
    input.add256(45255n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 45255n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint16, euint256) => euint256 test 4 (45255, 45251)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add16(45255n);
    input.add256(45251n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 45255n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint64, euint128) => euint128 test 1 (18439646191939862959, 18439646191939862959)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add64(18439646191939862959n);
    input.add128(18439646191939862959n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint64, euint128) => euint128 test 2 (18439646191939862959, 18439646191939862955)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add64(18439646191939862959n);
    input.add128(18439646191939862955n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 1 (939, 9099)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);

    input.add16(9099n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_uint16_euint16(939n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract5.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 939n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 2 (28653, 28657)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);

    input.add16(28657n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_uint16_euint16(28653n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract5.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 28653n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 3 (28657, 28657)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);

    input.add16(28657n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_uint16_euint16(28657n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract5.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 28657n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 4 (28657, 28653)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);

    input.add16(28653n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_uint16_euint16(28657n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract5.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 28653n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (uint128, euint128) => euint128 test 1 (340282366920938463463367956382031476921, 340282366920938463463373189985976815179)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);

    input.add128(340282366920938463463373189985976815179n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_uint128_euint128(
      340282366920938463463367956382031476921n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463373189985976815179n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (uint128, euint128) => euint128 test 2 (340282366920938463463371563207649937683, 340282366920938463463371563207649937687)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);

    input.add128(340282366920938463463371563207649937687n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_uint128_euint128(
      340282366920938463463371563207649937683n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463371563207649937687n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (uint128, euint128) => euint128 test 3 (340282366920938463463371563207649937687, 340282366920938463463371563207649937687)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);

    input.add128(340282366920938463463371563207649937687n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_uint128_euint128(
      340282366920938463463371563207649937687n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463371563207649937687n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (uint128, euint128) => euint128 test 4 (340282366920938463463371563207649937687, 340282366920938463463371563207649937683)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);

    input.add128(340282366920938463463371563207649937683n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_uint128_euint128(
      340282366920938463463371563207649937687n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463371563207649937687n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 1 (18438118695059203909, 18443753646578158425)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add64(18438118695059203909n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint64_uint64(
      encryptedAmount.handles[0],
      18443753646578158425n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 2 (18438118695059203905, 18438118695059203909)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add64(18438118695059203905n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint64_uint64(
      encryptedAmount.handles[0],
      18438118695059203909n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 3 (18438118695059203909, 18438118695059203909)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add64(18438118695059203909n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint64_uint64(
      encryptedAmount.handles[0],
      18438118695059203909n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 4 (18438118695059203909, 18438118695059203905)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add64(18438118695059203909n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint64_uint64(
      encryptedAmount.handles[0],
      18438118695059203905n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint8, uint8) => euint8 test 1 (93, 6)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add8(93n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shr_euint8_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract5.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint8, uint8) => euint8 test 2 (2, 6)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add8(2n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shr_euint8_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract5.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint8, uint8) => euint8 test 3 (6, 6)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add8(6n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shr_euint8_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract5.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint8, uint8) => euint8 test 4 (6, 2)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add8(6n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shr_euint8_uint8(encryptedAmount.handles[0], 2n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract5.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1n,
    };
    assert.deepEqual(res, expectedRes);
  });
});
