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

describe('FHEVM operations 62', function () {
  before(async function () {
    this.signer = await getSigner(62);

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

  it('test operator "le" overload (euint128, euint128) => ebool test 1 (340282366920938463463369431786062180615, 340282366920938463463367329768413053359)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add128(340282366920938463463369431786062180615n);
    input.add128(340282366920938463463367329768413053359n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint128, euint128) => ebool test 2 (340282366920938463463367329768413053355, 340282366920938463463367329768413053359)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add128(340282366920938463463367329768413053355n);
    input.add128(340282366920938463463367329768413053359n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint128, euint128) => ebool test 3 (340282366920938463463367329768413053359, 340282366920938463463367329768413053359)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add128(340282366920938463463367329768413053359n);
    input.add128(340282366920938463463367329768413053359n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint128, euint128) => ebool test 4 (340282366920938463463367329768413053359, 340282366920938463463367329768413053355)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add128(340282366920938463463367329768413053359n);
    input.add128(340282366920938463463367329768413053355n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint128, euint128) => ebool test 1 (340282366920938463463371647940600324849, 340282366920938463463372791110312491575)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add128(340282366920938463463371647940600324849n);
    input.add128(340282366920938463463372791110312491575n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint128, euint128) => ebool test 2 (340282366920938463463371647940600324845, 340282366920938463463371647940600324849)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add128(340282366920938463463371647940600324845n);
    input.add128(340282366920938463463371647940600324849n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint128, euint128) => ebool test 3 (340282366920938463463371647940600324849, 340282366920938463463371647940600324849)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add128(340282366920938463463371647940600324849n);
    input.add128(340282366920938463463371647940600324849n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint128, euint128) => ebool test 4 (340282366920938463463371647940600324849, 340282366920938463463371647940600324845)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add128(340282366920938463463371647940600324849n);
    input.add128(340282366920938463463371647940600324845n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint128, euint128) => euint128 test 1 (340282366920938463463374432859434778255, 340282366920938463463371780590875218175)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add128(340282366920938463463374432859434778255n);
    input.add128(340282366920938463463371780590875218175n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463371780590875218175n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint128, euint128) => euint128 test 2 (340282366920938463463371780590875218171, 340282366920938463463371780590875218175)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add128(340282366920938463463371780590875218171n);
    input.add128(340282366920938463463371780590875218175n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463371780590875218171n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint128, euint128) => euint128 test 3 (340282366920938463463371780590875218175, 340282366920938463463371780590875218175)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add128(340282366920938463463371780590875218175n);
    input.add128(340282366920938463463371780590875218175n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463371780590875218175n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint128, euint128) => euint128 test 4 (340282366920938463463371780590875218175, 340282366920938463463371780590875218171)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add128(340282366920938463463371780590875218175n);
    input.add128(340282366920938463463371780590875218171n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463371780590875218171n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint128, euint128) => euint128 test 1 (340282366920938463463367474228925970149, 340282366920938463463367382444010695721)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add128(340282366920938463463367474228925970149n);
    input.add128(340282366920938463463367382444010695721n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463367474228925970149n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint128, euint128) => euint128 test 2 (340282366920938463463367382444010695717, 340282366920938463463367382444010695721)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add128(340282366920938463463367382444010695717n);
    input.add128(340282366920938463463367382444010695721n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463367382444010695721n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint128, euint128) => euint128 test 3 (340282366920938463463367382444010695721, 340282366920938463463367382444010695721)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add128(340282366920938463463367382444010695721n);
    input.add128(340282366920938463463367382444010695721n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463367382444010695721n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint128, euint128) => euint128 test 4 (340282366920938463463367382444010695721, 340282366920938463463367382444010695717)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add128(340282366920938463463367382444010695721n);
    input.add128(340282366920938463463367382444010695717n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463367382444010695721n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint128, euint256) => euint256 test 1 (340282366920938463463373907793626314231, 115792089237316195423570985008687907853269984665640564039457578545393601235193)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add128(340282366920938463463373907793626314231n);
    input.add256(115792089237316195423570985008687907853269984665640564039457578545393601235193n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463369122578360959217n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint128, euint256) => euint256 test 2 (340282366920938463463373907793626314227, 340282366920938463463373907793626314231)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add128(340282366920938463463373907793626314227n);
    input.add256(340282366920938463463373907793626314231n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463373907793626314227n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint128, euint256) => euint256 test 3 (340282366920938463463373907793626314231, 340282366920938463463373907793626314231)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add128(340282366920938463463373907793626314231n);
    input.add256(340282366920938463463373907793626314231n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463373907793626314231n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint128, euint256) => euint256 test 4 (340282366920938463463373907793626314231, 340282366920938463463373907793626314227)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add128(340282366920938463463373907793626314231n);
    input.add256(340282366920938463463373907793626314227n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463373907793626314227n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint128, euint256) => euint256 test 1 (340282366920938463463369773824201675833, 115792089237316195423570985008687907853269984665640564039457577934122967382619)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add128(340282366920938463463369773824201675833n);
    input.add256(115792089237316195423570985008687907853269984665640564039457577934122967382619n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457579218438582402683n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint128, euint256) => euint256 test 2 (340282366920938463463369773824201675829, 340282366920938463463369773824201675833)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add128(340282366920938463463369773824201675829n);
    input.add256(340282366920938463463369773824201675833n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463369773824201675837n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint128, euint256) => euint256 test 3 (340282366920938463463369773824201675833, 340282366920938463463369773824201675833)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add128(340282366920938463463369773824201675833n);
    input.add256(340282366920938463463369773824201675833n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463369773824201675833n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint128, euint256) => euint256 test 4 (340282366920938463463369773824201675833, 340282366920938463463369773824201675829)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add128(340282366920938463463369773824201675833n);
    input.add256(340282366920938463463369773824201675829n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463369773824201675837n,
    };
    assert.deepEqual(res, expectedRes);
  });
});
