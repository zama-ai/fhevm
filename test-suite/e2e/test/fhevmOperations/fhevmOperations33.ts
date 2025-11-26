import { HardhatEthersSigner } from '@nomicfoundation/hardhat-ethers/signers';
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

describe('FHEVM operations 33', function () {
  before(async function () {
    this.signer = await getSigner(33);

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

  it('test operator "eq" overload (uint256, euint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457579550633204727523, 115792089237316195423570985008687907853269984665640564039457578709698494561291)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457578709698494561291n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457579550633204727523n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "eq" overload (uint256, euint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457575496626475170537, 115792089237316195423570985008687907853269984665640564039457575496626475170541)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457575496626475170541n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457575496626475170537n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "eq" overload (uint256, euint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457575496626475170541, 115792089237316195423570985008687907853269984665640564039457575496626475170541)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457575496626475170541n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457575496626475170541n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "eq" overload (uint256, euint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457575496626475170541, 115792089237316195423570985008687907853269984665640564039457575496626475170537)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457575496626475170537n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457575496626475170541n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "not" overload (euint8) => euint8 test 1 (109)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(109n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.not_euint8(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract3.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 146n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "min" overload (euint128, euint64) => euint128 test 1 (340282366920938463463367468006661140089, 18438522459626149825)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add128(340282366920938463463367468006661140089n);
    input.add64(18438522459626149825n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18438522459626149825n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "min" overload (euint128, euint64) => euint128 test 2 (18438522459626149821, 18438522459626149825)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add128(18438522459626149821n);
    input.add64(18438522459626149825n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18438522459626149821n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "min" overload (euint128, euint64) => euint128 test 3 (18438522459626149825, 18438522459626149825)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add128(18438522459626149825n);
    input.add64(18438522459626149825n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18438522459626149825n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "min" overload (euint128, euint64) => euint128 test 4 (18438522459626149825, 18438522459626149821)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add128(18438522459626149825n);
    input.add64(18438522459626149821n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18438522459626149821n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "mul" overload (euint64, euint16) => euint64 test 1 (32754, 2)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add64(32754n);
    input.add16(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 65508n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "mul" overload (euint64, euint16) => euint64 test 2 (212, 212)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add64(212n);
    input.add16(212n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 44944n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "mul" overload (euint64, euint16) => euint64 test 3 (212, 212)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add64(212n);
    input.add16(212n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 44944n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "mul" overload (euint64, euint16) => euint64 test 4 (212, 212)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add64(212n);
    input.add16(212n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 44944n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "lt" overload (euint8, euint8) => ebool test 1 (52, 166)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(52n);
    input.add8(166n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "lt" overload (euint8, euint8) => ebool test 2 (48, 52)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(48n);
    input.add8(52n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "lt" overload (euint8, euint8) => ebool test 3 (52, 52)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(52n);
    input.add8(52n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "lt" overload (euint8, euint8) => ebool test 4 (52, 48)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(52n);
    input.add8(48n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 1 (43195, 201)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add16(43195n);
    input.add8(201n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 2 (197, 201)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add16(197n);
    input.add8(201n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 3 (201, 201)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add16(201n);
    input.add8(201n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 4 (201, 197)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add16(201n);
    input.add8(197n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });
});
