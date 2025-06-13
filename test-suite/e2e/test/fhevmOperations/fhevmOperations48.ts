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

describe('FHEVM operations 48', function () {
  before(async function () {
    this.signer = await getSigner(48);

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

  it('test operator "and" overload (euint256, uint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457580596510218340189, 115792089237316195423570985008687907853269984665640564039457577095229406311417)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580596510218340189n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457577095229406311417n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457575951036660138841n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint256, uint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457577126449691216897, 115792089237316195423570985008687907853269984665640564039457577126449691216901)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577126449691216897n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457577126449691216901n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457577126449691216897n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint256, uint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457577126449691216901, 115792089237316195423570985008687907853269984665640564039457577126449691216901)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577126449691216901n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457577126449691216901n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457577126449691216901n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint256, uint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457577126449691216901, 115792089237316195423570985008687907853269984665640564039457577126449691216897)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577126449691216901n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457577126449691216897n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457577126449691216897n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 1 (18444710445516952473, 18438342655451204343)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);

    input.add64(18438342655451204343n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_uint64_euint64(
      18444710445516952473n,
      encryptedAmount.handles[0],
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

  it('test operator "ne" overload (uint64, euint64) => ebool test 2 (18441316501920671163, 18441316501920671167)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);

    input.add64(18441316501920671167n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_uint64_euint64(
      18441316501920671163n,
      encryptedAmount.handles[0],
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

  it('test operator "ne" overload (uint64, euint64) => ebool test 3 (18441316501920671167, 18441316501920671167)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);

    input.add64(18441316501920671167n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_uint64_euint64(
      18441316501920671167n,
      encryptedAmount.handles[0],
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

  it('test operator "ne" overload (uint64, euint64) => ebool test 4 (18441316501920671167, 18441316501920671163)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);

    input.add64(18441316501920671163n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_uint64_euint64(
      18441316501920671167n,
      encryptedAmount.handles[0],
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

  it('test operator "sub" overload (euint64, euint32) => euint64 test 1 (209562563, 209562563)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add64(209562563n);
    input.add32(209562563n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint64, euint32) => euint64 test 2 (209562563, 209562559)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add64(209562563n);
    input.add32(209562559n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint8, euint16) => euint16 test 1 (172, 36131)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add8(172n);
    input.add16(36131n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 32n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint8, euint16) => euint16 test 2 (168, 172)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add8(168n);
    input.add16(172n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 168n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint8, euint16) => euint16 test 3 (172, 172)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add8(172n);
    input.add16(172n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 172n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint8, euint16) => euint16 test 4 (172, 168)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add8(172n);
    input.add16(168n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 168n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (uint256, euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457576083944460858451, 115792089237316195423570985008687907853269984665640564039457577095229406311417)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457577095229406311417n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457576083944460858451n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457575933311363657809n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (uint256, euint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457577126449691216897, 115792089237316195423570985008687907853269984665640564039457577126449691216901)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457577126449691216901n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457577126449691216897n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457577126449691216897n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (uint256, euint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457577126449691216901, 115792089237316195423570985008687907853269984665640564039457577126449691216901)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457577126449691216901n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457577126449691216901n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457577126449691216901n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (uint256, euint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457577126449691216901, 115792089237316195423570985008687907853269984665640564039457577126449691216897)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457577126449691216897n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457577126449691216901n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457577126449691216897n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "neg" overload (euint128) => euint128 test 1 (340282366920938463463369413354541547957)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(340282366920938463463369413354541547957n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.neg_euint128(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 5194077226663499n,
    };
    assert.deepEqual(res, expectedRes);
  });
});
