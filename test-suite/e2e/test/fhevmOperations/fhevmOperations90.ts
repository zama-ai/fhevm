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

describe('FHEVM operations 90', function () {
  before(async function () {
    this.signer = await getSigner(90);

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

  it('test operator "and" overload (uint128, euint128) => euint128 test 1 (340282366920938463463372042619194911899, 340282366920938463463369517417588913885)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add128(340282366920938463463369517417588913885n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_uint128_euint128(
      340282366920938463463372042619194911899n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463366974622192903321n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (uint128, euint128) => euint128 test 2 (340282366920938463463369620863023408923, 340282366920938463463369620863023408927)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add128(340282366920938463463369620863023408927n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_uint128_euint128(
      340282366920938463463369620863023408923n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463369620863023408923n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (uint128, euint128) => euint128 test 3 (340282366920938463463369620863023408927, 340282366920938463463369620863023408927)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add128(340282366920938463463369620863023408927n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_uint128_euint128(
      340282366920938463463369620863023408927n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463369620863023408927n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (uint128, euint128) => euint128 test 4 (340282366920938463463369620863023408927, 340282366920938463463369620863023408923)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add128(340282366920938463463369620863023408923n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_uint128_euint128(
      340282366920938463463369620863023408927n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463369620863023408923n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint128, uint128) => euint128 test 1 (340282366920938463463372632433264727071, 340282366920938463463368998126415641363)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add128(340282366920938463463372632433264727071n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368998126415641363n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463373758608049569567n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint128, uint128) => euint128 test 2 (340282366920938463463372632433264727067, 340282366920938463463372632433264727071)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add128(340282366920938463463372632433264727067n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463372632433264727071n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463372632433264727071n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint128, uint128) => euint128 test 3 (340282366920938463463372632433264727071, 340282366920938463463372632433264727071)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add128(340282366920938463463372632433264727071n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463372632433264727071n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463372632433264727071n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint128, uint128) => euint128 test 4 (340282366920938463463372632433264727071, 340282366920938463463372632433264727067)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add128(340282366920938463463372632433264727071n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463372632433264727067n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463372632433264727071n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (uint128, euint128) => euint128 test 1 (340282366920938463463367781861913971727, 340282366920938463463368998126415641363)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add128(340282366920938463463368998126415641363n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_uint128_euint128(
      340282366920938463463367781861913971727n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463370053804352864031n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (uint128, euint128) => euint128 test 2 (340282366920938463463372632433264727067, 340282366920938463463372632433264727071)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add128(340282366920938463463372632433264727071n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_uint128_euint128(
      340282366920938463463372632433264727067n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463372632433264727071n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (uint128, euint128) => euint128 test 3 (340282366920938463463372632433264727071, 340282366920938463463372632433264727071)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add128(340282366920938463463372632433264727071n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_uint128_euint128(
      340282366920938463463372632433264727071n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463372632433264727071n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (uint128, euint128) => euint128 test 4 (340282366920938463463372632433264727071, 340282366920938463463372632433264727067)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add128(340282366920938463463372632433264727067n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_uint128_euint128(
      340282366920938463463372632433264727071n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463372632433264727071n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint128, uint128) => euint128 test 1 (340282366920938463463370663230474381789, 340282366920938463463368162365024047043)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add128(340282366920938463463370663230474381789n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368162365024047043n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 7009368991884830n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint128, uint128) => euint128 test 2 (340282366920938463463366711241836032013, 340282366920938463463366711241836032017)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add128(340282366920938463463366711241836032013n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463366711241836032017n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 28n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint128, uint128) => euint128 test 3 (340282366920938463463366711241836032017, 340282366920938463463366711241836032017)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add128(340282366920938463463366711241836032017n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463366711241836032017n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint128, uint128) => euint128 test 4 (340282366920938463463366711241836032017, 340282366920938463463366711241836032013)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add128(340282366920938463463366711241836032017n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463366711241836032013n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 28n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (uint128, euint128) => euint128 test 1 (340282366920938463463365988352988897707, 340282366920938463463368162365024047043)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add128(340282366920938463463368162365024047043n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_uint128_euint128(
      340282366920938463463365988352988897707n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2386767570824808n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (uint128, euint128) => euint128 test 2 (340282366920938463463366711241836032013, 340282366920938463463366711241836032017)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add128(340282366920938463463366711241836032017n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_uint128_euint128(
      340282366920938463463366711241836032013n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 28n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (uint128, euint128) => euint128 test 3 (340282366920938463463366711241836032017, 340282366920938463463366711241836032017)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add128(340282366920938463463366711241836032017n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_uint128_euint128(
      340282366920938463463366711241836032017n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (uint128, euint128) => euint128 test 4 (340282366920938463463366711241836032017, 340282366920938463463366711241836032013)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add128(340282366920938463463366711241836032013n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_uint128_euint128(
      340282366920938463463366711241836032017n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 28n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint128, uint128) => ebool test 1 (340282366920938463463369402042742166611, 340282366920938463463369900732334670961)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add128(340282366920938463463369402042742166611n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463369900732334670961n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint128, uint128) => ebool test 2 (340282366920938463463369402042742166607, 340282366920938463463369402042742166611)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add128(340282366920938463463369402042742166607n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463369402042742166611n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint128, uint128) => ebool test 3 (340282366920938463463369402042742166611, 340282366920938463463369402042742166611)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add128(340282366920938463463369402042742166611n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463369402042742166611n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint128, uint128) => ebool test 4 (340282366920938463463369402042742166611, 340282366920938463463369402042742166607)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add128(340282366920938463463369402042742166611n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463369402042742166607n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });
});
