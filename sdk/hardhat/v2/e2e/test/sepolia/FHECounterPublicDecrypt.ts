import { FhevmType } from '@fhevm/hardhat-plugin';
import type { HardhatEthersSigner } from '@nomicfoundation/hardhat-ethers/signers';
import { expect } from 'chai';
import { ethers, fhevm, network } from 'hardhat';

import type { FHECounterPublicDecrypt } from '../../typechain-types';
import {
  DevnetFHECounterPublicDecryptAddress,
  TestnetACLAddress,
  TestnetFHECounterPublicDecryptAddress,
} from './addresses';

// Sepolia
// =======
// npx hardhat test --grep "Sepolia:FHECounterPublicDecrypt:Check:" --network sepolia
// npx hardhat test --grep "Sepolia:FHECounterPublicDecrypt:EncryptOnly:" --network sepolia
// npx hardhat test --grep "Sepolia:FHECounterPublicDecrypt:Increment:" --network sepolia
// npx hardhat test --grep "Sepolia:FHECounterPublicDecrypt:GetCount:" --network sepolia
// npx hardhat test --grep "Sepolia:FHECounterPublicDecrypt:PublicDecrypt:" --network sepolia
// npx hardhat test --grep "Sepolia:FHECounterPublicDecrypt:Verify:" --network sepolia
// npx hardhat test --grep "Sepolia:FHECounterPublicDecrypt:IncrementAndPublicDecrypt:" --network sepolia

// Devnet
// ======
// npx hardhat test --grep "Sepolia:FHECounterPublicDecrypt:Check:" --network devnet
// npx hardhat test --grep "Sepolia:FHECounterPublicDecrypt:GetCount:" --network devnet
// npx hardhat test --grep "Sepolia:FHECounterPublicDecrypt:PublicDecrypt:" --network devnet
// npx hardhat test --grep "Sepolia:FHECounterPublicDecrypt:Verify:" --network devnet
// npx hardhat test --grep "Sepolia:FHECounterPublicDecrypt:IncrementAndPublicDecrypt:" --network devnet

type Signers = {
  alice: HardhatEthersSigner;
};

async function deployFixture(): Promise<{
  readonly fheCounterContract: FHECounterPublicDecrypt;
  readonly fheCounterContractAddress: string;
}> {
  let fheCounterContractAddress;
  if (network.name === 'devnet') {
    fheCounterContractAddress = DevnetFHECounterPublicDecryptAddress;
  } else {
    fheCounterContractAddress = TestnetFHECounterPublicDecryptAddress;
  }
  const fheCounterContract = await ethers.getContractAt('FHECounterPublicDecrypt', fheCounterContractAddress);

  return { fheCounterContract, fheCounterContractAddress };
}

describe('Sepolia:FHECounterPublicDecrypt', function () {
  let signers: Signers;
  let fheCounterContract: FHECounterPublicDecrypt;
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

  it('Sepolia:FHECounterPublicDecrypt:Check:', async function () {
    // Only Sepolia
    if (fhevm.isCleartext) {
      return;
    }

    this.timeout(4 * 40000);

    console.log(`Checking ${network.name}...`);
    console.log(`FHECounterPublicDecrypt=${fheCounterContractAddress}`);

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

  it('Sepolia:FHECounterPublicDecrypt:EncryptOnly:', async function () {
    // Only Sepolia
    if (fhevm.isCleartext) {
      return;
    }

    this.timeout(4 * 40000);

    const inc = 456;

    console.log(`Encrypting ${inc}...`);
    const enc = await fhevm.createEncryptedInput(fheCounterContractAddress, signers.alice.address).add32(inc).encrypt();

    console.log(`FHECounterPublicDecrypt=${fheCounterContractAddress}`);
    console.log(`User=${signers.alice.address}`);
    console.log(`Handle=${ethers.hexlify(enc.handles[0])}`);
    console.log(`InputProof=${ethers.hexlify(enc.inputProof)}`);
  });

  //////////////////////////////////////////////////////////////////////////////
  // Increment
  //////////////////////////////////////////////////////////////////////////////

  it('Sepolia:FHECounterPublicDecrypt:Increment:', async function () {
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

    console.log(`handle: ${ethers.toBeHex(ethers.toBigInt(handle))}`);
    console.log(`proof : ${ethers.toBeHex(ethers.toBigInt(proof))}`);
    console.log(`increment...`);

    const tx = await fheCounterContract.connect(signers.alice).increment(handle, proof);
    const receipt = await tx.wait();

    console.log(`Tx: ${receipt?.hash}`);
  });

  //////////////////////////////////////////////////////////////////////////////
  // GetCount
  //////////////////////////////////////////////////////////////////////////////

  it('Sepolia:FHECounterPublicDecrypt:GetCount:', async function () {
    // Only Sepolia
    if (fhevm.isCleartext) {
      return;
    }

    this.timeout(4 * 40000);

    console.log(`GetCount...`);

    const enc = await fheCounterContract.connect(signers.alice).getCount();

    // Count=0x72be79061de479187921844d862adb503af6991609ff0000000000aa36a70400
    console.log(`Handle: ${enc}`);
  });

  //////////////////////////////////////////////////////////////////////////////
  // PublicDecrypt
  //////////////////////////////////////////////////////////////////////////////

  it('Sepolia:FHECounterPublicDecrypt:PublicDecrypt:', async function () {
    // Only Sepolia
    if (fhevm.isCleartext) {
      return;
    }

    this.timeout(4 * 40000);

    console.log(`GetCount...`);

    const enc = await fheCounterContract.connect(signers.alice).getCount();

    console.log(`Handle: ${enc}`);

    try {
      const publicDecryptResults = await fhevm.publicDecrypt([enc]);

      console.log(`Clear: ${publicDecryptResults.clearValues[enc as `0x${string}`]}`);
      console.log(`abiEncodedClearValues: ${publicDecryptResults.abiEncodedClearValues}`);
      console.log(`decryptionProof: ${publicDecryptResults.decryptionProof}`);
    } catch (e) {
      console.log(e);
      console.log((e as { cause: unknown }).cause);
      throw e;
    }
  });

  //////////////////////////////////////////////////////////////////////////////
  // Verify
  //////////////////////////////////////////////////////////////////////////////

  it('Sepolia:FHECounterPublicDecrypt:Verify:', async function () {
    // Only Sepolia
    if (fhevm.isCleartext) {
      return;
    }

    this.timeout(4 * 40000);

    const enc = await fheCounterContract.connect(signers.alice).getCount();

    try {
      const publicDecryptResults = await fhevm.publicDecrypt([enc]);

      console.log(`Clear: ${publicDecryptResults.clearValues[enc as `0x${string}`]}`);
      console.log(`abiEncodedClearValues: ${publicDecryptResults.abiEncodedClearValues}`);
      console.log(`decryptionProof: ${publicDecryptResults.decryptionProof}`);

      console.log(`Verify...`);

      const tx = await fheCounterContract
        .connect(signers.alice)
        .verify([enc], publicDecryptResults.abiEncodedClearValues, publicDecryptResults.decryptionProof);
      const txReceipt = await tx.wait();
      if (txReceipt?.status === 1) {
        console.log(`Verification succeeded! Tx:${txReceipt.hash}`);
      } else {
        console.log(`Verification failed! Tx:${txReceipt?.hash}`);
      }
    } catch (e) {
      console.log(e);
      console.log((e as { cause: unknown }).cause);
      throw e;
    }
  });

  //////////////////////////////////////////////////////////////////////////////
  // Increment & PublicDecrypt
  //////////////////////////////////////////////////////////////////////////////

  it('Sepolia:FHECounterPublicDecrypt:IncrementAndPublicDecrypt:', async function () {
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
    const clearCountBeforeInc = await fhevm.publicDecryptEuint(FhevmType.euint32, encryptedCountBeforeInc);
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
    const clearCountAfterInc = await fhevm.publicDecryptEuint(FhevmType.euint32, encryptedCountAfterInc);
    console.log(`${step}/${steps} Clear FHECounter.getCount()=${clearCountAfterInc}`);

    expect(clearCountAfterInc - clearCountBeforeInc).to.eq(inc);
  });
});
