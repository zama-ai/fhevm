import { FhevmType } from '@fhevm/hardhat-plugin';
import type { HardhatEthersSigner } from '@nomicfoundation/hardhat-ethers/signers';
import { expect } from 'chai';
import { ethers, fhevm, network } from 'hardhat';

import type { FHECounterUserDecrypt } from '../../typechain-types';
import {
  DevnetFHECounterUserDecryptAddress,
  TestnetACLAddress,
  TestnetFHECounterUserDecryptAddress,
} from './addresses';

// Sepolia
// =======
// npx hardhat test --grep "Sepolia:FHECounterUserDecrypt:Check:" --network sepolia
// npx hardhat test --grep "Sepolia:FHECounterUserDecrypt:Encrypt:" --network sepolia
// npx hardhat test --grep "Sepolia:FHECounterUserDecrypt:Increment:" --network sepolia
// npx hardhat test --grep "Sepolia:FHECounterUserDecrypt:GetCount:" --network sepolia
// npx hardhat test --grep "Sepolia:FHECounterUserDecrypt:UserDecrypt:" --network sepolia
// npx hardhat test --grep "Sepolia:FHECounterUserDecrypt:IncrementAndUserDecrypt:" --network sepolia

// Devnet
// ======
// npx hardhat test --grep "Sepolia:FHECounterUserDecrypt:Check:" --network devnet
// npx hardhat test --grep "Sepolia:FHECounterUserDecrypt:Encrypt:" --network devnet
// npx hardhat test --grep "Sepolia:FHECounterUserDecrypt:IncrementAndUserDecrypt:" --network devnet

type Signers = {
  alice: HardhatEthersSigner;
};

async function deployFixture(): Promise<{
  readonly fheCounterContract: FHECounterUserDecrypt;
  readonly fheCounterContractAddress: string;
}> {
  let fheCounterContractAddress;
  if (network.name === 'devnet') {
    fheCounterContractAddress = DevnetFHECounterUserDecryptAddress;
  } else {
    fheCounterContractAddress = TestnetFHECounterUserDecryptAddress;
  }
  const fheCounterContract = await ethers.getContractAt('FHECounterUserDecrypt', fheCounterContractAddress);

  const coprocessorConfig = await fhevm.getCoprocessorConfig(fheCounterContractAddress);
  if (network.name !== 'devnet') {
    expect(coprocessorConfig.ACLAddress).to.eq(TestnetACLAddress);
  } else {
    expect(coprocessorConfig.ACLAddress).to.not.eq(TestnetACLAddress);
  }

  return { fheCounterContract, fheCounterContractAddress };
}

describe('Sepolia:FHECounterUserDecrypt:', function () {
  let signers: Signers;
  let fheCounterContract: FHECounterUserDecrypt;
  let fheCounterContractAddress: string;

  before(async function () {
    // Only Sepolia
    if (fhevm.isCleartext) {
      return;
    }

    const ethSigners: HardhatEthersSigner[] = await ethers.getSigners();
    signers = { alice: ethSigners[0] };

    const d = await deployFixture();
    fheCounterContract = d.fheCounterContract;
    fheCounterContractAddress = d.fheCounterContractAddress;
  });

  //////////////////////////////////////////////////////////////////////////////
  // Check config
  //////////////////////////////////////////////////////////////////////////////

  it('Sepolia:FHECounterUserDecrypt:Check:', async function () {
    // Only Sepolia
    if (fhevm.isCleartext) {
      return;
    }

    this.timeout(4 * 40000);

    console.log(`Checking ${network.name}...`);
    console.log(`FHECounterUserDecrypt=${fheCounterContractAddress}`);

    const coprocessorConfig = await fhevm.getCoprocessorConfig(fheCounterContractAddress);
    if (network.name !== 'devnet') {
      expect(coprocessorConfig.ACLAddress).to.eq(TestnetACLAddress);
    } else {
      expect(coprocessorConfig.ACLAddress).to.not.eq(TestnetACLAddress);
    }
  });

  //////////////////////////////////////////////////////////////////////////////
  // Encrypt only
  //////////////////////////////////////////////////////////////////////////////

  it('Sepolia:FHECounterUserDecrypt:EncryptOnly:', async function () {
    // Only Sepolia
    if (fhevm.isCleartext) {
      return;
    }

    this.timeout(4 * 40000);

    const inc = 123;

    console.log(`Encrypting ${inc}...`);
    const enc = await fhevm.createEncryptedInput(fheCounterContractAddress, signers.alice.address).add32(inc).encrypt();

    console.log(`FHECounterUserDecrypt=${fheCounterContractAddress}`);
    console.log(`User=${signers.alice.address}`);
    console.log(`Handle=${ethers.hexlify(enc.handles[0])}`);
    console.log(`InputProof=${ethers.hexlify(enc.inputProof)}`);
  });

  //////////////////////////////////////////////////////////////////////////////
  // Increment
  //////////////////////////////////////////////////////////////////////////////

  it('Sepolia:FHECounterUserDecrypt:Increment:', async function () {
    // Only Sepolia
    if (fhevm.isCleartext) {
      return;
    }

    const inc = 456;

    this.timeout(4 * 40000);

    console.log(`Encrypting ${inc}...`);
    const enc = await fhevm.createEncryptedInput(fheCounterContractAddress, signers.alice.address).add32(inc).encrypt();

    const handle = enc.handles[0];
    const proof = enc.inputProof;
    const handleHex = ethers.toBeHex(ethers.toBigInt(handle));

    console.log(`handle: ${handleHex}`);
    console.log(`proof : ${ethers.toBeHex(ethers.toBigInt(proof))}`);
    console.log(`increment...`);

    const tx = await fheCounterContract.connect(signers.alice).increment(handle, proof);
    const receipt = await tx.wait();

    console.log(`Tx: ${receipt?.hash}`);

    const theHandle = await fheCounterContract.connect(signers.alice).getCount();

    console.log(`new handle: ${theHandle}`);
  });

  //////////////////////////////////////////////////////////////////////////////
  // GetCount no decryption
  //////////////////////////////////////////////////////////////////////////////

  it('Sepolia:FHECounterUserDecrypt:GetCount:', async function () {
    // Only Sepolia
    if (fhevm.isCleartext) {
      return;
    }

    this.timeout(4 * 40000);

    console.log(`GetCount...`);

    const enc = await fheCounterContract.connect(signers.alice).getCount();

    console.log(`Handle: ${enc}`);
  });

  //////////////////////////////////////////////////////////////////////////////
  // userDecrypt
  //////////////////////////////////////////////////////////////////////////////

  it('Sepolia:FHECounterUserDecrypt:UserDecrypt:', async function () {
    // Only Sepolia
    if (fhevm.isCleartext) {
      return;
    }

    this.timeout(4 * 40000);

    console.log(`GetCount...`);

    const enc = await fheCounterContract.connect(signers.alice).getCount();

    console.log(`Handle: ${enc}`);

    try {
      const clear = await fhevm.userDecryptEuint(FhevmType.euint32, enc, fheCounterContractAddress, signers.alice);

      console.log(`Clear: ${clear}`);
    } catch (e) {
      console.log(e);
      console.log((e as { cause: unknown }).cause);
      throw e;
    }
  });

  //////////////////////////////////////////////////////////////////////////////
  // Increment and userDecrypt
  //////////////////////////////////////////////////////////////////////////////

  it('Sepolia:FHECounterUserDecrypt:IncrementAndUserDecrypt:', async function () {
    // Only Sepolia
    if (fhevm.isCleartext) {
      return;
    }

    this.timeout(4 * 40000);

    const inc = 1;

    let step = 1;
    const steps = 10;

    console.log(`${step++}/${steps} Encrypting '0'...`);
    const encryptedZero = await fhevm
      .createEncryptedInput(fheCounterContractAddress, signers.alice.address)
      .add32(0)
      .encrypt();

    console.log(
      `${step++}/${steps} Call increment(0) FHECounter=${fheCounterContractAddress} handle=${ethers.hexlify(encryptedZero.handles[0])} signer=${signers.alice.address}...`,
    );
    let tx = await fheCounterContract
      .connect(signers.alice)
      .increment(encryptedZero.handles[0], encryptedZero.inputProof);
    await tx.wait();

    console.log(`${step++}/${steps} Call FHECounter.getCount()...`);
    const encryptedCountBeforeInc = await fheCounterContract.getCount();
    expect(encryptedCountBeforeInc).to.not.eq(ethers.ZeroHash);

    console.log(`${step++}/${steps} Decrypting FHECounter.getCount()=${encryptedCountBeforeInc}...`);
    const clearCountBeforeInc = await fhevm.userDecryptEuint(
      FhevmType.euint32,
      encryptedCountBeforeInc,
      fheCounterContractAddress,
      signers.alice,
    );
    console.log(`${step++}/${steps} Clear FHECounter.getCount()=${clearCountBeforeInc}`);

    console.log(`${step++}/${steps} Encrypting '${inc}'...`);
    const encryptedOne = await fhevm
      .createEncryptedInput(fheCounterContractAddress, signers.alice.address)
      .add32(inc)
      .encrypt();

    console.log(
      `${step++}/${steps} Call increment(1) FHECounter=${fheCounterContractAddress} handle=${ethers.hexlify(encryptedOne.handles[0])} signer=${signers.alice.address}...`,
    );
    tx = await fheCounterContract.connect(signers.alice).increment(encryptedOne.handles[0], encryptedOne.inputProof);
    await tx.wait();

    console.log(`${step++}/${steps} Call FHECounter.getCount()...`);
    const encryptedCountAfterInc = await fheCounterContract.getCount();

    console.log(`${step++}/${steps} Decrypting FHECounter.getCount()=${encryptedCountAfterInc}...`);
    const clearCountAfterInc = await fhevm.userDecryptEuint(
      FhevmType.euint32,
      encryptedCountAfterInc,
      fheCounterContractAddress,
      signers.alice,
    );
    console.log(`${step}/${steps} Clear FHECounter.getCount()=${clearCountAfterInc}`);

    expect(clearCountAfterInc - clearCountBeforeInc).to.eq(inc);
  });
});
