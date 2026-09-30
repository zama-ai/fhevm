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

describe('FHEVM operations 13', function () {
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

  it('test operator "not" overload (euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457575743487352122021)', async function () {
    const input = fhevm.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575743487352122021n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.not_euint256(at(encryptedAmount.handles, 0), encryptedAmount.inputProof);
    await tx.wait();
    const res = await fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract7.resEuint256());
    expect(res).to.equal(8264425777517914n);
  });
});
