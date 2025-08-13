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

describe('FHEVM operations 72', function () {
  before(async function () {
    this.signer = await getSigner(72);

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

  it('test operator "add" overload (euint8, euint8) => euint8 test 1 (123, 16)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add8(123n);
    input.add8(16n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 139n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint8, euint8) => euint8 test 2 (12, 16)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add8(12n);
    input.add8(16n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 28n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint8, euint8) => euint8 test 3 (16, 16)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add8(16n);
    input.add8(16n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 32n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint8, euint8) => euint8 test 4 (16, 12)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add8(16n);
    input.add8(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 28n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint128, euint256) => euint256 test 1 (340282366920938463463372996228688297193, 115792089237316195423570985008687907853269984665640564039457583382571766062153)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add128(340282366920938463463372996228688297193n);
    input.add256(115792089237316195423570985008687907853269984665640564039457583382571766062153n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457583946202484554985n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint128, euint256) => euint256 test 2 (340282366920938463463372996228688297189, 340282366920938463463372996228688297193)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add128(340282366920938463463372996228688297189n);
    input.add256(340282366920938463463372996228688297193n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463372996228688297197n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint128, euint256) => euint256 test 3 (340282366920938463463372996228688297193, 340282366920938463463372996228688297193)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add128(340282366920938463463372996228688297193n);
    input.add256(340282366920938463463372996228688297193n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463372996228688297193n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint128, euint256) => euint256 test 4 (340282366920938463463372996228688297193, 340282366920938463463372996228688297189)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add128(340282366920938463463372996228688297193n);
    input.add256(340282366920938463463372996228688297189n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463372996228688297197n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint128, euint128) => ebool test 1 (340282366920938463463374044506934573231, 340282366920938463463373233245058954601)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add128(340282366920938463463374044506934573231n);
    input.add128(340282366920938463463373233245058954601n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
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

  it('test operator "le" overload (euint128, euint128) => ebool test 2 (340282366920938463463373233245058954597, 340282366920938463463373233245058954601)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add128(340282366920938463463373233245058954597n);
    input.add128(340282366920938463463373233245058954601n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
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

  it('test operator "le" overload (euint128, euint128) => ebool test 3 (340282366920938463463373233245058954601, 340282366920938463463373233245058954601)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add128(340282366920938463463373233245058954601n);
    input.add128(340282366920938463463373233245058954601n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
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

  it('test operator "le" overload (euint128, euint128) => ebool test 4 (340282366920938463463373233245058954601, 340282366920938463463373233245058954597)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add128(340282366920938463463373233245058954601n);
    input.add128(340282366920938463463373233245058954597n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
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

  it('test operator "rotl" overload (euint64, uint8) => euint64 test 1 (18445582952718012215, 5)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add64(18445582952718012215n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotl_euint64_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract5.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18409588201980290815n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint64, uint8) => euint64 test 2 (1, 5)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add64(1n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotl_euint64_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract5.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 32n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint64, uint8) => euint64 test 3 (5, 5)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add64(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotl_euint64_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract5.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 160n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint64, uint8) => euint64 test 4 (5, 1)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add64(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotl_euint64_uint8(encryptedAmount.handles[0], 1n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract5.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 10n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint64, euint128) => euint128 test 1 (18439006272390009125, 340282366920938463463371772711833408309)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add64(18439006272390009125n);
    input.add128(340282366920938463463371772711833408309n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18439006272390009125n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint64, euint128) => euint128 test 2 (18439006272390009121, 18439006272390009125)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add64(18439006272390009121n);
    input.add128(18439006272390009125n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18439006272390009121n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint64, euint128) => euint128 test 3 (18439006272390009125, 18439006272390009125)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add64(18439006272390009125n);
    input.add128(18439006272390009125n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18439006272390009125n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint64, euint128) => euint128 test 4 (18439006272390009125, 18439006272390009121)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add64(18439006272390009125n);
    input.add128(18439006272390009121n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18439006272390009121n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (uint128, euint128) => euint128 test 1 (170141183460469231731684676888539948261, 170141183460469231731686022757356869593)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);

    input.add128(170141183460469231731686022757356869593n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_uint128_euint128(
      170141183460469231731684676888539948261n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463370699645896817854n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (uint128, euint128) => euint128 test 2 (170141183460469231731683215615520315653, 170141183460469231731683215615520315655)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);

    input.add128(170141183460469231731683215615520315655n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_uint128_euint128(
      170141183460469231731683215615520315653n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463366431231040631308n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (uint128, euint128) => euint128 test 3 (170141183460469231731683215615520315655, 170141183460469231731683215615520315655)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);

    input.add128(170141183460469231731683215615520315655n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_uint128_euint128(
      170141183460469231731683215615520315655n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463366431231040631310n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (uint128, euint128) => euint128 test 4 (170141183460469231731683215615520315655, 170141183460469231731683215615520315653)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);

    input.add128(170141183460469231731683215615520315653n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_uint128_euint128(
      170141183460469231731683215615520315655n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463366431231040631308n,
    };
    assert.deepEqual(res, expectedRes);
  });
});
