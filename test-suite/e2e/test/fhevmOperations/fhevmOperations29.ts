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

describe('FHEVM operations 29', function () {
  before(async function () {
    this.signer = await getSigner(29);

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

  it('test operator "or" overload (euint128, euint128) => euint128 test 1 (340282366920938463463373559624660537169, 340282366920938463463370372732293038463)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add128(340282366920938463463373559624660537169n);
    input.add128(340282366920938463463370372732293038463n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463373753757484306303n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint128, euint128) => euint128 test 2 (340282366920938463463370372732293038459, 340282366920938463463370372732293038463)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add128(340282366920938463463370372732293038459n);
    input.add128(340282366920938463463370372732293038463n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463370372732293038463n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint128, euint128) => euint128 test 3 (340282366920938463463370372732293038463, 340282366920938463463370372732293038463)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add128(340282366920938463463370372732293038463n);
    input.add128(340282366920938463463370372732293038463n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463370372732293038463n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint128, euint128) => euint128 test 4 (340282366920938463463370372732293038463, 340282366920938463463370372732293038459)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add128(340282366920938463463370372732293038463n);
    input.add128(340282366920938463463370372732293038459n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463370372732293038463n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint8, euint64) => euint64 test 1 (108, 108)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add8(108n);
    input.add64(108n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint8_euint64(
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

  it('test operator "sub" overload (euint8, euint64) => euint64 test 2 (108, 104)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add8(108n);
    input.add64(104n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint8_euint64(
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

  it('test operator "or" overload (euint32, euint128) => euint128 test 1 (4212208207, 340282366920938463463373681532840068623)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(4212208207n);
    input.add128(340282366920938463463373681532840068623n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463373681535525549647n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint32, euint128) => euint128 test 2 (4212208203, 4212208207)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(4212208203n);
    input.add128(4212208207n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4212208207n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint32, euint128) => euint128 test 3 (4212208207, 4212208207)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(4212208207n);
    input.add128(4212208207n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4212208207n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint32, euint128) => euint128 test 4 (4212208207, 4212208203)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(4212208207n);
    input.add128(4212208203n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4212208207n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 1 (4294661332, 4294842257)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add64(4294661332n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint64_uint64(
      encryptedAmount.handles[0],
      4294842257n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18444892968177506324n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 2 (4294475736, 4294475736)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add64(4294475736n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint64_uint64(
      encryptedAmount.handles[0],
      4294475736n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18442521847092741696n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 3 (4294475736, 4294475736)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add64(4294475736n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint64_uint64(
      encryptedAmount.handles[0],
      4294475736n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18442521847092741696n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 4 (4294475736, 4294475736)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add64(4294475736n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint64_uint64(
      encryptedAmount.handles[0],
      4294475736n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18442521847092741696n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 1 (18443903610248017297, 18437816847096639195)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add64(18443903610248017297n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint64_uint64(
      encryptedAmount.handles[0],
      18437816847096639195n,
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

  it('test operator "le" overload (euint64, uint64) => ebool test 2 (18443903610248017293, 18443903610248017297)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add64(18443903610248017293n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint64_uint64(
      encryptedAmount.handles[0],
      18443903610248017297n,
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

  it('test operator "le" overload (euint64, uint64) => ebool test 3 (18443903610248017297, 18443903610248017297)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add64(18443903610248017297n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint64_uint64(
      encryptedAmount.handles[0],
      18443903610248017297n,
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

  it('test operator "le" overload (euint64, uint64) => ebool test 4 (18443903610248017297, 18443903610248017293)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add64(18443903610248017297n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint64_uint64(
      encryptedAmount.handles[0],
      18443903610248017293n,
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

  it('test operator "min" overload (euint64, euint8) => euint64 test 1 (18442042172416344751, 212)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add64(18442042172416344751n);
    input.add8(212n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 212n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint64, euint8) => euint64 test 2 (208, 212)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add64(208n);
    input.add8(212n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 208n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint64, euint8) => euint64 test 3 (212, 212)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add64(212n);
    input.add8(212n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 212n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint64, euint8) => euint64 test 4 (212, 208)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add64(212n);
    input.add8(208n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 208n,
    };
    assert.deepEqual(res, expectedRes);
  });
});
