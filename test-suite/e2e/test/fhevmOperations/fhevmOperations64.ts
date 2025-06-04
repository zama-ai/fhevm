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

  it('test operator "and" overload (euint64, euint256) => euint256 test 1 (18443221601080337457, 115792089237316195423570985008687907853269984665640564039457576798092187173417)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add64(18443221601080337457n);
    input.add256(115792089237316195423570985008687907853269984665640564039457576798092187173417n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18438405464196907041n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint64, euint256) => euint256 test 2 (18443221601080337453, 18443221601080337457)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add64(18443221601080337453n);
    input.add256(18443221601080337457n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18443221601080337441n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint64, euint256) => euint256 test 3 (18443221601080337457, 18443221601080337457)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add64(18443221601080337457n);
    input.add256(18443221601080337457n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18443221601080337457n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint64, euint256) => euint256 test 4 (18443221601080337457, 18443221601080337453)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add64(18443221601080337457n);
    input.add256(18443221601080337453n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18443221601080337441n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint16, euint16) => ebool test 1 (19952, 36928)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add16(19952n);
    input.add16(36928n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint16_euint16(
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

  it('test operator "lt" overload (euint16, euint16) => ebool test 2 (19948, 19952)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add16(19948n);
    input.add16(19952n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint16_euint16(
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

  it('test operator "lt" overload (euint16, euint16) => ebool test 3 (19952, 19952)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add16(19952n);
    input.add16(19952n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint16_euint16(
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

  it('test operator "lt" overload (euint16, euint16) => ebool test 4 (19952, 19948)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add16(19952n);
    input.add16(19948n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint16_euint16(
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

  it('test operator "or" overload (uint256, euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457578723714675993293, 115792089237316195423570985008687907853269984665640564039457578124041847683471)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457578124041847683471n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457578723714675993293n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457579287361794079695n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (uint256, euint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457575017621057149735, 115792089237316195423570985008687907853269984665640564039457575017621057149739)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457575017621057149739n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457575017621057149735n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457575017621057149743n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (uint256, euint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457575017621057149739, 115792089237316195423570985008687907853269984665640564039457575017621057149739)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457575017621057149739n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457575017621057149739n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457575017621057149739n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (uint256, euint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457575017621057149739, 115792089237316195423570985008687907853269984665640564039457575017621057149735)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457575017621057149735n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457575017621057149739n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457575017621057149743n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint128, uint128) => ebool test 1 (340282366920938463463369745012331754173, 340282366920938463463368799249509144775)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add128(340282366920938463463369745012331754173n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368799249509144775n,
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

  it('test operator "ne" overload (euint128, uint128) => ebool test 2 (340282366920938463463369745012331754169, 340282366920938463463369745012331754173)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add128(340282366920938463463369745012331754169n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463369745012331754173n,
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

  it('test operator "ne" overload (euint128, uint128) => ebool test 3 (340282366920938463463369745012331754173, 340282366920938463463369745012331754173)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add128(340282366920938463463369745012331754173n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463369745012331754173n,
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

  it('test operator "ne" overload (euint128, uint128) => ebool test 4 (340282366920938463463369745012331754173, 340282366920938463463369745012331754169)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add128(340282366920938463463369745012331754173n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463369745012331754169n,
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

  it('test operator "or" overload (euint256, euint32) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457583021724038724323, 1712259918)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457583021724038724323n);
    input.add32(1712259918n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457583021724072410095n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint256, euint32) => euint256 test 2 (1712259914, 1712259918)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add256(1712259914n);
    input.add32(1712259918n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1712259918n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint256, euint32) => euint256 test 3 (1712259918, 1712259918)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add256(1712259918n);
    input.add32(1712259918n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1712259918n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint256, euint32) => euint256 test 4 (1712259918, 1712259914)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add256(1712259918n);
    input.add32(1712259914n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1712259918n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint128, euint128) => ebool test 1 (340282366920938463463368791044669707091, 340282366920938463463372300731751037827)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add128(340282366920938463463368791044669707091n);
    input.add128(340282366920938463463372300731751037827n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint128_euint128(
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

  it('test operator "eq" overload (euint128, euint128) => ebool test 2 (340282366920938463463368791044669707087, 340282366920938463463368791044669707091)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add128(340282366920938463463368791044669707087n);
    input.add128(340282366920938463463368791044669707091n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint128_euint128(
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

  it('test operator "eq" overload (euint128, euint128) => ebool test 3 (340282366920938463463368791044669707091, 340282366920938463463368791044669707091)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add128(340282366920938463463368791044669707091n);
    input.add128(340282366920938463463368791044669707091n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint128_euint128(
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

  it('test operator "eq" overload (euint128, euint128) => ebool test 4 (340282366920938463463368791044669707091, 340282366920938463463368791044669707087)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add128(340282366920938463463368791044669707091n);
    input.add128(340282366920938463463368791044669707087n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint128_euint128(
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
});
