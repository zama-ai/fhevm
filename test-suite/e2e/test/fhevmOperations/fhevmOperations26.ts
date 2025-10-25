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

  it('test operator "add" overload (euint32, euint8) => euint32 test 1 (201, 2)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(201n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 203n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 2 (95, 97)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(95n);
    input.add8(97n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 192n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 3 (97, 97)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(97n);
    input.add8(97n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 194n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 4 (97, 95)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(97n);
    input.add8(95n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 192n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint32, euint8) => euint32 test 1 (60, 60)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(60n);
    input.add8(60n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint32, euint8) => euint32 test 2 (60, 56)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(60n);
    input.add8(56n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 1 (83, 2)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(83n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 166n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 2 (14, 14)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(14n);
    input.add8(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 196n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 3 (14, 14)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(14n);
    input.add8(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 196n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 4 (14, 14)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(14n);
    input.add8(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 196n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 1 (2799313384, 73)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(2799313384n);
    input.add8(73n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 72n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 2 (69, 73)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(69n);
    input.add8(73n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 65n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 3 (73, 73)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(73n);
    input.add8(73n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 73n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 4 (73, 69)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(73n);
    input.add8(69n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 65n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 1 (710719662, 181)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(710719662n);
    input.add8(181n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 710719679n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 2 (177, 181)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(177n);
    input.add8(181n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 181n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 3 (181, 181)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(181n);
    input.add8(181n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 181n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 4 (181, 177)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(181n);
    input.add8(177n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 181n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 1 (1288312961, 171)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(1288312961n);
    input.add8(171n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1288312874n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 2 (167, 171)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(167n);
    input.add8(171n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 12n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 3 (171, 171)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(171n);
    input.add8(171n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 4 (171, 167)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(171n);
    input.add8(167n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 12n,
    };
    assert.deepEqual(res, expectedRes);
  });
});
