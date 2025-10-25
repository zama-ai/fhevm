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

describe('FHEVM operations 92', function () {
  before(async function () {
    this.signer = await getSigner(92);

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

  it('test operator "gt" overload (uint128, euint128) => ebool test 1 (340282366920938463463366301134497944171, 340282366920938463463372126018431653325)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);

    input.add128(340282366920938463463372126018431653325n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_uint128_euint128(
      340282366920938463463366301134497944171n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (uint128, euint128) => ebool test 2 (340282366920938463463369926229248711471, 340282366920938463463369926229248711475)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);

    input.add128(340282366920938463463369926229248711475n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_uint128_euint128(
      340282366920938463463369926229248711471n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (uint128, euint128) => ebool test 3 (340282366920938463463369926229248711475, 340282366920938463463369926229248711475)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);

    input.add128(340282366920938463463369926229248711475n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_uint128_euint128(
      340282366920938463463369926229248711475n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (uint128, euint128) => ebool test 4 (340282366920938463463369926229248711475, 340282366920938463463369926229248711471)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);

    input.add128(340282366920938463463369926229248711471n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_uint128_euint128(
      340282366920938463463369926229248711475n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint128, uint128) => ebool test 1 (340282366920938463463369431786062180615, 340282366920938463463373127578809249203)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add128(340282366920938463463369431786062180615n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463373127578809249203n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint128, uint128) => ebool test 2 (340282366920938463463367329768413053355, 340282366920938463463367329768413053359)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add128(340282366920938463463367329768413053355n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463367329768413053359n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint128, uint128) => ebool test 3 (340282366920938463463367329768413053359, 340282366920938463463367329768413053359)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add128(340282366920938463463367329768413053359n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463367329768413053359n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint128, uint128) => ebool test 4 (340282366920938463463367329768413053359, 340282366920938463463367329768413053355)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add128(340282366920938463463367329768413053359n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463367329768413053355n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (uint128, euint128) => ebool test 1 (340282366920938463463374517032478155835, 340282366920938463463373127578809249203)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);

    input.add128(340282366920938463463373127578809249203n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_uint128_euint128(
      340282366920938463463374517032478155835n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (uint128, euint128) => ebool test 2 (340282366920938463463367329768413053355, 340282366920938463463367329768413053359)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);

    input.add128(340282366920938463463367329768413053359n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_uint128_euint128(
      340282366920938463463367329768413053355n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (uint128, euint128) => ebool test 3 (340282366920938463463367329768413053359, 340282366920938463463367329768413053359)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);

    input.add128(340282366920938463463367329768413053359n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_uint128_euint128(
      340282366920938463463367329768413053359n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (uint128, euint128) => ebool test 4 (340282366920938463463367329768413053359, 340282366920938463463367329768413053355)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);

    input.add128(340282366920938463463367329768413053355n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_uint128_euint128(
      340282366920938463463367329768413053359n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint128, uint128) => ebool test 1 (340282366920938463463371647940600324849, 340282366920938463463374407153810090079)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add128(340282366920938463463371647940600324849n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463374407153810090079n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint128, uint128) => ebool test 2 (340282366920938463463371647940600324845, 340282366920938463463371647940600324849)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add128(340282366920938463463371647940600324845n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463371647940600324849n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint128, uint128) => ebool test 3 (340282366920938463463371647940600324849, 340282366920938463463371647940600324849)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add128(340282366920938463463371647940600324849n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463371647940600324849n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint128, uint128) => ebool test 4 (340282366920938463463371647940600324849, 340282366920938463463371647940600324845)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add128(340282366920938463463371647940600324849n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463371647940600324845n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (uint128, euint128) => ebool test 1 (340282366920938463463373443996970921113, 340282366920938463463374407153810090079)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);

    input.add128(340282366920938463463374407153810090079n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_uint128_euint128(
      340282366920938463463373443996970921113n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (uint128, euint128) => ebool test 2 (340282366920938463463371647940600324845, 340282366920938463463371647940600324849)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);

    input.add128(340282366920938463463371647940600324849n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_uint128_euint128(
      340282366920938463463371647940600324845n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (uint128, euint128) => ebool test 3 (340282366920938463463371647940600324849, 340282366920938463463371647940600324849)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);

    input.add128(340282366920938463463371647940600324849n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_uint128_euint128(
      340282366920938463463371647940600324849n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (uint128, euint128) => ebool test 4 (340282366920938463463371647940600324849, 340282366920938463463371647940600324845)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);

    input.add128(340282366920938463463371647940600324845n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_uint128_euint128(
      340282366920938463463371647940600324849n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint128, uint128) => euint128 test 1 (340282366920938463463374432859434778255, 340282366920938463463368561917409048927)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add128(340282366920938463463374432859434778255n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368561917409048927n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463368561917409048927n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint128, uint128) => euint128 test 2 (340282366920938463463371780590875218171, 340282366920938463463371780590875218175)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add128(340282366920938463463371780590875218171n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463371780590875218175n,
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

  it('test operator "min" overload (euint128, uint128) => euint128 test 3 (340282366920938463463371780590875218175, 340282366920938463463371780590875218175)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add128(340282366920938463463371780590875218175n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463371780590875218175n,
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

  it('test operator "min" overload (euint128, uint128) => euint128 test 4 (340282366920938463463371780590875218175, 340282366920938463463371780590875218171)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add128(340282366920938463463371780590875218175n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463371780590875218171n,
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
});
