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

describe('FHEVM operations 21', function () {
  before(async function () {
    this.signer = await getSigner(21);

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

  it('test operator "and" overload (euint16, euint64) => euint64 test 1 (43322, 18442679401548286127)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(43322n);
    input.add64(18442679401548286127n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2090n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint16, euint64) => euint64 test 2 (43318, 43322)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(43318n);
    input.add64(43322n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 43314n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint16, euint64) => euint64 test 3 (43322, 43322)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(43322n);
    input.add64(43322n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 43322n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint16, euint64) => euint64 test 4 (43322, 43318)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(43322n);
    input.add64(43318n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 43314n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint16, euint64) => euint64 test 1 (35521, 18440535816282915013)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(35521n);
    input.add64(18440535816282915013n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18440535816282950341n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint16, euint64) => euint64 test 2 (35517, 35521)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(35517n);
    input.add64(35521n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 35581n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint16, euint64) => euint64 test 3 (35521, 35521)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(35521n);
    input.add64(35521n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 35521n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint16, euint64) => euint64 test 4 (35521, 35517)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(35521n);
    input.add64(35517n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 35581n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint16, euint64) => euint64 test 1 (47227, 18440501421641543975)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(47227n);
    input.add64(18440501421641543975n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18440501421641501020n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint16, euint64) => euint64 test 2 (47223, 47227)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(47223n);
    input.add64(47227n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 12n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint16, euint64) => euint64 test 3 (47227, 47227)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(47227n);
    input.add64(47227n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint64(
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

  it('test operator "xor" overload (euint16, euint64) => euint64 test 4 (47227, 47223)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(47227n);
    input.add64(47223n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 12n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint16, euint64) => ebool test 1 (7070, 18445861434780263909)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(7070n);
    input.add64(18445861434780263909n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint64(
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

  it('test operator "eq" overload (euint16, euint64) => ebool test 2 (7066, 7070)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(7066n);
    input.add64(7070n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint64(
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

  it('test operator "eq" overload (euint16, euint64) => ebool test 3 (7070, 7070)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(7070n);
    input.add64(7070n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint64(
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

  it('test operator "eq" overload (euint16, euint64) => ebool test 4 (7070, 7066)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(7070n);
    input.add64(7066n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint64(
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

  it('test operator "ne" overload (euint16, euint64) => ebool test 1 (44133, 18442008324483527921)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(44133n);
    input.add64(18442008324483527921n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint64(
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

  it('test operator "ne" overload (euint16, euint64) => ebool test 2 (44129, 44133)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(44129n);
    input.add64(44133n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint64(
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

  it('test operator "ne" overload (euint16, euint64) => ebool test 3 (44133, 44133)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(44133n);
    input.add64(44133n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint64(
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

  it('test operator "ne" overload (euint16, euint64) => ebool test 4 (44133, 44129)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(44133n);
    input.add64(44129n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint64(
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

  it('test operator "ge" overload (euint16, euint64) => ebool test 1 (47018, 18440001367559116187)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(47018n);
    input.add64(18440001367559116187n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint64(
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

  it('test operator "ge" overload (euint16, euint64) => ebool test 2 (47014, 47018)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(47014n);
    input.add64(47018n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint64(
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

  it('test operator "ge" overload (euint16, euint64) => ebool test 3 (47018, 47018)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(47018n);
    input.add64(47018n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint64(
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

  it('test operator "ge" overload (euint16, euint64) => ebool test 4 (47018, 47014)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(47018n);
    input.add64(47014n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint64(
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
});
