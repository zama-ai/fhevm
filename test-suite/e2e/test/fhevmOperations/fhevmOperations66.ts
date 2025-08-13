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

  it('test operator "max" overload (euint8, euint16) => euint16 test 1 (109, 64828)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add8(109n);
    input.add16(64828n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 64828n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint8, euint16) => euint16 test 2 (105, 109)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add8(105n);
    input.add16(109n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 109n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint8, euint16) => euint16 test 3 (109, 109)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add8(109n);
    input.add16(109n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 109n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint8, euint16) => euint16 test 4 (109, 105)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add8(109n);
    input.add16(105n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 109n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 1 (58221, 1987878670)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add16(58221n);
    input.add32(1987878670n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1987903343n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 2 (58217, 58221)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add16(58217n);
    input.add32(58221n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 58221n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 3 (58221, 58221)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add16(58221n);
    input.add32(58221n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 58221n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 4 (58221, 58217)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add16(58221n);
    input.add32(58217n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 58221n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint128, euint64) => euint128 test 1 (340282366920938463463368101667533107725, 18442568670729516211)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add128(340282366920938463463368101667533107725n);
    input.add64(18442568670729516211n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463444935121734766593726n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint128, euint64) => euint128 test 2 (18442568670729516207, 18442568670729516211)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add128(18442568670729516207n);
    input.add64(18442568670729516211n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 28n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint128, euint64) => euint128 test 3 (18442568670729516211, 18442568670729516211)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add128(18442568670729516211n);
    input.add64(18442568670729516211n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint128, euint64) => euint128 test 4 (18442568670729516211, 18442568670729516207)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add128(18442568670729516211n);
    input.add64(18442568670729516207n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 28n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint16, euint64) => euint64 test 1 (28001, 18444416668198332021)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add16(28001n);
    input.add64(18444416668198332021n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18444416668198308628n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint16, euint64) => euint64 test 2 (27997, 28001)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add16(27997n);
    input.add64(28001n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 60n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint16, euint64) => euint64 test 3 (28001, 28001)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add16(28001n);
    input.add64(28001n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint16, euint64) => euint64 test 4 (28001, 27997)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add16(28001n);
    input.add64(27997n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 60n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 1 (18440429597721625533, 2668360404)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add64(18440429597721625533n);
    input.add32(2668360404n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18440429597721625533n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 2 (2668360400, 2668360404)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add64(2668360400n);
    input.add32(2668360404n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2668360404n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 3 (2668360404, 2668360404)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add64(2668360404n);
    input.add32(2668360404n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2668360404n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 4 (2668360404, 2668360400)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add64(2668360404n);
    input.add32(2668360400n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2668360404n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint256, uint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457577485992357038303, 11)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577485992357038303n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotl_euint256_uint8(encryptedAmount.handles[0], 11n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract5.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039444227114170841497599n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint256, uint8) => euint256 test 2 (7, 11)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add256(7n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotl_euint256_uint8(encryptedAmount.handles[0], 11n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract5.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 14336n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint256, uint8) => euint256 test 3 (11, 11)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add256(11n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotl_euint256_uint8(encryptedAmount.handles[0], 11n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract5.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 22528n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint256, uint8) => euint256 test 4 (11, 7)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add256(11n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotl_euint256_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract5.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1408n,
    };
    assert.deepEqual(res, expectedRes);
  });
});
