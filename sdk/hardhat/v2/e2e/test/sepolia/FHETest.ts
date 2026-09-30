import { FhevmType } from '@fhevm/hardhat-plugin';
import type { HardhatEthersSigner } from '@nomicfoundation/hardhat-ethers/signers';
import { expect } from 'chai';
import { ethers, fhevm, network } from 'hardhat';

import type { FHETest, FHETest__factory } from '../../typechain-types';
import {
  DevnetFHETestAddress,
  LocalACLAddress,
  MainnetACLAddress,
  MainnetFHETestAddress,
  TestnetACLAddress,
  TestnetFHETestAddress,
} from './addresses';

// Hardhat
// =======
// npx hardhat test --grep "FHETest:check:" --network hardhat
// npx hardhat test --grep "FHETest:encrypt:" --network hardhat
// npx hardhat test --grep "FHETest:addEuint32:" --network hardhat
// npx hardhat test --grep "FHETest:addEuint32:publicDecrypt:" --network hardhat
// npx hardhat test --grep "FHETest:getEuint32:" --network hardhat
// npx hardhat test --grep "FHETest:userDecrypt:" --network hardhat
// npx hardhat test --grep "FHETest:addEuint32:userDecrypt:" --network hardhat

// Sepolia
// =======
// npx hardhat test --grep "FHETest:check:" --network sepolia
// npx hardhat test --grep "FHETest:encrypt:" --network sepolia
// npx hardhat test --grep "FHETest:addEuint32:" --network sepolia
// npx hardhat test --grep "FHETest:addEuint32:publicDecrypt:" --network sepolia
// npx hardhat test --grep "FHETest:getEuint32:" --network sepolia
// npx hardhat test --grep "FHETest:userDecrypt:" --network sepolia
// npx hardhat test --grep "FHETest:addEuint32:userDecrypt:" --network sepolia

// Devnet
// ======
// npx hardhat test --grep "FHETest:check:" --network devnet
// npx hardhat test --grep "FHETest:encrypt:" --network devnet
// npx hardhat test --grep "FHETest:addEuint32:userDecrypt:" --network devnet

// Mainnet
// ======
// npx hardhat test --grep "FHETest:check:" --network mainnet
// npx hardhat test --grep "FHETest:encrypt:" --network mainnet
// npx hardhat test --grep "FHETest:addEuint32:" --network mainnet
// npx hardhat test --grep "FHETest:getEuint32:" --network mainnet
// npx hardhat test --grep "FHETest:userDecrypt:" --network mainnet
// npx hardhat test --grep "FHETest:addEuint32:publicDecrypt:" --network mainnet
// npx hardhat test --grep "FHETest:addEuint32:userDecrypt:" --network mainnet

type Signers = {
  alice: HardhatEthersSigner;
};

async function deployFixtureMock(): Promise<{
  readonly fheTestContract: FHETest;
  readonly fheTestContractAddress: string;
}> {
  const factory: FHETest__factory = await ethers.getContractFactory('FHETest');
  const fheTestContract = (await factory.deploy()) as FHETest;
  const fheTestContractAddress = await fheTestContract.getAddress();

  return { fheTestContract, fheTestContractAddress };
}

async function deployFixture(): Promise<{
  readonly fheTestContract: FHETest;
  readonly fheTestContractAddress: string;
}> {
  let fheTestContractAddress;
  if (network.name === 'devnet') {
    fheTestContractAddress = DevnetFHETestAddress;
  } else if (network.name === 'sepolia') {
    fheTestContractAddress = TestnetFHETestAddress;
  } else if (network.name === 'mainnet') {
    fheTestContractAddress = MainnetFHETestAddress;
  } else {
    throw new Error(`Unsupported network=${network.name}`);
  }

  console.log(`Getting FHETest contract at ${fheTestContractAddress}, network:${network.name}`);
  const fheTestContract = await ethers.getContractAt('FHETest', fheTestContractAddress);

  const coprocessorConfig = await fhevm.getCoprocessorConfig(fheTestContractAddress);
  if (network.name === 'sepolia') {
    expect(coprocessorConfig.ACLAddress).to.eq(TestnetACLAddress);
  } else if (network.name === 'mainnet') {
    expect(coprocessorConfig.ACLAddress).to.eq(MainnetACLAddress);
  } else {
    expect(coprocessorConfig.ACLAddress).to.not.eq(TestnetACLAddress);
    expect(coprocessorConfig.ACLAddress).to.not.eq(MainnetACLAddress);
  }

  return { fheTestContract, fheTestContractAddress };
}

describe('FHETest test suite', function () {
  let signers: Signers;
  let fheTestContract: FHETest;
  let fheTestContractAddress: string;

  before(async function () {
    const ethSigners: HardhatEthersSigner[] = await ethers.getSigners();
    signers = { alice: ethSigners[0] };

    let d;
    // Only Sepolia
    if (fhevm.isCleartext) {
      d = await deployFixtureMock();
    } else {
      d = await deployFixture();
    }

    fheTestContract = d.fheTestContract;
    fheTestContractAddress = d.fheTestContractAddress;
  });

  //////////////////////////////////////////////////////////////////////////////
  // check config
  //////////////////////////////////////////////////////////////////////////////

  it('FHETest:check:', async function () {
    this.timeout(4 * 40000);

    console.log(`Checking ${network.name}...`);
    console.log(`FHETest=${fheTestContractAddress}`);

    const coprocessorConfig = await fhevm.getCoprocessorConfig(fheTestContractAddress);
    if (network.name === 'sepolia') {
      expect(coprocessorConfig.ACLAddress).to.eq(TestnetACLAddress);
    } else if (network.name === 'devnet') {
      expect(coprocessorConfig.ACLAddress).to.not.eq(TestnetACLAddress);
    } else if (network.name === 'hardhat') {
      expect(coprocessorConfig.ACLAddress).to.eq(LocalACLAddress);
    }

    expect(await fheTestContract.CONTRACT_NAME()).to.eq('FHETestv1');

    const cfg = await fheTestContract.getCoprocessorConfig();
    expect(coprocessorConfig.ACLAddress).to.eq(cfg.ACLAddress);
    expect(coprocessorConfig.CoprocessorAddress).to.eq(cfg.CoprocessorAddress);
    expect(coprocessorConfig.KMSVerifierAddress).to.eq(cfg.KMSVerifierAddress);
  });

  //////////////////////////////////////////////////////////////////////////////
  // encrypt
  //////////////////////////////////////////////////////////////////////////////

  it('FHETest:encrypt:', async function () {
    this.timeout(4 * 40000);

    const inc = 123;

    console.log(`Encrypting ${inc}...`);
    const enc = await fhevm.createEncryptedInput(fheTestContractAddress, signers.alice.address).add32(inc).encrypt();

    console.log(`FHETest=${fheTestContractAddress}`);
    console.log(`User=${signers.alice.address}`);
    console.log(`Handle=${ethers.hexlify(enc.handles[0])}`);
    console.log(`InputProof=${ethers.hexlify(enc.inputProof)}`);
  });

  //////////////////////////////////////////////////////////////////////////////
  // addEuint32
  //////////////////////////////////////////////////////////////////////////////

  it('FHETest:addEuint32:', async function () {
    const inc = 456;

    this.timeout(4 * 40000);

    console.log(`Encrypting ${inc}...`);
    const enc = await fhevm.createEncryptedInput(fheTestContractAddress, signers.alice.address).add32(inc).encrypt();

    const handle = enc.handles[0];
    const proof = enc.inputProof;
    const handleHex = ethers.toBeHex(ethers.toBigInt(handle));

    console.log(`handle: ${handleHex}`);
    console.log(`proof : ${ethers.toBeHex(ethers.toBigInt(proof))}`);

    console.log(`addEuint32()...`);
    const tx = await fheTestContract.connect(signers.alice).addEuint32(handle, proof);
    const receipt = await tx.wait();

    console.log(`Tx: ${receipt?.hash}`);

    const theHandle = await fheTestContract.connect(signers.alice).getEuint32();

    console.log(`new handle: ${theHandle}`);
  });

  //////////////////////////////////////////////////////////////////////////////
  // addEuint32 and publicDecrypt
  //////////////////////////////////////////////////////////////////////////////

  it('FHETest:addEuint32:publicDecrypt:', async function () {
    const inc = 456;

    this.timeout(4 * 40000);

    console.log(`Encrypting ${inc}...`);
    const enc = await fhevm.createEncryptedInput(fheTestContractAddress, signers.alice.address).add32(inc).encrypt();

    const handle = enc.handles[0];
    const proof = enc.inputProof;
    const handleHex = ethers.toBeHex(ethers.toBigInt(handle));

    console.log(`handle: ${handleHex}`);
    console.log(`proof : ${ethers.toBeHex(ethers.toBigInt(proof))}`);

    console.log(`addEuint32()...`);
    let tx = await fheTestContract.connect(signers.alice).addEuint32(handle, proof);
    let receipt = await tx.wait();
    console.log(`Tx: ${receipt?.hash}`);

    console.log(`makePubliclyDecryptableEuint32()...`);
    tx = await fheTestContract.connect(signers.alice).makePubliclyDecryptableEuint32();
    receipt = await tx.wait();
    console.log(`Tx: ${receipt?.hash}`);

    const theHandle = await fheTestContract.connect(signers.alice).getEuint32();

    console.log(`publicDecryptEuint()...`);
    const res = await fhevm.publicDecryptEuint(FhevmType.euint32, theHandle);

    console.log(`new handle: ${theHandle}`);
    console.log(`new clear value: ${res} ${456 + 456}`);
  });

  //////////////////////////////////////////////////////////////////////////////
  // getEuint32 no decryption
  //////////////////////////////////////////////////////////////////////////////

  it('FHETest:getEuint32:', async function () {
    this.timeout(4 * 40000);

    console.log(`getEuint32...`);

    const enc = await fheTestContract.connect(signers.alice).getEuint32();

    console.log(`Handle: ${enc}`);
  });

  //////////////////////////////////////////////////////////////////////////////
  // userDecrypt
  //////////////////////////////////////////////////////////////////////////////

  it('FHETest:userDecrypt:', async function () {
    this.timeout(4 * 40000);

    console.log(`getEuint32...`);

    const enc = await fheTestContract.connect(signers.alice).getEuint32();

    console.log(`Handle: ${enc}`);

    try {
      const clear = await fhevm.userDecryptEuint(FhevmType.euint32, enc, fheTestContractAddress, signers.alice);

      console.log(`Clear: ${clear}`);
    } catch (e) {
      console.log(e);
      console.log((e as { cause: unknown }).cause);
      throw e;
    }
  });

  //////////////////////////////////////////////////////////////////////////////
  // addEuint32 and userDecrypt
  //////////////////////////////////////////////////////////////////////////////

  it('FHETest:addEuint32:userDecrypt:', async function () {
    this.timeout(4 * 40000);

    const inc: bigint = 1n;

    let step = 1;
    const steps = 10;

    console.log(`${step++}/${steps} Encrypting '0'...`);
    const encryptedZero = await fhevm
      .createEncryptedInput(fheTestContractAddress, signers.alice.address)
      .add32(0)
      .encrypt();

    console.log(
      `${step++}/${steps} Call addEuint32(0) FHETest=${fheTestContractAddress} handle=${ethers.hexlify(encryptedZero.handles[0])} signer=${signers.alice.address}...`,
    );
    let tx = await fheTestContract
      .connect(signers.alice)
      .addEuint32(encryptedZero.handles[0], encryptedZero.inputProof);
    await tx.wait();

    console.log(`${step++}/${steps} Call FHETest.getEuint32()...`);
    const encryptedEuint32BeforeInc = await fheTestContract.getEuint32();
    expect(encryptedEuint32BeforeInc).to.not.eq(ethers.ZeroHash);

    console.log(`${step++}/${steps} Decrypting FHETest.getEuint32()=${encryptedEuint32BeforeInc}...`);
    const clearEuint32BeforeInc = await fhevm.userDecryptEuint(
      FhevmType.euint32,
      encryptedEuint32BeforeInc,
      fheTestContractAddress,
      signers.alice,
    );
    console.log(`${step++}/${steps} Clear FHETest.getEuint32()=${clearEuint32BeforeInc}`);

    console.log(`${step++}/${steps} Encrypting '${inc}'...`);
    const encryptedOne = await fhevm
      .createEncryptedInput(fheTestContractAddress, signers.alice.address)
      .add32(inc)
      .encrypt();

    console.log(
      `${step++}/${steps} Call addEuint32(1) FHETest=${fheTestContractAddress} handle=${ethers.hexlify(encryptedOne.handles[0])} signer=${signers.alice.address}...`,
    );
    tx = await fheTestContract.connect(signers.alice).addEuint32(encryptedOne.handles[0], encryptedOne.inputProof);
    await tx.wait();

    console.log(`${step++}/${steps} Call FHETest.getEuint32()...`);
    const encryptedEuint32AfterInc = await fheTestContract.getEuint32();

    console.log(`${step++}/${steps} Decrypting FHETest.getEuint32()=${encryptedEuint32AfterInc}...`);
    const clearEuint32AfterInc = await fhevm.userDecryptEuint(
      FhevmType.euint32,
      encryptedEuint32AfterInc,
      fheTestContractAddress,
      signers.alice,
    );
    console.log(`${step}/${steps} Clear FHETest.getEuint32()=${clearEuint32AfterInc}`);

    expect(clearEuint32AfterInc - clearEuint32BeforeInc).to.eq(inc);
  });
});
