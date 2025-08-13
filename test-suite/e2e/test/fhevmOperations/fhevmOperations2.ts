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

describe('FHEVM operations 2', function () {
  before(async function () {
    this.signer = await getSigner(2);

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

  it('test operator "max" overload (euint64, euint64) => euint64 test 1 (18443029152777533213, 18444419935592779369)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add64(18443029152777533213n);
    input.add64(18444419935592779369n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18444419935592779369n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 2 (18443029152777533209, 18443029152777533213)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add64(18443029152777533209n);
    input.add64(18443029152777533213n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18443029152777533213n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 3 (18443029152777533213, 18443029152777533213)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add64(18443029152777533213n);
    input.add64(18443029152777533213n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18443029152777533213n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 4 (18443029152777533213, 18443029152777533209)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add64(18443029152777533213n);
    input.add64(18443029152777533209n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18443029152777533213n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint8, euint8) => ebool test 1 (76, 147)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(76n);
    input.add8(147n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint8(
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

  it('test operator "eq" overload (euint8, euint8) => ebool test 2 (72, 76)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(72n);
    input.add8(76n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint8(
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

  it('test operator "eq" overload (euint8, euint8) => ebool test 3 (76, 76)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(76n);
    input.add8(76n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint8(
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

  it('test operator "eq" overload (euint8, euint8) => ebool test 4 (76, 72)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(76n);
    input.add8(72n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint8(
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

  it('test operator "ge" overload (euint32, uint32) => ebool test 1 (940197925, 1424113892)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add32(940197925n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint32_uint32(
      encryptedAmount.handles[0],
      1424113892n,
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

  it('test operator "ge" overload (euint32, uint32) => ebool test 2 (518965360, 518965364)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add32(518965360n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint32_uint32(
      encryptedAmount.handles[0],
      518965364n,
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

  it('test operator "ge" overload (euint32, uint32) => ebool test 3 (518965364, 518965364)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add32(518965364n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint32_uint32(
      encryptedAmount.handles[0],
      518965364n,
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

  it('test operator "ge" overload (euint32, uint32) => ebool test 4 (518965364, 518965360)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add32(518965364n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint32_uint32(
      encryptedAmount.handles[0],
      518965360n,
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

  it('test operator "xor" overload (euint256, euint128) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457577934690347629639, 340282366920938463463365682509919785223)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577934690347629639n);
    input.add128(340282366920938463463365682509919785223n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907852929702298719625575994212257816047791424n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint256, euint128) => euint256 test 2 (340282366920938463463365682509919785219, 340282366920938463463365682509919785223)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add256(340282366920938463463365682509919785219n);
    input.add128(340282366920938463463365682509919785223n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint256, euint128) => euint256 test 3 (340282366920938463463365682509919785223, 340282366920938463463365682509919785223)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add256(340282366920938463463365682509919785223n);
    input.add128(340282366920938463463365682509919785223n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint256, euint128) => euint256 test 4 (340282366920938463463365682509919785223, 340282366920938463463365682509919785219)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add256(340282366920938463463365682509919785223n);
    input.add128(340282366920938463463365682509919785219n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint8, uint8) => euint8 test 1 (76, 10)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(76n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.rotr_euint8_uint8(encryptedAmount.handles[0], 10n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract1.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 19n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint8, uint8) => euint8 test 2 (6, 10)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(6n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.rotr_euint8_uint8(encryptedAmount.handles[0], 10n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract1.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 129n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint8, uint8) => euint8 test 3 (10, 10)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(10n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.rotr_euint8_uint8(encryptedAmount.handles[0], 10n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract1.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 130n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint8, uint8) => euint8 test 4 (10, 6)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(10n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.rotr_euint8_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract1.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 40n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint64, uint64) => euint64 test 1 (18437973024670093909, 18439997039756389969)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add64(18437973024670093909n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint64_uint64(
      encryptedAmount.handles[0],
      18439997039756389969n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18440226030233796181n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint64, uint64) => euint64 test 2 (18437973024670093905, 18437973024670093909)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add64(18437973024670093905n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint64_uint64(
      encryptedAmount.handles[0],
      18437973024670093909n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18437973024670093909n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint64, uint64) => euint64 test 3 (18437973024670093909, 18437973024670093909)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add64(18437973024670093909n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint64_uint64(
      encryptedAmount.handles[0],
      18437973024670093909n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18437973024670093909n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint64, uint64) => euint64 test 4 (18437973024670093909, 18437973024670093905)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add64(18437973024670093909n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint64_uint64(
      encryptedAmount.handles[0],
      18437973024670093905n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18437973024670093909n,
    };
    assert.deepEqual(res, expectedRes);
  });
});
