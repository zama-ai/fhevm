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

describe('FHEVM operations 60', function () {
  before(async function () {
    this.signer = await getSigner(60);

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

  it('test operator "min" overload (euint128, euint64) => euint128 test 1 (340282366920938463463367468006661140089, 18438522459626149825)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(340282366920938463463367468006661140089n);
    input.add64(18438522459626149825n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18438522459626149825n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint128, euint64) => euint128 test 2 (18438522459626149821, 18438522459626149825)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(18438522459626149821n);
    input.add64(18438522459626149825n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18438522459626149821n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint128, euint64) => euint128 test 3 (18438522459626149825, 18438522459626149825)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(18438522459626149825n);
    input.add64(18438522459626149825n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18438522459626149825n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint128, euint64) => euint128 test 4 (18438522459626149825, 18438522459626149821)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(18438522459626149825n);
    input.add64(18438522459626149821n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18438522459626149821n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint128, euint64) => euint128 test 1 (340282366920938463463373186356942280159, 18444886943995068945)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(340282366920938463463373186356942280159n);
    input.add64(18444886943995068945n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463373186356942280159n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint128, euint64) => euint128 test 2 (18444886943995068941, 18444886943995068945)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(18444886943995068941n);
    input.add64(18444886943995068945n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18444886943995068945n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint128, euint64) => euint128 test 3 (18444886943995068945, 18444886943995068945)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(18444886943995068945n);
    input.add64(18444886943995068945n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18444886943995068945n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint128, euint64) => euint128 test 4 (18444886943995068945, 18444886943995068941)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(18444886943995068945n);
    input.add64(18444886943995068941n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18444886943995068945n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint128, euint128) => euint128 test 1 (170141183460469231731687272015842765950, 170141183460469231731686444035462849345)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(170141183460469231731687272015842765950n);
    input.add128(170141183460469231731686444035462849345n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463373716051305615295n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint128, euint128) => euint128 test 2 (170141183460469231731686444035462849343, 170141183460469231731686444035462849345)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(170141183460469231731686444035462849343n);
    input.add128(170141183460469231731686444035462849345n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463372888070925698688n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint128, euint128) => euint128 test 3 (170141183460469231731686444035462849345, 170141183460469231731686444035462849345)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(170141183460469231731686444035462849345n);
    input.add128(170141183460469231731686444035462849345n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463372888070925698690n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint128, euint128) => euint128 test 4 (170141183460469231731686444035462849345, 170141183460469231731686444035462849343)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(170141183460469231731686444035462849345n);
    input.add128(170141183460469231731686444035462849343n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463372888070925698688n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint128, euint128) => euint128 test 1 (340282366920938463463365918437382145247, 340282366920938463463365918437382145247)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(340282366920938463463365918437382145247n);
    input.add128(340282366920938463463365918437382145247n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint128, euint128) => euint128 test 2 (340282366920938463463365918437382145247, 340282366920938463463365918437382145243)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(340282366920938463463365918437382145247n);
    input.add128(340282366920938463463365918437382145243n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint128, euint128) => euint128 test 1 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(9223372036854775809n);
    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 85070591730234615884290395931651604481n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint128, euint128) => euint128 test 2 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(9223372036854775809n);
    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 85070591730234615884290395931651604481n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint128, euint128) => euint128 test 3 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(9223372036854775809n);
    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 85070591730234615884290395931651604481n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint128, euint128) => euint128 test 4 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(9223372036854775809n);
    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 85070591730234615884290395931651604481n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint128, euint128) => euint128 test 1 (340282366920938463463369620863023408927, 340282366920938463463374566092192076539)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(340282366920938463463369620863023408927n);
    input.add128(340282366920938463463374566092192076539n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463369620205419446811n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint128, euint128) => euint128 test 2 (340282366920938463463369620863023408923, 340282366920938463463369620863023408927)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(340282366920938463463369620863023408923n);
    input.add128(340282366920938463463369620863023408927n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463369620863023408923n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint128, euint128) => euint128 test 3 (340282366920938463463369620863023408927, 340282366920938463463369620863023408927)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(340282366920938463463369620863023408927n);
    input.add128(340282366920938463463369620863023408927n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463369620863023408927n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint128, euint128) => euint128 test 4 (340282366920938463463369620863023408927, 340282366920938463463369620863023408923)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(340282366920938463463369620863023408927n);
    input.add128(340282366920938463463369620863023408923n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463369620863023408923n,
    };
    assert.deepEqual(res, expectedRes);
  });
});
