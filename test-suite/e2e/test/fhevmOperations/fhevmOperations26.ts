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

describe('FHEVM operations 26', function () {
  before(async function () {
    this.signer = await getSigner(26);

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

  it('test operator "gt" overload (euint128, euint128) => ebool test 1 (340282366920938463463369926229248711475, 340282366920938463463370021475627392493)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add128(340282366920938463463369926229248711475n);
    input.add128(340282366920938463463370021475627392493n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "gt" overload (euint128, euint128) => ebool test 2 (340282366920938463463369926229248711471, 340282366920938463463369926229248711475)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add128(340282366920938463463369926229248711471n);
    input.add128(340282366920938463463369926229248711475n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "gt" overload (euint128, euint128) => ebool test 3 (340282366920938463463369926229248711475, 340282366920938463463369926229248711475)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add128(340282366920938463463369926229248711475n);
    input.add128(340282366920938463463369926229248711475n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "gt" overload (euint128, euint128) => ebool test 4 (340282366920938463463369926229248711475, 340282366920938463463369926229248711471)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add128(340282366920938463463369926229248711475n);
    input.add128(340282366920938463463369926229248711471n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 1 (2321629392, 21828)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(2321629392n);
    input.add16(21828n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 21828n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 2 (21824, 21828)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(21824n);
    input.add16(21828n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 21824n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 3 (21828, 21828)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(21828n);
    input.add16(21828n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 21828n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 4 (21828, 21824)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(21828n);
    input.add16(21824n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 21824n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "and" overload (euint8, euint128) => euint128 test 1 (83, 340282366920938463463372481355988654629)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add8(83n);
    input.add128(340282366920938463463372481355988654629n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "and" overload (euint8, euint128) => euint128 test 2 (79, 83)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add8(79n);
    input.add128(83n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 67n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "and" overload (euint8, euint128) => euint128 test 3 (83, 83)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add8(83n);
    input.add128(83n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 83n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "and" overload (euint8, euint128) => euint128 test 4 (83, 79)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add8(83n);
    input.add128(79n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 67n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "eq" overload (euint256, uint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457575496626475170541, 115792089237316195423570985008687907853269984665640564039457578709698494561291)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575496626475170541n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457578709698494561291n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "eq" overload (euint256, uint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457575496626475170537, 115792089237316195423570985008687907853269984665640564039457575496626475170541)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575496626475170537n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457575496626475170541n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "eq" overload (euint256, uint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457575496626475170541, 115792089237316195423570985008687907853269984665640564039457575496626475170541)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575496626475170541n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457575496626475170541n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "eq" overload (euint256, uint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457575496626475170541, 115792089237316195423570985008687907853269984665640564039457575496626475170537)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575496626475170541n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457575496626475170537n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "ne" overload (euint256, uint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457576299642184965505, 115792089237316195423570985008687907853269984665640564039457583034685916919557)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576299642184965505n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457583034685916919557n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "ne" overload (euint256, uint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457576299642184965501, 115792089237316195423570985008687907853269984665640564039457576299642184965505)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576299642184965501n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457576299642184965505n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "ne" overload (euint256, uint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457576299642184965505, 115792089237316195423570985008687907853269984665640564039457576299642184965505)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576299642184965505n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457576299642184965505n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "ne" overload (euint256, uint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457576299642184965505, 115792089237316195423570985008687907853269984665640564039457576299642184965501)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576299642184965505n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457576299642184965501n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 1 (4070953825, 200)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(4070953825n);
    input.add8(200n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 2 (196, 200)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(196n);
    input.add8(200n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 3 (200, 200)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(200n);
    input.add8(200n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 4 (200, 196)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(200n);
    input.add8(196n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });
});
