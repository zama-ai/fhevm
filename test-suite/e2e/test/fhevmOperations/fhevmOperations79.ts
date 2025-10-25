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

describe('FHEVM operations 79', function () {
  before(async function () {
    this.signer = await getSigner(79);

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

  it('test operator "sub" overload (uint32, euint32) => euint32 test 1 (2271219408, 2271219408)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add32(2271219408n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_uint32_euint32(
      2271219408n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (uint32, euint32) => euint32 test 2 (2271219408, 2271219404)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add32(2271219404n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_uint32_euint32(
      2271219408n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 1 (62163, 40446)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add32(62163n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint32_uint32(encryptedAmount.handles[0], 40446n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2514244698n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 2 (57514, 57514)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add32(57514n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint32_uint32(encryptedAmount.handles[0], 57514n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3307860196n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 3 (57514, 57514)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add32(57514n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint32_uint32(encryptedAmount.handles[0], 57514n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3307860196n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 4 (57514, 57514)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add32(57514n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint32_uint32(encryptedAmount.handles[0], 57514n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3307860196n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 1 (45685, 40446)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add32(40446n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_uint32_euint32(45685n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1847775510n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 2 (57514, 57514)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add32(57514n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_uint32_euint32(57514n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3307860196n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 3 (57514, 57514)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add32(57514n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_uint32_euint32(57514n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3307860196n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 4 (57514, 57514)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add32(57514n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_uint32_euint32(57514n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3307860196n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 1 (1691714054, 2857364901)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add32(1691714054n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.div_euint32_uint32(
      encryptedAmount.handles[0],
      2857364901n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 2 (486270740, 486270744)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add32(486270740n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.div_euint32_uint32(
      encryptedAmount.handles[0],
      486270744n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 3 (486270744, 486270744)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add32(486270744n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.div_euint32_uint32(
      encryptedAmount.handles[0],
      486270744n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 4 (486270744, 486270740)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add32(486270744n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.div_euint32_uint32(
      encryptedAmount.handles[0],
      486270740n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 1 (924792227, 1960452484)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add32(924792227n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rem_euint32_uint32(
      encryptedAmount.handles[0],
      1960452484n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 924792227n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 2 (924792223, 924792227)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add32(924792223n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rem_euint32_uint32(
      encryptedAmount.handles[0],
      924792227n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 924792223n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 3 (924792227, 924792227)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add32(924792227n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rem_euint32_uint32(
      encryptedAmount.handles[0],
      924792227n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 4 (924792227, 924792223)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add32(924792227n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rem_euint32_uint32(
      encryptedAmount.handles[0],
      924792223n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint32, uint32) => euint32 test 1 (2802423601, 872226176)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add32(2802423601n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint32_uint32(
      encryptedAmount.handles[0],
      872226176n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 587797760n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint32, uint32) => euint32 test 2 (722020502, 722020506)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add32(722020502n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint32_uint32(
      encryptedAmount.handles[0],
      722020506n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 722020498n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint32, uint32) => euint32 test 3 (722020506, 722020506)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add32(722020506n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint32_uint32(
      encryptedAmount.handles[0],
      722020506n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 722020506n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint32, uint32) => euint32 test 4 (722020506, 722020502)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add32(722020506n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint32_uint32(
      encryptedAmount.handles[0],
      722020502n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 722020498n,
    };
    assert.deepEqual(res, expectedRes);
  });
});
