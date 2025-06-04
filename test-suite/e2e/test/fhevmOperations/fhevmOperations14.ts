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

describe('FHEVM operations 14', function () {
  before(async function () {
    this.signer = await getSigner(14);

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

  it('test operator "and" overload (uint128, euint128) => euint128 test 1 (340282366920938463463365890598606382587, 340282366920938463463373362206958024403)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);

    input.add128(340282366920938463463373362206958024403n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_uint128_euint128(
      340282366920938463463365890598606382587n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463365881776234865875n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (uint128, euint128) => euint128 test 2 (340282366920938463463369580143827812385, 340282366920938463463369580143827812389)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);

    input.add128(340282366920938463463369580143827812389n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_uint128_euint128(
      340282366920938463463369580143827812385n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463369580143827812385n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (uint128, euint128) => euint128 test 3 (340282366920938463463369580143827812389, 340282366920938463463369580143827812389)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);

    input.add128(340282366920938463463369580143827812389n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_uint128_euint128(
      340282366920938463463369580143827812389n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463369580143827812389n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (uint128, euint128) => euint128 test 4 (340282366920938463463369580143827812389, 340282366920938463463369580143827812385)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);

    input.add128(340282366920938463463369580143827812385n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_uint128_euint128(
      340282366920938463463369580143827812389n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463369580143827812385n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (uint8, euint8) => euint8 test 1 (9, 15)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);

    input.add8(15n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_uint8_euint8(9n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract1.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 135n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (uint8, euint8) => euint8 test 2 (11, 12)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);

    input.add8(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_uint8_euint8(11n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract1.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 132n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (uint8, euint8) => euint8 test 3 (12, 12)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);

    input.add8(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_uint8_euint8(12n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract1.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 144n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (uint8, euint8) => euint8 test 4 (12, 11)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);

    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_uint8_euint8(12n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract1.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 132n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 1 (1475221966, 875340298)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);

    input.add32(875340298n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_uint32_euint32(
      1475221966n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 2 (2239042912, 2239042916)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);

    input.add32(2239042916n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_uint32_euint32(
      2239042912n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 3 (2239042916, 2239042916)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);

    input.add32(2239042916n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_uint32_euint32(
      2239042916n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 4 (2239042916, 2239042912)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);

    input.add32(2239042912n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_uint32_euint32(
      2239042916n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint8, uint8) => euint8 test 1 (5, 5)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint8_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract1.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint8, uint8) => euint8 test 2 (5, 1)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint8_uint8(encryptedAmount.handles[0], 1n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract1.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 1 (1289585563, 3055847849)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add32(1289585563n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint32_uint32(
      encryptedAmount.handles[0],
      3055847849n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 2 (782760031, 782760035)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add32(782760031n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint32_uint32(
      encryptedAmount.handles[0],
      782760035n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 3 (782760035, 782760035)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add32(782760035n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint32_uint32(
      encryptedAmount.handles[0],
      782760035n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 4 (782760035, 782760031)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add32(782760035n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint32_uint32(
      encryptedAmount.handles[0],
      782760031n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint16, euint8) => ebool test 1 (33877, 20)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(33877n);
    input.add8(20n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint16, euint8) => ebool test 2 (16, 20)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(16n);
    input.add8(20n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint16, euint8) => ebool test 3 (20, 20)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(20n);
    input.add8(20n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint16, euint8) => ebool test 4 (20, 16)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(20n);
    input.add8(16n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });
});
