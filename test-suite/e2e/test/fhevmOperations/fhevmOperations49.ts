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

describe('FHEVM operations 49', function () {
  before(async function () {
    this.signer = await getSigner(49);

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

  it('test operator "ge" overload (euint16, euint32) => ebool test 1 (37615, 2523138817)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add16(37615n);
    input.add32(2523138817n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 2 (37611, 37615)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add16(37611n);
    input.add32(37615n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 3 (37615, 37615)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add16(37615n);
    input.add32(37615n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 4 (37615, 37611)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add16(37615n);
    input.add32(37611n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint128, euint8) => euint128 test 1 (340282366920938463463373438166170525991, 14)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(340282366920938463463373438166170525991n);
    input.add8(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463373438166170525993n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint128, euint8) => euint128 test 2 (10, 14)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(10n);
    input.add8(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint128, euint8) => euint128 test 3 (14, 14)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(14n);
    input.add8(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint128, euint8) => euint128 test 4 (14, 10)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(14n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint256, euint16) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457576572784219600935, 52083)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576572784219600935n);
    input.add16(52083n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint256, euint16) => ebool test 2 (52079, 52083)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add256(52079n);
    input.add16(52083n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint256, euint16) => ebool test 3 (52083, 52083)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add256(52083n);
    input.add16(52083n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint256, euint16) => ebool test 4 (52083, 52079)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add256(52083n);
    input.add16(52079n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint128, euint256) => euint256 test 1 (340282366920938463463367833682660429651, 115792089237316195423570985008687907853269984665640564039457581500422929706231)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(340282366920938463463367833682660429651n);
    input.add256(115792089237316195423570985008687907853269984665640564039457581500422929706231n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463367578542231653459n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint128, euint256) => euint256 test 2 (340282366920938463463367833682660429647, 340282366920938463463367833682660429651)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(340282366920938463463367833682660429647n);
    input.add256(340282366920938463463367833682660429651n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463367833682660429635n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint128, euint256) => euint256 test 3 (340282366920938463463367833682660429651, 340282366920938463463367833682660429651)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(340282366920938463463367833682660429651n);
    input.add256(340282366920938463463367833682660429651n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463367833682660429651n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint128, euint256) => euint256 test 4 (340282366920938463463367833682660429651, 340282366920938463463367833682660429647)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(340282366920938463463367833682660429651n);
    input.add256(340282366920938463463367833682660429647n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463367833682660429635n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (uint64, euint64) => euint64 test 1 (18443590342941199675, 18439997039756389969)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);

    input.add64(18439997039756389969n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_uint64_euint64(
      18443590342941199675n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18445846580129951611n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (uint64, euint64) => euint64 test 2 (18437973024670093905, 18437973024670093909)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);

    input.add64(18437973024670093909n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_uint64_euint64(
      18437973024670093905n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18437973024670093909n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (uint64, euint64) => euint64 test 3 (18437973024670093909, 18437973024670093909)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);

    input.add64(18437973024670093909n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_uint64_euint64(
      18437973024670093909n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18437973024670093909n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (uint64, euint64) => euint64 test 4 (18437973024670093909, 18437973024670093905)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);

    input.add64(18437973024670093905n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_uint64_euint64(
      18437973024670093909n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18437973024670093909n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 1 (941506101, 3341468952)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add32(941506101n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint32_uint32(
      encryptedAmount.handles[0],
      3341468952n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 2 (941506097, 941506101)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add32(941506097n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint32_uint32(
      encryptedAmount.handles[0],
      941506101n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 3 (941506101, 941506101)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add32(941506101n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint32_uint32(
      encryptedAmount.handles[0],
      941506101n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 4 (941506101, 941506097)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add32(941506101n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint32_uint32(
      encryptedAmount.handles[0],
      941506097n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });
});
