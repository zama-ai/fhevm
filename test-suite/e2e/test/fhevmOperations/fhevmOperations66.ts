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

describe('FHEVM operations 66', function () {
  before(async function () {
    this.signer = await getSigner(66);

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

  it('test operator "and" overload (euint256, euint64) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457576029112850821961, 18439189234875686269)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576029112850821961n);
    input.add64(18439189234875686269n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18438053817315529033n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint256, euint64) => euint256 test 2 (18439189234875686265, 18439189234875686269)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add256(18439189234875686265n);
    input.add64(18439189234875686269n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18439189234875686265n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint256, euint64) => euint256 test 3 (18439189234875686269, 18439189234875686269)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add256(18439189234875686269n);
    input.add64(18439189234875686269n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18439189234875686269n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint256, euint64) => euint256 test 4 (18439189234875686269, 18439189234875686265)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add256(18439189234875686269n);
    input.add64(18439189234875686265n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18439189234875686265n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint256, euint64) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457577215480610259831, 18441662610641197675)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577215480610259831n);
    input.add64(18441662610641197675n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457579502663142185855n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint256, euint64) => euint256 test 2 (18441662610641197671, 18441662610641197675)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add256(18441662610641197671n);
    input.add64(18441662610641197675n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18441662610641197679n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint256, euint64) => euint256 test 3 (18441662610641197675, 18441662610641197675)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add256(18441662610641197675n);
    input.add64(18441662610641197675n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18441662610641197675n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint256, euint64) => euint256 test 4 (18441662610641197675, 18441662610641197671)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add256(18441662610641197675n);
    input.add64(18441662610641197671n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18441662610641197679n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint256, euint64) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457575486637890877703, 18443967652537936061)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575486637890877703n);
    input.add64(18443967652537936061n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039439143908386058113466n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint256, euint64) => euint256 test 2 (18443967652537936057, 18443967652537936061)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add256(18443967652537936057n);
    input.add64(18443967652537936061n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint256, euint64) => euint256 test 3 (18443967652537936061, 18443967652537936061)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add256(18443967652537936061n);
    input.add64(18443967652537936061n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint256, euint64) => euint256 test 4 (18443967652537936061, 18443967652537936057)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add256(18443967652537936061n);
    input.add64(18443967652537936057n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint256, euint64) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457580793699504277477, 18439638370960042927)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580793699504277477n);
    input.add64(18439638370960042927n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint256_euint64(
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

  it('test operator "eq" overload (euint256, euint64) => ebool test 2 (18439638370960042923, 18439638370960042927)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add256(18439638370960042923n);
    input.add64(18439638370960042927n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint256_euint64(
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

  it('test operator "eq" overload (euint256, euint64) => ebool test 3 (18439638370960042927, 18439638370960042927)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add256(18439638370960042927n);
    input.add64(18439638370960042927n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint256_euint64(
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

  it('test operator "eq" overload (euint256, euint64) => ebool test 4 (18439638370960042927, 18439638370960042923)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add256(18439638370960042927n);
    input.add64(18439638370960042923n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint256_euint64(
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

  it('test operator "ne" overload (euint256, euint64) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457575420275373658947, 18440351101654254241)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575420275373658947n);
    input.add64(18440351101654254241n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint256_euint64(
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

  it('test operator "ne" overload (euint256, euint64) => ebool test 2 (18440351101654254237, 18440351101654254241)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add256(18440351101654254237n);
    input.add64(18440351101654254241n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint256_euint64(
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

  it('test operator "ne" overload (euint256, euint64) => ebool test 3 (18440351101654254241, 18440351101654254241)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add256(18440351101654254241n);
    input.add64(18440351101654254241n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint256_euint64(
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

  it('test operator "ne" overload (euint256, euint64) => ebool test 4 (18440351101654254241, 18440351101654254237)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add256(18440351101654254241n);
    input.add64(18440351101654254237n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint256_euint64(
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

  it('test operator "and" overload (euint256, euint128) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457578104479961759913, 340282366920938463463372582172919627471)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578104479961759913n);
    input.add128(340282366920938463463372582172919627471n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463367858670350123145n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint256, euint128) => euint256 test 2 (340282366920938463463372582172919627467, 340282366920938463463372582172919627471)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add256(340282366920938463463372582172919627467n);
    input.add128(340282366920938463463372582172919627471n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463372582172919627467n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint256, euint128) => euint256 test 3 (340282366920938463463372582172919627471, 340282366920938463463372582172919627471)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add256(340282366920938463463372582172919627471n);
    input.add128(340282366920938463463372582172919627471n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463372582172919627471n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint256, euint128) => euint256 test 4 (340282366920938463463372582172919627471, 340282366920938463463372582172919627467)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add256(340282366920938463463372582172919627471n);
    input.add128(340282366920938463463372582172919627467n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463372582172919627467n,
    };
    assert.deepEqual(res, expectedRes);
  });
});
