import { HardhatEthersSigner } from '@nomicfoundation/hardhat-ethers/signers';
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

describe('FHEVM operations 102', function () {
  before(async function () {
    this.signer = await getSigner(102);

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

  it('test operator "xor" overload (euint64, euint256) => euint256 test 1 (18443357991836926003, 115792089237316195423570985008687907853269984665640564039457575821445683354907)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add64(18443357991836926003n);
    input.add256(115792089237316195423570985008687907853269984665640564039457575821445683354907n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039439142068790649117992n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "xor" overload (euint64, euint256) => euint256 test 2 (18443357991836925999, 18443357991836926003)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add64(18443357991836925999n);
    input.add256(18443357991836926003n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 28n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "xor" overload (euint64, euint256) => euint256 test 3 (18443357991836926003, 18443357991836926003)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add64(18443357991836926003n);
    input.add256(18443357991836926003n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "xor" overload (euint64, euint256) => euint256 test 4 (18443357991836926003, 18443357991836925999)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add64(18443357991836926003n);
    input.add256(18443357991836925999n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 28n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "ge" overload (euint64, euint128) => ebool test 1 (18439095146183369197, 340282366920938463463372031592158684153)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add64(18439095146183369197n);
    input.add128(340282366920938463463372031592158684153n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "ge" overload (euint64, euint128) => ebool test 2 (18439095146183369193, 18439095146183369197)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add64(18439095146183369193n);
    input.add128(18439095146183369197n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "ge" overload (euint64, euint128) => ebool test 3 (18439095146183369197, 18439095146183369197)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add64(18439095146183369197n);
    input.add128(18439095146183369197n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "ge" overload (euint64, euint128) => ebool test 4 (18439095146183369197, 18439095146183369193)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add64(18439095146183369197n);
    input.add128(18439095146183369193n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 1 (710719662, 181)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add32(710719662n);
    input.add8(181n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 710719679n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 2 (177, 181)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add32(177n);
    input.add8(181n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 181n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 3 (181, 181)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add32(181n);
    input.add8(181n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 181n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 4 (181, 177)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add32(181n);
    input.add8(177n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 181n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 1 (18446607332850591667, 18444669461336692671)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);

    input.add64(18444669461336692671n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_uint64_euint64(
      18446607332850591667n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 2 (18438850655314522435, 18438850655314522439)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);

    input.add64(18438850655314522439n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_uint64_euint64(
      18438850655314522435n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 3 (18438850655314522439, 18438850655314522439)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);

    input.add64(18438850655314522439n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_uint64_euint64(
      18438850655314522439n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 4 (18438850655314522439, 18438850655314522435)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);

    input.add64(18438850655314522435n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_uint64_euint64(
      18438850655314522439n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 1 (28986, 730827779)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add16(28986n);
    input.add32(730827779n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 2 (28982, 28986)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add16(28982n);
    input.add32(28986n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 28978n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 3 (28986, 28986)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add16(28986n);
    input.add32(28986n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 28986n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 4 (28986, 28982)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add16(28986n);
    input.add32(28982n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 28978n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 1 (2056964365, 2047425512)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add32(2056964365n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_euint32_uint32(
      encryptedAmount.handles[0],
      2047425512n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4104389877n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 2 (2056964361, 2056964365)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add32(2056964361n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_euint32_uint32(
      encryptedAmount.handles[0],
      2056964365n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4113928726n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 3 (2056964365, 2056964365)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add32(2056964365n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_euint32_uint32(
      encryptedAmount.handles[0],
      2056964365n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4113928730n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 4 (2056964365, 2056964361)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add32(2056964365n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_euint32_uint32(
      encryptedAmount.handles[0],
      2056964361n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4113928726n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });
});
