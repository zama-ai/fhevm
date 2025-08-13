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

describe('FHEVM operations 27', function () {
  before(async function () {
    this.signer = await getSigner(27);

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

  it('test operator "max" overload (uint32, euint32) => euint32 test 1 (768669539, 4228083879)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);

    input.add32(4228083879n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_uint32_euint32(
      768669539n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4228083879n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 2 (1627200456, 1627200460)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);

    input.add32(1627200460n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_uint32_euint32(
      1627200456n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1627200460n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 3 (1627200460, 1627200460)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);

    input.add32(1627200460n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_uint32_euint32(
      1627200460n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1627200460n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 4 (1627200460, 1627200456)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);

    input.add32(1627200456n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_uint32_euint32(
      1627200460n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1627200460n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 1 (239, 27)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add8(239n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint8_uint8(encryptedAmount.handles[0], 27n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract2.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 239n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 2 (80, 84)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add8(80n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint8_uint8(encryptedAmount.handles[0], 84n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract2.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 84n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 3 (84, 84)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add8(84n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint8_uint8(encryptedAmount.handles[0], 84n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract2.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 84n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 4 (84, 80)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add8(84n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint8_uint8(encryptedAmount.handles[0], 80n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract2.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 84n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint64, euint8) => euint64 test 1 (18442100042053626655, 9)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add64(18442100042053626655n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.shr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 36019726644635989n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint64, euint8) => euint64 test 2 (5, 9)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add64(5n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.shr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint64, euint8) => euint64 test 3 (9, 9)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add64(9n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.shr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint64, euint8) => euint64 test 4 (9, 5)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add64(9n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.shr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint256, euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457581964254966221319, 115792089237316195423570985008687907853269984665640564039457583522105885686945)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457581964254966221319n);
    input.add256(115792089237316195423570985008687907853269984665640564039457583522105885686945n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457581760550033950721n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint256, euint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457581964254966221315, 115792089237316195423570985008687907853269984665640564039457581964254966221319)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457581964254966221315n);
    input.add256(115792089237316195423570985008687907853269984665640564039457581964254966221319n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457581964254966221315n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint256, euint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457581964254966221319, 115792089237316195423570985008687907853269984665640564039457581964254966221319)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457581964254966221319n);
    input.add256(115792089237316195423570985008687907853269984665640564039457581964254966221319n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457581964254966221319n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint256, euint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457581964254966221319, 115792089237316195423570985008687907853269984665640564039457581964254966221315)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457581964254966221319n);
    input.add256(115792089237316195423570985008687907853269984665640564039457581964254966221315n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457581964254966221315n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint64, euint32) => euint64 test 1 (18444383458226034229, 624713525)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add64(18444383458226034229n);
    input.add32(624713525n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18444383457772243200n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint64, euint32) => euint64 test 2 (624713521, 624713525)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add64(624713521n);
    input.add32(624713525n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint64, euint32) => euint64 test 3 (624713525, 624713525)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add64(624713525n);
    input.add32(624713525n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint64, euint32) => euint64 test 4 (624713525, 624713521)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add64(624713525n);
    input.add32(624713521n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint32, euint256) => ebool test 1 (2349145921, 115792089237316195423570985008687907853269984665640564039457578644806288048555)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(2349145921n);
    input.add256(115792089237316195423570985008687907853269984665640564039457578644806288048555n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint32_euint256(
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
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint32, euint256) => ebool test 2 (2349145917, 2349145921)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(2349145917n);
    input.add256(2349145921n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint32_euint256(
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
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint32, euint256) => ebool test 3 (2349145921, 2349145921)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(2349145921n);
    input.add256(2349145921n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint32_euint256(
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
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint32, euint256) => ebool test 4 (2349145921, 2349145917)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(2349145921n);
    input.add256(2349145917n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint32_euint256(
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
    assert.deepEqual(res, expectedRes);
  });
});
