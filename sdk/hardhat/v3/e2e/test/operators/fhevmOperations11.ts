import { FhevmType } from '@fhevm/hardhat-plugin-v3';
import { expect } from 'chai';
import { network } from 'hardhat';

import type {
  FHEVMTestSuite1,
  FHEVMTestSuite2,
  FHEVMTestSuite3,
  FHEVMTestSuite4,
  FHEVMTestSuite5,
  FHEVMTestSuite6,
  FHEVMTestSuite7,
} from '../../types/ethers-contracts/index.ts';
import { getSigners } from '../utils/signers.ts';

// Generated upstream (library-solidity/test/fhevmOperations), mechanically adapted: `hre.fhevm` → the
// connection's `fhevm`, handles narrowed with `at`. Suite state stays on mocha's `this`, as upstream.
const connection = await network.getOrCreate();
const { ethers, fhevm } = connection;

type Hex = `0x${string}`;

// `handles` is `Hex[]`: the i-th handle, or a loud failure.
function at(handles: readonly Hex[], i: number): Hex {
  const handle = handles[i];
  if (handle === undefined) throw new Error(`encrypt() returned no handle #${String(i)}`);
  return handle;
}

async function deployFHEVMTestFixture1(): Promise<FHEVMTestSuite1> {
  const signers = await getSigners(connection);
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite1');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployFHEVMTestFixture2(): Promise<FHEVMTestSuite2> {
  const signers = await getSigners(connection);
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite2');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployFHEVMTestFixture3(): Promise<FHEVMTestSuite3> {
  const signers = await getSigners(connection);
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite3');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployFHEVMTestFixture4(): Promise<FHEVMTestSuite4> {
  const signers = await getSigners(connection);
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite4');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployFHEVMTestFixture5(): Promise<FHEVMTestSuite5> {
  const signers = await getSigners(connection);
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite5');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployFHEVMTestFixture6(): Promise<FHEVMTestSuite6> {
  const signers = await getSigners(connection);
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite6');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployFHEVMTestFixture7(): Promise<FHEVMTestSuite7> {
  const signers = await getSigners(connection);
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite7');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

describe('FHEVM operations 11', function () {
  before(async function () {
    this.signers = await getSigners(connection);

    const contract1 = await deployFHEVMTestFixture1();
    this.contract1Address = await contract1.getAddress();
    this.contract1 = contract1;

    const contract2 = await deployFHEVMTestFixture2();
    this.contract2Address = await contract2.getAddress();
    this.contract2 = contract2;

    const contract3 = await deployFHEVMTestFixture3();
    this.contract3Address = await contract3.getAddress();
    this.contract3 = contract3;

    const contract4 = await deployFHEVMTestFixture4();
    this.contract4Address = await contract4.getAddress();
    this.contract4 = contract4;

    const contract5 = await deployFHEVMTestFixture5();
    this.contract5Address = await contract5.getAddress();
    this.contract5 = contract5;

    const contract6 = await deployFHEVMTestFixture6();
    this.contract6Address = await contract6.getAddress();
    this.contract6 = contract6;

    const contract7 = await deployFHEVMTestFixture7();
    this.contract7Address = await contract7.getAddress();
    this.contract7 = contract7;
  });

  it('test operator "lt" overload (uint64, euint64) => ebool test 1 (18445253595426581561, 18446191914714610405)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18446191914714610405n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint64_euint64(
      18445253595426581561n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint64, euint64) => ebool test 2 (18444790581199458975, 18444790581199458979)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18444790581199458979n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint64_euint64(
      18444790581199458975n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint64, euint64) => ebool test 3 (18444790581199458979, 18444790581199458979)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18444790581199458979n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint64_euint64(
      18444790581199458979n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint64, euint64) => ebool test 4 (18444790581199458979, 18444790581199458975)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18444790581199458975n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint64_euint64(
      18444790581199458979n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, uint64) => euint64 test 1 (18437934124931735627, 18440024887203299775)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18437934124931735627n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint64_uint64(
      at(encryptedAmount.handles, 0),
      18440024887203299775n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract6.resEuint64());
    expect(res).to.equal(18437934124931735627n);
  });

  it('test operator "min" overload (euint64, uint64) => euint64 test 2 (18437934124931735623, 18437934124931735627)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18437934124931735623n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint64_uint64(
      at(encryptedAmount.handles, 0),
      18437934124931735627n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract6.resEuint64());
    expect(res).to.equal(18437934124931735623n);
  });

  it('test operator "min" overload (euint64, uint64) => euint64 test 3 (18437934124931735627, 18437934124931735627)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18437934124931735627n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint64_uint64(
      at(encryptedAmount.handles, 0),
      18437934124931735627n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract6.resEuint64());
    expect(res).to.equal(18437934124931735627n);
  });

  it('test operator "min" overload (euint64, uint64) => euint64 test 4 (18437934124931735627, 18437934124931735623)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18437934124931735627n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint64_uint64(
      at(encryptedAmount.handles, 0),
      18437934124931735623n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract6.resEuint64());
    expect(res).to.equal(18437934124931735623n);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 1 (18438711784729090867, 18440024887203299775)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18440024887203299775n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint64_euint64(
      18438711784729090867n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract6.resEuint64());
    expect(res).to.equal(18438711784729090867n);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 2 (18437934124931735623, 18437934124931735627)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18437934124931735627n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint64_euint64(
      18437934124931735623n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract6.resEuint64());
    expect(res).to.equal(18437934124931735623n);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 3 (18437934124931735627, 18437934124931735627)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18437934124931735627n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint64_euint64(
      18437934124931735627n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract6.resEuint64());
    expect(res).to.equal(18437934124931735627n);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 4 (18437934124931735627, 18437934124931735623)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18437934124931735623n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint64_euint64(
      18437934124931735627n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract6.resEuint64());
    expect(res).to.equal(18437934124931735623n);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 1 (18441187267678247311, 18442651532572608437)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18441187267678247311n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint64_uint64(
      at(encryptedAmount.handles, 0),
      18442651532572608437n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract6.resEuint64());
    expect(res).to.equal(18442651532572608437n);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 2 (18441187267678247307, 18441187267678247311)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18441187267678247307n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint64_uint64(
      at(encryptedAmount.handles, 0),
      18441187267678247311n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract6.resEuint64());
    expect(res).to.equal(18441187267678247311n);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 3 (18441187267678247311, 18441187267678247311)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18441187267678247311n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint64_uint64(
      at(encryptedAmount.handles, 0),
      18441187267678247311n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract6.resEuint64());
    expect(res).to.equal(18441187267678247311n);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 4 (18441187267678247311, 18441187267678247307)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18441187267678247311n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint64_uint64(
      at(encryptedAmount.handles, 0),
      18441187267678247307n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract6.resEuint64());
    expect(res).to.equal(18441187267678247311n);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 1 (18443767409867266799, 18442651532572608437)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18442651532572608437n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint64_euint64(
      18443767409867266799n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract6.resEuint64());
    expect(res).to.equal(18443767409867266799n);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 2 (18441187267678247307, 18441187267678247311)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18441187267678247311n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint64_euint64(
      18441187267678247307n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract6.resEuint64());
    expect(res).to.equal(18441187267678247311n);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 3 (18441187267678247311, 18441187267678247311)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18441187267678247311n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint64_euint64(
      18441187267678247311n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract6.resEuint64());
    expect(res).to.equal(18441187267678247311n);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 4 (18441187267678247311, 18441187267678247307)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18441187267678247307n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint64_euint64(
      18441187267678247311n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract6.resEuint64());
    expect(res).to.equal(18441187267678247311n);
  });

  it('test operator "add" overload (euint128, uint128) => euint128 test 1 (170141183460469231731686721535437342948, 170141183460469231731683519697464432965)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(170141183460469231731686721535437342948n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint128_uint128(
      at(encryptedAmount.handles, 0),
      170141183460469231731683519697464432965n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463370241232901775913n);
  });

  it('test operator "add" overload (euint128, uint128) => euint128 test 2 (170141183460469231731685124726320525389, 170141183460469231731685124726320525391)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(170141183460469231731685124726320525389n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint128_uint128(
      at(encryptedAmount.handles, 0),
      170141183460469231731685124726320525391n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463370249452641050780n);
  });

  it('test operator "add" overload (euint128, uint128) => euint128 test 3 (170141183460469231731685124726320525391, 170141183460469231731685124726320525391)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(170141183460469231731685124726320525391n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint128_uint128(
      at(encryptedAmount.handles, 0),
      170141183460469231731685124726320525391n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463370249452641050782n);
  });

  it('test operator "add" overload (euint128, uint128) => euint128 test 4 (170141183460469231731685124726320525391, 170141183460469231731685124726320525389)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(170141183460469231731685124726320525391n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint128_uint128(
      at(encryptedAmount.handles, 0),
      170141183460469231731685124726320525389n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463370249452641050780n);
  });

  it('test operator "add" overload (uint128, euint128) => euint128 test 1 (170141183460469231731686175342116148026, 170141183460469231731683519697464432965)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(170141183460469231731683519697464432965n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_uint128_euint128(
      170141183460469231731686175342116148026n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463369695039580580991n);
  });

  it('test operator "add" overload (uint128, euint128) => euint128 test 2 (170141183460469231731685124726320525389, 170141183460469231731685124726320525391)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(170141183460469231731685124726320525391n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_uint128_euint128(
      170141183460469231731685124726320525389n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463370249452641050780n);
  });

  it('test operator "add" overload (uint128, euint128) => euint128 test 3 (170141183460469231731685124726320525391, 170141183460469231731685124726320525391)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(170141183460469231731685124726320525391n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_uint128_euint128(
      170141183460469231731685124726320525391n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463370249452641050782n);
  });

  it('test operator "add" overload (uint128, euint128) => euint128 test 4 (170141183460469231731685124726320525391, 170141183460469231731685124726320525389)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(170141183460469231731685124726320525389n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_uint128_euint128(
      170141183460469231731685124726320525391n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463370249452641050780n);
  });

  it('test operator "sub" overload (euint128, uint128) => euint128 test 1 (340282366920938463463370119270961786529, 340282366920938463463370119270961786529)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463370119270961786529n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_euint128_uint128(
      at(encryptedAmount.handles, 0),
      340282366920938463463370119270961786529n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract6.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint128, uint128) => euint128 test 2 (340282366920938463463370119270961786529, 340282366920938463463370119270961786525)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463370119270961786529n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_euint128_uint128(
      at(encryptedAmount.handles, 0),
      340282366920938463463370119270961786525n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract6.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "sub" overload (uint128, euint128) => euint128 test 1 (340282366920938463463370119270961786529, 340282366920938463463370119270961786529)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(340282366920938463463370119270961786529n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_uint128_euint128(
      340282366920938463463370119270961786529n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract6.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (uint128, euint128) => euint128 test 2 (340282366920938463463370119270961786529, 340282366920938463463370119270961786525)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(340282366920938463463370119270961786525n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_uint128_euint128(
      340282366920938463463370119270961786529n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract6.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint128, uint128) => euint128 test 1 (9223372036854775809, 9223372036854775809)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(9223372036854775809n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint128_uint128(
      at(encryptedAmount.handles, 0),
      9223372036854775809n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract6.resEuint128());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "mul" overload (euint128, uint128) => euint128 test 2 (9223372036854775809, 9223372036854775809)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(9223372036854775809n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint128_uint128(
      at(encryptedAmount.handles, 0),
      9223372036854775809n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract6.resEuint128());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "mul" overload (euint128, uint128) => euint128 test 3 (9223372036854775809, 9223372036854775809)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(9223372036854775809n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint128_uint128(
      at(encryptedAmount.handles, 0),
      9223372036854775809n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract6.resEuint128());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "mul" overload (euint128, uint128) => euint128 test 4 (9223372036854775809, 9223372036854775809)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(9223372036854775809n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint128_uint128(
      at(encryptedAmount.handles, 0),
      9223372036854775809n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract6.resEuint128());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "mul" overload (uint128, euint128) => euint128 test 1 (9223372036854775809, 9223372036854775809)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_uint128_euint128(
      9223372036854775809n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract6.resEuint128());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "mul" overload (uint128, euint128) => euint128 test 2 (9223372036854775809, 9223372036854775809)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_uint128_euint128(
      9223372036854775809n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract6.resEuint128());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "mul" overload (uint128, euint128) => euint128 test 3 (9223372036854775809, 9223372036854775809)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_uint128_euint128(
      9223372036854775809n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract6.resEuint128());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "mul" overload (uint128, euint128) => euint128 test 4 (9223372036854775809, 9223372036854775809)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_uint128_euint128(
      9223372036854775809n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract6.resEuint128());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "div" overload (euint128, uint128) => euint128 test 1 (340282366920938463463373628093423161489, 340282366920938463463373913163013931695)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463373628093423161489n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.div_euint128_uint128(
      at(encryptedAmount.handles, 0),
      340282366920938463463373913163013931695n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract6.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint128, uint128) => euint128 test 2 (340282366920938463463373427922114772061, 340282366920938463463373427922114772065)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463373427922114772061n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.div_euint128_uint128(
      at(encryptedAmount.handles, 0),
      340282366920938463463373427922114772065n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract6.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint128, uint128) => euint128 test 3 (340282366920938463463373427922114772065, 340282366920938463463373427922114772065)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463373427922114772065n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.div_euint128_uint128(
      at(encryptedAmount.handles, 0),
      340282366920938463463373427922114772065n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract6.resEuint128());
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint128, uint128) => euint128 test 4 (340282366920938463463373427922114772065, 340282366920938463463373427922114772061)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463373427922114772065n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.div_euint128_uint128(
      at(encryptedAmount.handles, 0),
      340282366920938463463373427922114772061n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract6.resEuint128());
    expect(res).to.equal(1n);
  });

  it('test operator "rem" overload (euint128, uint128) => euint128 test 1 (340282366920938463463369854817654524237, 340282366920938463463366101444463864671)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463369854817654524237n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rem_euint128_uint128(
      at(encryptedAmount.handles, 0),
      340282366920938463463366101444463864671n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract6.resEuint128());
    expect(res).to.equal(3753373190659566n);
  });

  it('test operator "rem" overload (euint128, uint128) => euint128 test 2 (340282366920938463463369816898575570763, 340282366920938463463369816898575570767)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463369816898575570763n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rem_euint128_uint128(
      at(encryptedAmount.handles, 0),
      340282366920938463463369816898575570767n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463369816898575570763n);
  });

  it('test operator "rem" overload (euint128, uint128) => euint128 test 3 (340282366920938463463369816898575570767, 340282366920938463463369816898575570767)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463369816898575570767n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rem_euint128_uint128(
      at(encryptedAmount.handles, 0),
      340282366920938463463369816898575570767n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract6.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "rem" overload (euint128, uint128) => euint128 test 4 (340282366920938463463369816898575570767, 340282366920938463463369816898575570763)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463369816898575570767n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rem_euint128_uint128(
      at(encryptedAmount.handles, 0),
      340282366920938463463369816898575570763n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract6.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "and" overload (euint128, uint128) => euint128 test 1 (340282366920938463463367044172939119359, 340282366920938463463366379225427479931)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463367044172939119359n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint128_uint128(
      at(encryptedAmount.handles, 0),
      340282366920938463463366379225427479931n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463365600511688706171n);
  });

  it('test operator "and" overload (euint128, uint128) => euint128 test 2 (340282366920938463463367044172939119355, 340282366920938463463367044172939119359)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463367044172939119355n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint128_uint128(
      at(encryptedAmount.handles, 0),
      340282366920938463463367044172939119359n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463367044172939119355n);
  });

  it('test operator "and" overload (euint128, uint128) => euint128 test 3 (340282366920938463463367044172939119359, 340282366920938463463367044172939119359)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463367044172939119359n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint128_uint128(
      at(encryptedAmount.handles, 0),
      340282366920938463463367044172939119359n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463367044172939119359n);
  });

  it('test operator "and" overload (euint128, uint128) => euint128 test 4 (340282366920938463463367044172939119359, 340282366920938463463367044172939119355)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463367044172939119359n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint128_uint128(
      at(encryptedAmount.handles, 0),
      340282366920938463463367044172939119355n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463367044172939119355n);
  });

  it('test operator "and" overload (uint128, euint128) => euint128 test 1 (340282366920938463463367713641747631459, 340282366920938463463366379225427479931)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(340282366920938463463366379225427479931n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_uint128_euint128(
      340282366920938463463367713641747631459n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463366304059183598947n);
  });

  it('test operator "and" overload (uint128, euint128) => euint128 test 2 (340282366920938463463367044172939119355, 340282366920938463463367044172939119359)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(340282366920938463463367044172939119359n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_uint128_euint128(
      340282366920938463463367044172939119355n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463367044172939119355n);
  });

  it('test operator "and" overload (uint128, euint128) => euint128 test 3 (340282366920938463463367044172939119359, 340282366920938463463367044172939119359)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(340282366920938463463367044172939119359n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_uint128_euint128(
      340282366920938463463367044172939119359n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463367044172939119359n);
  });

  it('test operator "and" overload (uint128, euint128) => euint128 test 4 (340282366920938463463367044172939119359, 340282366920938463463367044172939119355)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(340282366920938463463367044172939119355n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_uint128_euint128(
      340282366920938463463367044172939119359n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463367044172939119355n);
  });

  it('test operator "or" overload (euint128, uint128) => euint128 test 1 (340282366920938463463373650833185201525, 340282366920938463463369483560148003905)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463373650833185201525n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint128_uint128(
      at(encryptedAmount.handles, 0),
      340282366920938463463369483560148003905n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463374004898011544949n);
  });

  it('test operator "or" overload (euint128, uint128) => euint128 test 2 (340282366920938463463367653969196802041, 340282366920938463463367653969196802045)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463367653969196802041n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint128_uint128(
      at(encryptedAmount.handles, 0),
      340282366920938463463367653969196802045n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463367653969196802045n);
  });

  it('test operator "or" overload (euint128, uint128) => euint128 test 3 (340282366920938463463367653969196802045, 340282366920938463463367653969196802045)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463367653969196802045n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint128_uint128(
      at(encryptedAmount.handles, 0),
      340282366920938463463367653969196802045n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463367653969196802045n);
  });

  it('test operator "or" overload (euint128, uint128) => euint128 test 4 (340282366920938463463367653969196802045, 340282366920938463463367653969196802041)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463367653969196802045n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint128_uint128(
      at(encryptedAmount.handles, 0),
      340282366920938463463367653969196802041n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463367653969196802045n);
  });

  it('test operator "or" overload (uint128, euint128) => euint128 test 1 (340282366920938463463367629489056725583, 340282366920938463463369483560148003905)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(340282366920938463463369483560148003905n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_uint128_euint128(
      340282366920938463463367629489056725583n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463370103685029559887n);
  });

  it('test operator "or" overload (uint128, euint128) => euint128 test 2 (340282366920938463463367653969196802041, 340282366920938463463367653969196802045)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(340282366920938463463367653969196802045n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_uint128_euint128(
      340282366920938463463367653969196802041n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463367653969196802045n);
  });

  it('test operator "or" overload (uint128, euint128) => euint128 test 3 (340282366920938463463367653969196802045, 340282366920938463463367653969196802045)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(340282366920938463463367653969196802045n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_uint128_euint128(
      340282366920938463463367653969196802045n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463367653969196802045n);
  });

  it('test operator "or" overload (uint128, euint128) => euint128 test 4 (340282366920938463463367653969196802045, 340282366920938463463367653969196802041)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(340282366920938463463367653969196802041n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_uint128_euint128(
      340282366920938463463367653969196802045n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract6.resEuint128());
    expect(res).to.equal(340282366920938463463367653969196802045n);
  });

  it('test operator "xor" overload (euint128, uint128) => euint128 test 1 (340282366920938463463372811472428286857, 340282366920938463463370392206377184739)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463372811472428286857n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint128_uint128(
      at(encryptedAmount.handles, 0),
      340282366920938463463370392206377184739n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract6.resEuint128());
    expect(res).to.equal(2419836020338282n);
  });

  it('test operator "xor" overload (euint128, uint128) => euint128 test 2 (340282366920938463463367127915705168167, 340282366920938463463367127915705168171)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463367127915705168167n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint128_uint128(
      at(encryptedAmount.handles, 0),
      340282366920938463463367127915705168171n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract6.resEuint128());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint128, uint128) => euint128 test 3 (340282366920938463463367127915705168171, 340282366920938463463367127915705168171)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463367127915705168171n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint128_uint128(
      at(encryptedAmount.handles, 0),
      340282366920938463463367127915705168171n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract6.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint128, uint128) => euint128 test 4 (340282366920938463463367127915705168171, 340282366920938463463367127915705168167)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463367127915705168171n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint128_uint128(
      at(encryptedAmount.handles, 0),
      340282366920938463463367127915705168167n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract6.resEuint128());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (uint128, euint128) => euint128 test 1 (340282366920938463463367920888046121283, 340282366920938463463370392206377184739)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(340282366920938463463370392206377184739n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_uint128_euint128(
      340282366920938463463367920888046121283n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract6.resEuint128());
    expect(res).to.equal(7099382889454752n);
  });

  it('test operator "xor" overload (uint128, euint128) => euint128 test 2 (340282366920938463463367127915705168167, 340282366920938463463367127915705168171)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(340282366920938463463367127915705168171n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_uint128_euint128(
      340282366920938463463367127915705168167n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract6.resEuint128());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (uint128, euint128) => euint128 test 3 (340282366920938463463367127915705168171, 340282366920938463463367127915705168171)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(340282366920938463463367127915705168171n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_uint128_euint128(
      340282366920938463463367127915705168171n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract6.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (uint128, euint128) => euint128 test 4 (340282366920938463463367127915705168171, 340282366920938463463367127915705168167)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(340282366920938463463367127915705168167n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_uint128_euint128(
      340282366920938463463367127915705168171n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract6.resEuint128());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint128, uint128) => ebool test 1 (340282366920938463463372481304885606321, 340282366920938463463373934701871037993)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463372481304885606321n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint128_uint128(
      at(encryptedAmount.handles, 0),
      340282366920938463463373934701871037993n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, uint128) => ebool test 2 (340282366920938463463368792243012180473, 340282366920938463463368792243012180477)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463368792243012180473n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint128_uint128(
      at(encryptedAmount.handles, 0),
      340282366920938463463368792243012180477n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, uint128) => ebool test 3 (340282366920938463463368792243012180477, 340282366920938463463368792243012180477)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463368792243012180477n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint128_uint128(
      at(encryptedAmount.handles, 0),
      340282366920938463463368792243012180477n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint128, uint128) => ebool test 4 (340282366920938463463368792243012180477, 340282366920938463463368792243012180473)', async function () {
    const input = fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463368792243012180477n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint128_uint128(
      at(encryptedAmount.handles, 0),
      340282366920938463463368792243012180473n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint128, euint128) => ebool test 1 (340282366920938463463374409994588831715, 340282366920938463463373934701871037993)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463373934701871037993n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_uint128_euint128(
      340282366920938463463374409994588831715n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint128, euint128) => ebool test 2 (340282366920938463463368792243012180473, 340282366920938463463368792243012180477)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463368792243012180477n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_uint128_euint128(
      340282366920938463463368792243012180473n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint128, euint128) => ebool test 3 (340282366920938463463368792243012180477, 340282366920938463463368792243012180477)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463368792243012180477n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_uint128_euint128(
      340282366920938463463368792243012180477n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint128, euint128) => ebool test 4 (340282366920938463463368792243012180477, 340282366920938463463368792243012180473)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463368792243012180473n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_uint128_euint128(
      340282366920938463463368792243012180477n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, uint128) => ebool test 1 (340282366920938463463369221426186835803, 340282366920938463463369803455389879789)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463369221426186835803n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint128_uint128(
      at(encryptedAmount.handles, 0),
      340282366920938463463369803455389879789n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, uint128) => ebool test 2 (340282366920938463463368143814524497093, 340282366920938463463368143814524497097)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463368143814524497093n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint128_uint128(
      at(encryptedAmount.handles, 0),
      340282366920938463463368143814524497097n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, uint128) => ebool test 3 (340282366920938463463368143814524497097, 340282366920938463463368143814524497097)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463368143814524497097n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint128_uint128(
      at(encryptedAmount.handles, 0),
      340282366920938463463368143814524497097n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, uint128) => ebool test 4 (340282366920938463463368143814524497097, 340282366920938463463368143814524497093)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463368143814524497097n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint128_uint128(
      at(encryptedAmount.handles, 0),
      340282366920938463463368143814524497093n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint128, euint128) => ebool test 1 (340282366920938463463374145223283710219, 340282366920938463463369803455389879789)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463369803455389879789n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_uint128_euint128(
      340282366920938463463374145223283710219n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint128, euint128) => ebool test 2 (340282366920938463463368143814524497093, 340282366920938463463368143814524497097)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463368143814524497097n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_uint128_euint128(
      340282366920938463463368143814524497093n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint128, euint128) => ebool test 3 (340282366920938463463368143814524497097, 340282366920938463463368143814524497097)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463368143814524497097n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_uint128_euint128(
      340282366920938463463368143814524497097n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint128, euint128) => ebool test 4 (340282366920938463463368143814524497097, 340282366920938463463368143814524497093)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463368143814524497093n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_uint128_euint128(
      340282366920938463463368143814524497097n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, uint128) => ebool test 1 (340282366920938463463368669803305650403, 340282366920938463463369479120071713949)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463368669803305650403n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint128_uint128(
      at(encryptedAmount.handles, 0),
      340282366920938463463369479120071713949n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint128, uint128) => ebool test 2 (340282366920938463463368669803305650399, 340282366920938463463368669803305650403)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463368669803305650399n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint128_uint128(
      at(encryptedAmount.handles, 0),
      340282366920938463463368669803305650403n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint128, uint128) => ebool test 3 (340282366920938463463368669803305650403, 340282366920938463463368669803305650403)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463368669803305650403n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint128_uint128(
      at(encryptedAmount.handles, 0),
      340282366920938463463368669803305650403n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, uint128) => ebool test 4 (340282366920938463463368669803305650403, 340282366920938463463368669803305650399)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463368669803305650403n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint128_uint128(
      at(encryptedAmount.handles, 0),
      340282366920938463463368669803305650399n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint128, euint128) => ebool test 1 (340282366920938463463371972091836481483, 340282366920938463463369479120071713949)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463369479120071713949n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_uint128_euint128(
      340282366920938463463371972091836481483n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint128, euint128) => ebool test 2 (340282366920938463463368669803305650399, 340282366920938463463368669803305650403)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463368669803305650403n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_uint128_euint128(
      340282366920938463463368669803305650399n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint128, euint128) => ebool test 3 (340282366920938463463368669803305650403, 340282366920938463463368669803305650403)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463368669803305650403n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_uint128_euint128(
      340282366920938463463368669803305650403n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint128, euint128) => ebool test 4 (340282366920938463463368669803305650403, 340282366920938463463368669803305650399)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463368669803305650399n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_uint128_euint128(
      340282366920938463463368669803305650403n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, uint128) => ebool test 1 (340282366920938463463365956072615144479, 340282366920938463463373896900348982549)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463365956072615144479n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint128_uint128(
      at(encryptedAmount.handles, 0),
      340282366920938463463373896900348982549n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, uint128) => ebool test 2 (340282366920938463463365956072615144475, 340282366920938463463365956072615144479)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463365956072615144475n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint128_uint128(
      at(encryptedAmount.handles, 0),
      340282366920938463463365956072615144479n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, uint128) => ebool test 3 (340282366920938463463365956072615144479, 340282366920938463463365956072615144479)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463365956072615144479n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint128_uint128(
      at(encryptedAmount.handles, 0),
      340282366920938463463365956072615144479n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, uint128) => ebool test 4 (340282366920938463463365956072615144479, 340282366920938463463365956072615144475)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463365956072615144479n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint128_uint128(
      at(encryptedAmount.handles, 0),
      340282366920938463463365956072615144475n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint128, euint128) => ebool test 1 (340282366920938463463367615976897147845, 340282366920938463463373896900348982549)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463373896900348982549n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_uint128_euint128(
      340282366920938463463367615976897147845n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint128, euint128) => ebool test 2 (340282366920938463463365956072615144475, 340282366920938463463365956072615144479)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463365956072615144479n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_uint128_euint128(
      340282366920938463463365956072615144475n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint128, euint128) => ebool test 3 (340282366920938463463365956072615144479, 340282366920938463463365956072615144479)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463365956072615144479n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_uint128_euint128(
      340282366920938463463365956072615144479n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint128, euint128) => ebool test 4 (340282366920938463463365956072615144479, 340282366920938463463365956072615144475)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463365956072615144475n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_uint128_euint128(
      340282366920938463463365956072615144479n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, uint128) => ebool test 1 (340282366920938463463366569487674808983, 340282366920938463463372339406974654287)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463366569487674808983n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint128_uint128(
      at(encryptedAmount.handles, 0),
      340282366920938463463372339406974654287n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, uint128) => ebool test 2 (340282366920938463463366569487674808979, 340282366920938463463366569487674808983)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463366569487674808979n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint128_uint128(
      at(encryptedAmount.handles, 0),
      340282366920938463463366569487674808983n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, uint128) => ebool test 3 (340282366920938463463366569487674808983, 340282366920938463463366569487674808983)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463366569487674808983n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint128_uint128(
      at(encryptedAmount.handles, 0),
      340282366920938463463366569487674808983n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, uint128) => ebool test 4 (340282366920938463463366569487674808983, 340282366920938463463366569487674808979)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463366569487674808983n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint128_uint128(
      at(encryptedAmount.handles, 0),
      340282366920938463463366569487674808979n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint128, euint128) => ebool test 1 (340282366920938463463366272696917862387, 340282366920938463463372339406974654287)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463372339406974654287n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_uint128_euint128(
      340282366920938463463366272696917862387n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint128, euint128) => ebool test 2 (340282366920938463463366569487674808979, 340282366920938463463366569487674808983)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463366569487674808983n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_uint128_euint128(
      340282366920938463463366569487674808979n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint128, euint128) => ebool test 3 (340282366920938463463366569487674808983, 340282366920938463463366569487674808983)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463366569487674808983n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_uint128_euint128(
      340282366920938463463366569487674808983n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint128, euint128) => ebool test 4 (340282366920938463463366569487674808983, 340282366920938463463366569487674808979)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463366569487674808979n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_uint128_euint128(
      340282366920938463463366569487674808983n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, uint128) => ebool test 1 (340282366920938463463373727768301400539, 340282366920938463463366495793253069513)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463373727768301400539n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint128_uint128(
      at(encryptedAmount.handles, 0),
      340282366920938463463366495793253069513n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, uint128) => ebool test 2 (340282366920938463463366181206754403845, 340282366920938463463366181206754403849)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463366181206754403845n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint128_uint128(
      at(encryptedAmount.handles, 0),
      340282366920938463463366181206754403849n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint128, uint128) => ebool test 3 (340282366920938463463366181206754403849, 340282366920938463463366181206754403849)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463366181206754403849n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint128_uint128(
      at(encryptedAmount.handles, 0),
      340282366920938463463366181206754403849n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, uint128) => ebool test 4 (340282366920938463463366181206754403849, 340282366920938463463366181206754403845)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463366181206754403849n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint128_uint128(
      at(encryptedAmount.handles, 0),
      340282366920938463463366181206754403845n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint128, euint128) => ebool test 1 (340282366920938463463368549026440812767, 340282366920938463463366495793253069513)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463366495793253069513n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_uint128_euint128(
      340282366920938463463368549026440812767n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint128, euint128) => ebool test 2 (340282366920938463463366181206754403845, 340282366920938463463366181206754403849)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463366181206754403849n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_uint128_euint128(
      340282366920938463463366181206754403845n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint128, euint128) => ebool test 3 (340282366920938463463366181206754403849, 340282366920938463463366181206754403849)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463366181206754403849n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_uint128_euint128(
      340282366920938463463366181206754403849n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint128, euint128) => ebool test 4 (340282366920938463463366181206754403849, 340282366920938463463366181206754403845)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463366181206754403845n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_uint128_euint128(
      340282366920938463463366181206754403849n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint128, uint128) => euint128 test 1 (340282366920938463463372005690002957133, 340282366920938463463369584146321077511)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463372005690002957133n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint128_uint128(
      at(encryptedAmount.handles, 0),
      340282366920938463463369584146321077511n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463463369584146321077511n);
  });

  it('test operator "min" overload (euint128, uint128) => euint128 test 2 (340282366920938463463366530779050469389, 340282366920938463463366530779050469393)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463366530779050469389n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint128_uint128(
      at(encryptedAmount.handles, 0),
      340282366920938463463366530779050469393n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463463366530779050469389n);
  });

  it('test operator "min" overload (euint128, uint128) => euint128 test 3 (340282366920938463463366530779050469393, 340282366920938463463366530779050469393)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463366530779050469393n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint128_uint128(
      at(encryptedAmount.handles, 0),
      340282366920938463463366530779050469393n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463463366530779050469393n);
  });

  it('test operator "min" overload (euint128, uint128) => euint128 test 4 (340282366920938463463366530779050469393, 340282366920938463463366530779050469389)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463366530779050469393n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint128_uint128(
      at(encryptedAmount.handles, 0),
      340282366920938463463366530779050469389n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463463366530779050469389n);
  });

  it('test operator "min" overload (uint128, euint128) => euint128 test 1 (340282366920938463463369440790149068661, 340282366920938463463369584146321077511)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463369584146321077511n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_uint128_euint128(
      340282366920938463463369440790149068661n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463463369440790149068661n);
  });

  it('test operator "min" overload (uint128, euint128) => euint128 test 2 (340282366920938463463366530779050469389, 340282366920938463463366530779050469393)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463366530779050469393n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_uint128_euint128(
      340282366920938463463366530779050469389n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463463366530779050469389n);
  });

  it('test operator "min" overload (uint128, euint128) => euint128 test 3 (340282366920938463463366530779050469393, 340282366920938463463366530779050469393)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463366530779050469393n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_uint128_euint128(
      340282366920938463463366530779050469393n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463463366530779050469393n);
  });

  it('test operator "min" overload (uint128, euint128) => euint128 test 4 (340282366920938463463366530779050469393, 340282366920938463463366530779050469389)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463366530779050469389n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_uint128_euint128(
      340282366920938463463366530779050469393n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463463366530779050469389n);
  });

  it('test operator "max" overload (euint128, uint128) => euint128 test 1 (340282366920938463463369865492848935785, 340282366920938463463367826702852300509)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463369865492848935785n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint128_uint128(
      at(encryptedAmount.handles, 0),
      340282366920938463463367826702852300509n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463463369865492848935785n);
  });

  it('test operator "max" overload (euint128, uint128) => euint128 test 2 (340282366920938463463368446594301323717, 340282366920938463463368446594301323721)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463368446594301323717n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint128_uint128(
      at(encryptedAmount.handles, 0),
      340282366920938463463368446594301323721n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463463368446594301323721n);
  });

  it('test operator "max" overload (euint128, uint128) => euint128 test 3 (340282366920938463463368446594301323721, 340282366920938463463368446594301323721)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463368446594301323721n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint128_uint128(
      at(encryptedAmount.handles, 0),
      340282366920938463463368446594301323721n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463463368446594301323721n);
  });

  it('test operator "max" overload (euint128, uint128) => euint128 test 4 (340282366920938463463368446594301323721, 340282366920938463463368446594301323717)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463368446594301323721n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint128_uint128(
      at(encryptedAmount.handles, 0),
      340282366920938463463368446594301323717n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463463368446594301323721n);
  });

  it('test operator "max" overload (uint128, euint128) => euint128 test 1 (340282366920938463463374110987899673029, 340282366920938463463367826702852300509)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463367826702852300509n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_uint128_euint128(
      340282366920938463463374110987899673029n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463463374110987899673029n);
  });

  it('test operator "max" overload (uint128, euint128) => euint128 test 2 (340282366920938463463368446594301323717, 340282366920938463463368446594301323721)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463368446594301323721n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_uint128_euint128(
      340282366920938463463368446594301323717n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463463368446594301323721n);
  });

  it('test operator "max" overload (uint128, euint128) => euint128 test 3 (340282366920938463463368446594301323721, 340282366920938463463368446594301323721)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463368446594301323721n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_uint128_euint128(
      340282366920938463463368446594301323721n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463463368446594301323721n);
  });

  it('test operator "max" overload (uint128, euint128) => euint128 test 4 (340282366920938463463368446594301323721, 340282366920938463463368446594301323717)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463368446594301323717n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_uint128_euint128(
      340282366920938463463368446594301323721n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463463368446594301323721n);
  });

  it('test operator "and" overload (euint256, uint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457575356816400318221, 115792089237316195423570985008687907853269984665640564039457578483462122713915)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575356816400318221n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint256_uint256(
      at(encryptedAmount.handles, 0),
      115792089237316195423570985008687907853269984665640564039457578483462122713915n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457575074976074511113n);
  });

  it('test operator "and" overload (euint256, uint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457575356816400318217, 115792089237316195423570985008687907853269984665640564039457575356816400318221)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575356816400318217n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint256_uint256(
      at(encryptedAmount.handles, 0),
      115792089237316195423570985008687907853269984665640564039457575356816400318221n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457575356816400318217n);
  });

  it('test operator "and" overload (euint256, uint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457575356816400318221, 115792089237316195423570985008687907853269984665640564039457575356816400318221)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575356816400318221n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint256_uint256(
      at(encryptedAmount.handles, 0),
      115792089237316195423570985008687907853269984665640564039457575356816400318221n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457575356816400318221n);
  });

  it('test operator "and" overload (euint256, uint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457575356816400318221, 115792089237316195423570985008687907853269984665640564039457575356816400318217)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575356816400318221n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint256_uint256(
      at(encryptedAmount.handles, 0),
      115792089237316195423570985008687907853269984665640564039457575356816400318217n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457575356816400318217n);
  });

  it('test operator "and" overload (uint256, euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457582788763878613105, 115792089237316195423570985008687907853269984665640564039457578483462122713915)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457578483462122713915n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457582788763878613105n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457577264610466471985n);
  });

  it('test operator "and" overload (uint256, euint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457575356816400318217, 115792089237316195423570985008687907853269984665640564039457575356816400318221)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457575356816400318221n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457575356816400318217n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457575356816400318217n);
  });

  it('test operator "and" overload (uint256, euint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457575356816400318221, 115792089237316195423570985008687907853269984665640564039457575356816400318221)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457575356816400318221n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457575356816400318221n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457575356816400318221n);
  });

  it('test operator "and" overload (uint256, euint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457575356816400318221, 115792089237316195423570985008687907853269984665640564039457575356816400318217)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457575356816400318217n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457575356816400318221n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457575356816400318217n);
  });

  it('test operator "or" overload (euint256, uint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457575852478858871057, 115792089237316195423570985008687907853269984665640564039457575899044080167609)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575852478858871057n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint256_uint256(
      at(encryptedAmount.handles, 0),
      115792089237316195423570985008687907853269984665640564039457575899044080167609n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457575906363527130041n);
  });

  it('test operator "or" overload (euint256, uint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457575852478858871053, 115792089237316195423570985008687907853269984665640564039457575852478858871057)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575852478858871053n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint256_uint256(
      at(encryptedAmount.handles, 0),
      115792089237316195423570985008687907853269984665640564039457575852478858871057n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457575852478858871069n);
  });

  it('test operator "or" overload (euint256, uint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457575852478858871057, 115792089237316195423570985008687907853269984665640564039457575852478858871057)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575852478858871057n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint256_uint256(
      at(encryptedAmount.handles, 0),
      115792089237316195423570985008687907853269984665640564039457575852478858871057n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457575852478858871057n);
  });

  it('test operator "or" overload (euint256, uint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457575852478858871057, 115792089237316195423570985008687907853269984665640564039457575852478858871053)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575852478858871057n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint256_uint256(
      at(encryptedAmount.handles, 0),
      115792089237316195423570985008687907853269984665640564039457575852478858871053n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457575852478858871069n);
  });

  it('test operator "or" overload (uint256, euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457577413783736724049, 115792089237316195423570985008687907853269984665640564039457575899044080167609)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457575899044080167609n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457577413783736724049n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457578294502281358073n);
  });

  it('test operator "or" overload (uint256, euint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457575852478858871053, 115792089237316195423570985008687907853269984665640564039457575852478858871057)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457575852478858871057n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457575852478858871053n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457575852478858871069n);
  });

  it('test operator "or" overload (uint256, euint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457575852478858871057, 115792089237316195423570985008687907853269984665640564039457575852478858871057)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457575852478858871057n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457575852478858871057n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457575852478858871057n);
  });

  it('test operator "or" overload (uint256, euint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457575852478858871057, 115792089237316195423570985008687907853269984665640564039457575852478858871053)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457575852478858871053n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457575852478858871057n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457575852478858871069n);
  });

  it('test operator "xor" overload (euint256, uint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457579203735796060739, 115792089237316195423570985008687907853269984665640564039457578345624206780907)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579203735796060739n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint256_uint256(
      at(encryptedAmount.handles, 0),
      115792089237316195423570985008687907853269984665640564039457578345624206780907n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract7.resEuint256());
    expect(res).to.equal(1421178585168808n);
  });

  it('test operator "xor" overload (euint256, uint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457579203735796060735, 115792089237316195423570985008687907853269984665640564039457579203735796060739)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579203735796060735n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint256_uint256(
      at(encryptedAmount.handles, 0),
      115792089237316195423570985008687907853269984665640564039457579203735796060739n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract7.resEuint256());
    expect(res).to.equal(124n);
  });

  it('test operator "xor" overload (euint256, uint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457579203735796060739, 115792089237316195423570985008687907853269984665640564039457579203735796060739)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579203735796060739n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint256_uint256(
      at(encryptedAmount.handles, 0),
      115792089237316195423570985008687907853269984665640564039457579203735796060739n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract7.resEuint256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint256, uint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457579203735796060739, 115792089237316195423570985008687907853269984665640564039457579203735796060735)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579203735796060739n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint256_uint256(
      at(encryptedAmount.handles, 0),
      115792089237316195423570985008687907853269984665640564039457579203735796060735n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract7.resEuint256());
    expect(res).to.equal(124n);
  });

  it('test operator "xor" overload (uint256, euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457579049851759049395, 115792089237316195423570985008687907853269984665640564039457578345624206780907)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457578345624206780907n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457579049851759049395n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract7.resEuint256());
    expect(res).to.equal(1548691137106776n);
  });

  it('test operator "xor" overload (uint256, euint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457579203735796060735, 115792089237316195423570985008687907853269984665640564039457579203735796060739)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457579203735796060739n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457579203735796060735n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract7.resEuint256());
    expect(res).to.equal(124n);
  });

  it('test operator "xor" overload (uint256, euint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457579203735796060739, 115792089237316195423570985008687907853269984665640564039457579203735796060739)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457579203735796060739n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457579203735796060739n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract7.resEuint256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (uint256, euint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457579203735796060739, 115792089237316195423570985008687907853269984665640564039457579203735796060735)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457579203735796060735n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457579203735796060739n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract7.resEuint256());
    expect(res).to.equal(124n);
  });

  it('test operator "eq" overload (euint256, uint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457583724238425323955, 115792089237316195423570985008687907853269984665640564039457581448935372067265)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457583724238425323955n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint256_uint256(
      at(encryptedAmount.handles, 0),
      115792089237316195423570985008687907853269984665640564039457581448935372067265n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, uint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457579495105216444457, 115792089237316195423570985008687907853269984665640564039457579495105216444461)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579495105216444457n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint256_uint256(
      at(encryptedAmount.handles, 0),
      115792089237316195423570985008687907853269984665640564039457579495105216444461n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, uint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457579495105216444461, 115792089237316195423570985008687907853269984665640564039457579495105216444461)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579495105216444461n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint256_uint256(
      at(encryptedAmount.handles, 0),
      115792089237316195423570985008687907853269984665640564039457579495105216444461n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint256, uint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457579495105216444461, 115792089237316195423570985008687907853269984665640564039457579495105216444457)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579495105216444461n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint256_uint256(
      at(encryptedAmount.handles, 0),
      115792089237316195423570985008687907853269984665640564039457579495105216444457n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint256, euint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457577715225035525323, 115792089237316195423570985008687907853269984665640564039457581448935372067265)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457581448935372067265n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457577715225035525323n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint256, euint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457579495105216444457, 115792089237316195423570985008687907853269984665640564039457579495105216444461)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457579495105216444461n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457579495105216444457n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint256, euint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457579495105216444461, 115792089237316195423570985008687907853269984665640564039457579495105216444461)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457579495105216444461n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457579495105216444461n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint256, euint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457579495105216444461, 115792089237316195423570985008687907853269984665640564039457579495105216444457)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457579495105216444457n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457579495105216444461n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, uint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457583555262774063695, 115792089237316195423570985008687907853269984665640564039457580923347220675367)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457583555262774063695n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint256_uint256(
      at(encryptedAmount.handles, 0),
      115792089237316195423570985008687907853269984665640564039457580923347220675367n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, uint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457579280144501982149, 115792089237316195423570985008687907853269984665640564039457579280144501982153)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579280144501982149n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint256_uint256(
      at(encryptedAmount.handles, 0),
      115792089237316195423570985008687907853269984665640564039457579280144501982153n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, uint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457579280144501982153, 115792089237316195423570985008687907853269984665640564039457579280144501982153)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579280144501982153n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint256_uint256(
      at(encryptedAmount.handles, 0),
      115792089237316195423570985008687907853269984665640564039457579280144501982153n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, uint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457579280144501982153, 115792089237316195423570985008687907853269984665640564039457579280144501982149)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579280144501982153n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint256_uint256(
      at(encryptedAmount.handles, 0),
      115792089237316195423570985008687907853269984665640564039457579280144501982149n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint256, euint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457577704923654258635, 115792089237316195423570985008687907853269984665640564039457580923347220675367)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457580923347220675367n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457577704923654258635n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint256, euint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457579280144501982149, 115792089237316195423570985008687907853269984665640564039457579280144501982153)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457579280144501982153n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457579280144501982149n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint256, euint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457579280144501982153, 115792089237316195423570985008687907853269984665640564039457579280144501982153)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457579280144501982153n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457579280144501982153n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint256, euint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457579280144501982153, 115792089237316195423570985008687907853269984665640564039457579280144501982149)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457579280144501982149n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457579280144501982153n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEbool(await this.contract7.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "shl" overload (euint8, euint8) => euint8 test 1 (81, 9)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(81n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint8_euint8(
      at(encryptedAmount.handles, 0),
      at(encryptedAmount.handles, 1),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract7.resEuint8());
    expect(res).to.equal(162n);
  });

  it('test operator "shl" overload (euint8, euint8) => euint8 test 2 (5, 9)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(5n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint8_euint8(
      at(encryptedAmount.handles, 0),
      at(encryptedAmount.handles, 1),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract7.resEuint8());
    expect(res).to.equal(10n);
  });

  it('test operator "shl" overload (euint8, euint8) => euint8 test 3 (9, 9)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(9n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint8_euint8(
      at(encryptedAmount.handles, 0),
      at(encryptedAmount.handles, 1),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract7.resEuint8());
    expect(res).to.equal(18n);
  });

  it('test operator "shl" overload (euint8, euint8) => euint8 test 4 (9, 5)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(9n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint8_euint8(
      at(encryptedAmount.handles, 0),
      at(encryptedAmount.handles, 1),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract7.resEuint8());
    expect(res).to.equal(32n);
  });

  it('test operator "shl" overload (euint8, uint8) => euint8 test 1 (81, 9)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(81n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint8_uint8(at(encryptedAmount.handles, 0), 9n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract7.resEuint8());
    expect(res).to.equal(162n);
  });

  it('test operator "shl" overload (euint8, uint8) => euint8 test 2 (5, 9)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint8_uint8(at(encryptedAmount.handles, 0), 9n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract7.resEuint8());
    expect(res).to.equal(10n);
  });

  it('test operator "shl" overload (euint8, uint8) => euint8 test 3 (9, 9)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(9n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint8_uint8(at(encryptedAmount.handles, 0), 9n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract7.resEuint8());
    expect(res).to.equal(18n);
  });

  it('test operator "shl" overload (euint8, uint8) => euint8 test 4 (9, 5)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(9n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint8_uint8(at(encryptedAmount.handles, 0), 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract7.resEuint8());
    expect(res).to.equal(32n);
  });

  it('test operator "shr" overload (euint8, euint8) => euint8 test 1 (69, 10)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(69n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint8_euint8(
      at(encryptedAmount.handles, 0),
      at(encryptedAmount.handles, 1),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract7.resEuint8());
    expect(res).to.equal(17n);
  });

  it('test operator "shr" overload (euint8, euint8) => euint8 test 2 (6, 10)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(6n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint8_euint8(
      at(encryptedAmount.handles, 0),
      at(encryptedAmount.handles, 1),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract7.resEuint8());
    expect(res).to.equal(1n);
  });

  it('test operator "shr" overload (euint8, euint8) => euint8 test 3 (10, 10)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(10n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint8_euint8(
      at(encryptedAmount.handles, 0),
      at(encryptedAmount.handles, 1),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract7.resEuint8());
    expect(res).to.equal(2n);
  });

  it('test operator "shr" overload (euint8, euint8) => euint8 test 4 (10, 6)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(10n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint8_euint8(
      at(encryptedAmount.handles, 0),
      at(encryptedAmount.handles, 1),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract7.resEuint8());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint8, uint8) => euint8 test 1 (69, 10)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(69n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint8_uint8(at(encryptedAmount.handles, 0), 10n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract7.resEuint8());
    expect(res).to.equal(17n);
  });

  it('test operator "shr" overload (euint8, uint8) => euint8 test 2 (6, 10)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(6n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint8_uint8(at(encryptedAmount.handles, 0), 10n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract7.resEuint8());
    expect(res).to.equal(1n);
  });

  it('test operator "shr" overload (euint8, uint8) => euint8 test 3 (10, 10)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(10n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint8_uint8(at(encryptedAmount.handles, 0), 10n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract7.resEuint8());
    expect(res).to.equal(2n);
  });

  it('test operator "shr" overload (euint8, uint8) => euint8 test 4 (10, 6)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(10n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint8_uint8(at(encryptedAmount.handles, 0), 6n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract7.resEuint8());
    expect(res).to.equal(0n);
  });

  it('test operator "rotl" overload (euint8, euint8) => euint8 test 1 (13, 6)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(13n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint8_euint8(
      at(encryptedAmount.handles, 0),
      at(encryptedAmount.handles, 1),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract7.resEuint8());
    expect(res).to.equal(67n);
  });

  it('test operator "rotl" overload (euint8, euint8) => euint8 test 2 (2, 6)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(2n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint8_euint8(
      at(encryptedAmount.handles, 0),
      at(encryptedAmount.handles, 1),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract7.resEuint8());
    expect(res).to.equal(128n);
  });

  it('test operator "rotl" overload (euint8, euint8) => euint8 test 3 (6, 6)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(6n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint8_euint8(
      at(encryptedAmount.handles, 0),
      at(encryptedAmount.handles, 1),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract7.resEuint8());
    expect(res).to.equal(129n);
  });

  it('test operator "rotl" overload (euint8, euint8) => euint8 test 4 (6, 2)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(6n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint8_euint8(
      at(encryptedAmount.handles, 0),
      at(encryptedAmount.handles, 1),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract7.resEuint8());
    expect(res).to.equal(24n);
  });

  it('test operator "rotl" overload (euint8, uint8) => euint8 test 1 (13, 6)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(13n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint8_uint8(at(encryptedAmount.handles, 0), 6n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract7.resEuint8());
    expect(res).to.equal(67n);
  });

  it('test operator "rotl" overload (euint8, uint8) => euint8 test 2 (2, 6)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(2n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint8_uint8(at(encryptedAmount.handles, 0), 6n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract7.resEuint8());
    expect(res).to.equal(128n);
  });

  it('test operator "rotl" overload (euint8, uint8) => euint8 test 3 (6, 6)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(6n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint8_uint8(at(encryptedAmount.handles, 0), 6n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract7.resEuint8());
    expect(res).to.equal(129n);
  });

  it('test operator "rotl" overload (euint8, uint8) => euint8 test 4 (6, 2)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(6n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint8_uint8(at(encryptedAmount.handles, 0), 2n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract7.resEuint8());
    expect(res).to.equal(24n);
  });

  it('test operator "rotr" overload (euint8, euint8) => euint8 test 1 (226, 10)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(226n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint8_euint8(
      at(encryptedAmount.handles, 0),
      at(encryptedAmount.handles, 1),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract7.resEuint8());
    expect(res).to.equal(184n);
  });

  it('test operator "rotr" overload (euint8, euint8) => euint8 test 2 (6, 10)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(6n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint8_euint8(
      at(encryptedAmount.handles, 0),
      at(encryptedAmount.handles, 1),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract7.resEuint8());
    expect(res).to.equal(129n);
  });

  it('test operator "rotr" overload (euint8, euint8) => euint8 test 3 (10, 10)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(10n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint8_euint8(
      at(encryptedAmount.handles, 0),
      at(encryptedAmount.handles, 1),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract7.resEuint8());
    expect(res).to.equal(130n);
  });

  it('test operator "rotr" overload (euint8, euint8) => euint8 test 4 (10, 6)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(10n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint8_euint8(
      at(encryptedAmount.handles, 0),
      at(encryptedAmount.handles, 1),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract7.resEuint8());
    expect(res).to.equal(40n);
  });
});
