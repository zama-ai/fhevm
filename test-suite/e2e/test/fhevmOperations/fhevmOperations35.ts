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

describe('FHEVM operations 35', function () {
  before(async function () {
    this.signer = await getSigner(35);

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

  it('test operator "mul" overload (euint32, euint128) => euint128 test 1 (2, 1073741825)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add32(2n);
    input.add128(1073741825n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2147483650n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint32, euint128) => euint128 test 2 (52474, 52474)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add32(52474n);
    input.add128(52474n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2753520676n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint32, euint128) => euint128 test 3 (52474, 52474)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add32(52474n);
    input.add128(52474n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2753520676n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint32, euint128) => euint128 test 4 (52474, 52474)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add32(52474n);
    input.add128(52474n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2753520676n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 1 (18440508658271461895, 18440408966720301669)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add64(18440508658271461895n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.div_euint64_uint64(
      encryptedAmount.handles[0],
      18440408966720301669n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 2 (18440508658271461891, 18440508658271461895)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add64(18440508658271461891n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.div_euint64_uint64(
      encryptedAmount.handles[0],
      18440508658271461895n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 3 (18440508658271461895, 18440508658271461895)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add64(18440508658271461895n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.div_euint64_uint64(
      encryptedAmount.handles[0],
      18440508658271461895n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 4 (18440508658271461895, 18440508658271461891)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add64(18440508658271461895n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.div_euint64_uint64(
      encryptedAmount.handles[0],
      18440508658271461891n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint128, euint8) => euint128 test 1 (340282366920938463463368072661735701933, 14)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add128(340282366920938463463368072661735701933n);
    input.add8(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463368072661735701935n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint128, euint8) => euint128 test 2 (10, 14)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add128(10n);
    input.add8(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 14n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint128, euint8) => euint128 test 3 (14, 14)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add128(14n);
    input.add8(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 14n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint128, euint8) => euint128 test 4 (14, 10)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add128(14n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 14n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint8, uint8) => ebool test 1 (208, 234)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(208n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint8_uint8(encryptedAmount.handles[0], 234n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint8, uint8) => ebool test 2 (114, 118)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(114n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint8_uint8(encryptedAmount.handles[0], 118n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint8, uint8) => ebool test 3 (118, 118)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(118n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint8_uint8(encryptedAmount.handles[0], 118n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint8, uint8) => ebool test 4 (118, 114)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(118n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint8_uint8(encryptedAmount.handles[0], 114n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint128, euint8) => euint128 test 1 (99, 99)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add128(99n);
    input.add8(99n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint128, euint8) => euint128 test 2 (99, 95)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add128(99n);
    input.add8(95n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint8, euint64) => euint64 test 1 (52, 18438001796120362459)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(52n);
    input.add64(18438001796120362459n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18438001796120362479n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint8, euint64) => euint64 test 2 (48, 52)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(48n);
    input.add64(52n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint8, euint64) => euint64 test 3 (52, 52)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(52n);
    input.add64(52n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint8, euint64) => euint64 test 4 (52, 48)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(52n);
    input.add64(48n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });
});
