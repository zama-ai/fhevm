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

describe('FHEVM operations 40', function () {
  before(async function () {
    this.signer = await getSigner(40);

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

  it('test operator "ne" overload (euint32, euint256) => ebool test 1 (296674264, 115792089237316195423570985008687907853269984665640564039457578621184520276501)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add32(296674264n);
    input.add256(115792089237316195423570985008687907853269984665640564039457578621184520276501n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint256(
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
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint32, euint256) => ebool test 2 (296674260, 296674264)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add32(296674260n);
    input.add256(296674264n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint256(
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
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint32, euint256) => ebool test 3 (296674264, 296674264)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add32(296674264n);
    input.add256(296674264n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint256(
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
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint32, euint256) => ebool test 4 (296674264, 296674260)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add32(296674264n);
    input.add256(296674260n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint256(
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
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (uint16, euint16) => euint16 test 1 (1593, 1593)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);

    input.add16(1593n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_uint16_euint16(1593n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract3.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (uint16, euint16) => euint16 test 2 (1593, 1589)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);

    input.add16(1589n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_uint16_euint16(1593n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract3.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (uint256, euint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457579773753745488495, 115792089237316195423570985008687907853269984665640564039457580051207119472743)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457580051207119472743n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457579773753745488495n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (uint256, euint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457576108325688778349, 115792089237316195423570985008687907853269984665640564039457576108325688778353)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457576108325688778353n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457576108325688778349n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (uint256, euint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457576108325688778353, 115792089237316195423570985008687907853269984665640564039457576108325688778353)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457576108325688778353n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457576108325688778353n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (uint256, euint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457576108325688778353, 115792089237316195423570985008687907853269984665640564039457576108325688778349)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457576108325688778349n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457576108325688778353n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint128, euint128) => euint128 test 1 (340282366920938463463369580143827812389, 340282366920938463463371636800776899801)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add128(340282366920938463463369580143827812389n);
    input.add128(340282366920938463463371636800776899801n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463366763722242588673n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint128, euint128) => euint128 test 2 (340282366920938463463369580143827812385, 340282366920938463463369580143827812389)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add128(340282366920938463463369580143827812385n);
    input.add128(340282366920938463463369580143827812389n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463369580143827812385n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint128, euint128) => euint128 test 3 (340282366920938463463369580143827812389, 340282366920938463463369580143827812389)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add128(340282366920938463463369580143827812389n);
    input.add128(340282366920938463463369580143827812389n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463369580143827812389n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint128, euint128) => euint128 test 4 (340282366920938463463369580143827812389, 340282366920938463463369580143827812385)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add128(340282366920938463463369580143827812389n);
    input.add128(340282366920938463463369580143827812385n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463369580143827812385n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint256, euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457575017621057149739, 115792089237316195423570985008687907853269984665640564039457579571461707808381)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575017621057149739n);
    input.add256(115792089237316195423570985008687907853269984665640564039457579571461707808381n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457579574075229454207n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint256, euint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457575017621057149735, 115792089237316195423570985008687907853269984665640564039457575017621057149739)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575017621057149735n);
    input.add256(115792089237316195423570985008687907853269984665640564039457575017621057149739n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457575017621057149743n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint256, euint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457575017621057149739, 115792089237316195423570985008687907853269984665640564039457575017621057149739)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575017621057149739n);
    input.add256(115792089237316195423570985008687907853269984665640564039457575017621057149739n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457575017621057149739n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint256, euint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457575017621057149739, 115792089237316195423570985008687907853269984665640564039457575017621057149735)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575017621057149739n);
    input.add256(115792089237316195423570985008687907853269984665640564039457575017621057149735n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457575017621057149743n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (uint128, euint128) => ebool test 1 (340282366920938463463366383408546800861, 340282366920938463463367192195201233237)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);

    input.add128(340282366920938463463367192195201233237n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_uint128_euint128(
      340282366920938463463366383408546800861n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (uint128, euint128) => ebool test 2 (340282366920938463463368791044669707087, 340282366920938463463368791044669707091)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);

    input.add128(340282366920938463463368791044669707091n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_uint128_euint128(
      340282366920938463463368791044669707087n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (uint128, euint128) => ebool test 3 (340282366920938463463368791044669707091, 340282366920938463463368791044669707091)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);

    input.add128(340282366920938463463368791044669707091n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_uint128_euint128(
      340282366920938463463368791044669707091n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (uint128, euint128) => ebool test 4 (340282366920938463463368791044669707091, 340282366920938463463368791044669707087)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);

    input.add128(340282366920938463463368791044669707087n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_uint128_euint128(
      340282366920938463463368791044669707091n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });
});
