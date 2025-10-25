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

describe('FHEVM operations 88', function () {
  before(async function () {
    this.signer = await getSigner(88);

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

  it('test operator "min" overload (uint64, euint64) => euint64 test 1 (18445419736884143619, 18445725819605010405)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add64(18445725819605010405n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint64_euint64(
      18445419736884143619n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18445419736884143619n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 2 (18438248006307692213, 18438248006307692217)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add64(18438248006307692217n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint64_euint64(
      18438248006307692213n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18438248006307692213n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 3 (18438248006307692217, 18438248006307692217)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add64(18438248006307692217n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint64_euint64(
      18438248006307692217n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18438248006307692217n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 4 (18438248006307692217, 18438248006307692213)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add64(18438248006307692213n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint64_euint64(
      18438248006307692217n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18438248006307692213n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 1 (18444237080447470479, 18443180247415512627)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add64(18444237080447470479n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint64_uint64(
      encryptedAmount.handles[0],
      18443180247415512627n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18444237080447470479n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 2 (18439875117400843375, 18439875117400843379)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add64(18439875117400843375n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint64_uint64(
      encryptedAmount.handles[0],
      18439875117400843379n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18439875117400843379n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 3 (18439875117400843379, 18439875117400843379)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add64(18439875117400843379n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint64_uint64(
      encryptedAmount.handles[0],
      18439875117400843379n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18439875117400843379n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 4 (18439875117400843379, 18439875117400843375)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add64(18439875117400843379n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint64_uint64(
      encryptedAmount.handles[0],
      18439875117400843375n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18439875117400843379n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 1 (18445863906332305427, 18443180247415512627)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add64(18443180247415512627n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint64_euint64(
      18445863906332305427n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18445863906332305427n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 2 (18439875117400843375, 18439875117400843379)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add64(18439875117400843379n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint64_euint64(
      18439875117400843375n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18439875117400843379n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 3 (18439875117400843379, 18439875117400843379)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add64(18439875117400843379n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint64_euint64(
      18439875117400843379n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18439875117400843379n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 4 (18439875117400843379, 18439875117400843375)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add64(18439875117400843375n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint64_euint64(
      18439875117400843379n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18439875117400843379n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint128, uint128) => euint128 test 1 (170141183460469231731687272015842765950, 170141183460469231731685762323310324907)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add128(170141183460469231731687272015842765950n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint128_uint128(
      encryptedAmount.handles[0],
      170141183460469231731685762323310324907n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463373034339153090857n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint128, uint128) => euint128 test 2 (170141183460469231731686444035462849343, 170141183460469231731686444035462849345)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add128(170141183460469231731686444035462849343n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint128_uint128(
      encryptedAmount.handles[0],
      170141183460469231731686444035462849345n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463372888070925698688n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint128, uint128) => euint128 test 3 (170141183460469231731686444035462849345, 170141183460469231731686444035462849345)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add128(170141183460469231731686444035462849345n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint128_uint128(
      encryptedAmount.handles[0],
      170141183460469231731686444035462849345n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463372888070925698690n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint128, uint128) => euint128 test 4 (170141183460469231731686444035462849345, 170141183460469231731686444035462849343)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add128(170141183460469231731686444035462849345n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint128_uint128(
      encryptedAmount.handles[0],
      170141183460469231731686444035462849343n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463372888070925698688n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (uint128, euint128) => euint128 test 1 (170141183460469231731686785650767026796, 170141183460469231731685762323310324907)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add128(170141183460469231731685762323310324907n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_uint128_euint128(
      170141183460469231731686785650767026796n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463372547974077351703n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (uint128, euint128) => euint128 test 2 (170141183460469231731686444035462849343, 170141183460469231731686444035462849345)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add128(170141183460469231731686444035462849345n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_uint128_euint128(
      170141183460469231731686444035462849343n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463372888070925698688n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (uint128, euint128) => euint128 test 3 (170141183460469231731686444035462849345, 170141183460469231731686444035462849345)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add128(170141183460469231731686444035462849345n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_uint128_euint128(
      170141183460469231731686444035462849345n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463372888070925698690n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (uint128, euint128) => euint128 test 4 (170141183460469231731686444035462849345, 170141183460469231731686444035462849343)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add128(170141183460469231731686444035462849343n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_uint128_euint128(
      170141183460469231731686444035462849345n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463372888070925698688n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint128, uint128) => euint128 test 1 (340282366920938463463365918437382145247, 340282366920938463463365918437382145247)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add128(340282366920938463463365918437382145247n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463365918437382145247n,
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

  it('test operator "sub" overload (euint128, uint128) => euint128 test 2 (340282366920938463463365918437382145247, 340282366920938463463365918437382145243)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add128(340282366920938463463365918437382145247n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463365918437382145243n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });
});
