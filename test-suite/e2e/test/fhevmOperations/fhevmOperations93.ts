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

describe('FHEVM operations 93', function () {
  before(async function () {
    this.signer = await getSigner(93);

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

  it('test operator "min" overload (uint128, euint128) => euint128 test 1 (340282366920938463463366529078120732589, 340282366920938463463368561917409048927)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);

    input.add128(340282366920938463463368561917409048927n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_uint128_euint128(
      340282366920938463463366529078120732589n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463366529078120732589n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (uint128, euint128) => euint128 test 2 (340282366920938463463371780590875218171, 340282366920938463463371780590875218175)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);

    input.add128(340282366920938463463371780590875218175n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_uint128_euint128(
      340282366920938463463371780590875218171n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463371780590875218171n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (uint128, euint128) => euint128 test 3 (340282366920938463463371780590875218175, 340282366920938463463371780590875218175)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);

    input.add128(340282366920938463463371780590875218175n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_uint128_euint128(
      340282366920938463463371780590875218175n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463371780590875218175n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (uint128, euint128) => euint128 test 4 (340282366920938463463371780590875218175, 340282366920938463463371780590875218171)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);

    input.add128(340282366920938463463371780590875218171n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_uint128_euint128(
      340282366920938463463371780590875218175n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463371780590875218171n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint128, uint128) => euint128 test 1 (340282366920938463463367474228925970149, 340282366920938463463368463750923395115)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add128(340282366920938463463367474228925970149n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368463750923395115n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463368463750923395115n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint128, uint128) => euint128 test 2 (340282366920938463463367382444010695717, 340282366920938463463367382444010695721)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add128(340282366920938463463367382444010695717n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463367382444010695721n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463367382444010695721n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint128, uint128) => euint128 test 3 (340282366920938463463367382444010695721, 340282366920938463463367382444010695721)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add128(340282366920938463463367382444010695721n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463367382444010695721n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463367382444010695721n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint128, uint128) => euint128 test 4 (340282366920938463463367382444010695721, 340282366920938463463367382444010695717)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add128(340282366920938463463367382444010695721n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463367382444010695717n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463367382444010695721n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (uint128, euint128) => euint128 test 1 (340282366920938463463373007535719097705, 340282366920938463463368463750923395115)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);

    input.add128(340282366920938463463368463750923395115n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_uint128_euint128(
      340282366920938463463373007535719097705n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463373007535719097705n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (uint128, euint128) => euint128 test 2 (340282366920938463463367382444010695717, 340282366920938463463367382444010695721)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);

    input.add128(340282366920938463463367382444010695721n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_uint128_euint128(
      340282366920938463463367382444010695717n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463367382444010695721n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (uint128, euint128) => euint128 test 3 (340282366920938463463367382444010695721, 340282366920938463463367382444010695721)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);

    input.add128(340282366920938463463367382444010695721n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_uint128_euint128(
      340282366920938463463367382444010695721n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463367382444010695721n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (uint128, euint128) => euint128 test 4 (340282366920938463463367382444010695721, 340282366920938463463367382444010695717)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);

    input.add128(340282366920938463463367382444010695717n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_uint128_euint128(
      340282366920938463463367382444010695721n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463367382444010695721n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint256, uint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457580727388371776955, 115792089237316195423570985008687907853269984665640564039457581513258294236641)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580727388371776955n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457581513258294236641n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457580630218914157985n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint256, uint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457580727388371776951, 115792089237316195423570985008687907853269984665640564039457580727388371776955)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580727388371776951n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457580727388371776955n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457580727388371776947n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint256, uint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457580727388371776955, 115792089237316195423570985008687907853269984665640564039457580727388371776955)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580727388371776955n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457580727388371776955n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457580727388371776955n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint256, uint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457580727388371776955, 115792089237316195423570985008687907853269984665640564039457580727388371776951)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580727388371776955n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457580727388371776951n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457580727388371776947n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (uint256, euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457576386541072865835, 115792089237316195423570985008687907853269984665640564039457581513258294236641)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457581513258294236641n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457576386541072865835n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457576161828362440737n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (uint256, euint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457580727388371776951, 115792089237316195423570985008687907853269984665640564039457580727388371776955)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457580727388371776955n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457580727388371776951n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457580727388371776947n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (uint256, euint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457580727388371776955, 115792089237316195423570985008687907853269984665640564039457580727388371776955)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457580727388371776955n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457580727388371776955n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457580727388371776955n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (uint256, euint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457580727388371776955, 115792089237316195423570985008687907853269984665640564039457580727388371776951)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457580727388371776951n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457580727388371776955n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457580727388371776947n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint256, uint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457576582324561149931, 115792089237316195423570985008687907853269984665640564039457577588921391832383)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576582324561149931n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457577588921391832383n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457578870958023180287n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint256, uint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457576582324561149927, 115792089237316195423570985008687907853269984665640564039457576582324561149931)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576582324561149927n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457576582324561149931n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457576582324561149935n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint256, uint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457576582324561149931, 115792089237316195423570985008687907853269984665640564039457576582324561149931)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576582324561149931n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457576582324561149931n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457576582324561149931n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint256, uint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457576582324561149931, 115792089237316195423570985008687907853269984665640564039457576582324561149927)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576582324561149931n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457576582324561149927n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457576582324561149935n,
    };
    assert.deepEqual(res, expectedRes);
  });
});
