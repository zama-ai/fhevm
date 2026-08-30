import { FhevmType } from '@fhevm/hardhat-plugin';
import type { HardhatEthersSigner } from '@nomicfoundation/hardhat-ethers/signers';
import { expect } from 'chai';
import { ethers, fhevm } from 'hardhat';

import type { TestACL, TestACL__factory } from '../../../typechain-types';

type Signers = {
  deployer: HardhatEthersSigner;
  alice: HardhatEthersSigner;
  bob: HardhatEthersSigner;
};

async function deployFixture(): Promise<{
  readonly contract: TestACL;
  readonly contractAddress: string;
}> {
  const factory: TestACL__factory = await ethers.getContractFactory('TestACL');
  const contract = (await factory.deploy()) as TestACL;
  const contractAddress = await contract.getAddress();

  return { contract, contractAddress };
}

describe('TestACL', function () {
  let signers: Signers;
  let contract: TestACL;
  let contractAddress: string;

  before(async function () {
    const ethSigners: HardhatEthersSigner[] = await ethers.getSigners();
    signers = { deployer: ethSigners[0], alice: ethSigners[1], bob: ethSigners[2] };
  });

  beforeEach(async () => {
    // Check whether the tests are running against an FHEVM mock environment
    if (!fhevm.isCleartext) {
      throw new Error(`This hardhat test suite can only run on a cleartext node`);
    }
    ({ contract, contractAddress } = await deployFixture());
  });

  it('encrypted count should be uninitialized after deployment', async function () {
    const encryptedCount = await contract.getCount();
    // Expect initial count to be bytes32(0) after deployment,
    // (meaning the encrypted count value is uninitialized)
    expect(encryptedCount).to.eq(ethers.ZeroHash);
  });

  it('Alice increment the counter by 1 using Alice encrypted input', async function () {
    const encryptedCountBeforeInc = await contract.getCount();
    expect(encryptedCountBeforeInc).to.eq(ethers.ZeroHash);
    //const clearCountBeforeInc = 0;

    // Encrypt constant 1 as a euint32
    const clearOne = 1;
    const encryptedOne = await fhevm
      .createEncryptedInput(contractAddress, signers.alice.address)
      .add32(clearOne)
      .encrypt();

    const tx = await contract.connect(signers.alice).increment1(encryptedOne.handles[0], encryptedOne.inputProof);
    await tx.wait();

    const encryptedCountAfterInc = await contract.getCount();
    console.log(encryptedCountAfterInc);

    const clearCountAlice = await fhevm.userDecryptEuint(
      FhevmType.euint32, // Specify the encrypted type
      encryptedCountAfterInc,
      contractAddress, // The contract address
      signers.alice, // The user wallet
    );

    expect(clearCountAlice).to.eq(0 + clearOne);
  });

  it('Bob cannot increment the counter by 1 using Alice encrypted input', async function () {
    const encryptedCountBeforeInc = await contract.getCount();
    expect(encryptedCountBeforeInc).to.eq(ethers.ZeroHash);

    // Encrypt constant 1 as a euint32
    const clearOne = 1;
    const encryptedOne = await fhevm
      .createEncryptedInput(contractAddress, signers.alice.address)
      .add32(clearOne)
      .encrypt();

    await expect(
      contract.connect(signers.bob).increment1(encryptedOne.handles[0], encryptedOne.inputProof),
    ).to.be.revertedWithCustomError(...fhevm.revertedWithCustomErrorArgs('InputVerifier', 'InvalidSigner'));
  });

  it('Bob successfully increments the counter by 1 using Alice encrypted input', async function () {
    const encryptedCountBeforeInc = await contract.getCount();
    expect(encryptedCountBeforeInc).to.eq(ethers.ZeroHash);

    // Encrypt constant 1 as a euint32
    const clearOne = 1;
    const encryptedOne = await fhevm
      .createEncryptedInput(contractAddress, signers.alice.address)
      .add32(clearOne)
      .encrypt();

    const tx = await contract
      .connect(signers.bob)
      .increment2(signers.alice.address, encryptedOne.handles[0], encryptedOne.inputProof);
    await tx.wait();

    const encryptedCountAfterInc = await contract.getCount();

    const clearCountAlice = await fhevm.userDecryptEuint(
      FhevmType.euint32, // Specify the encrypted type
      encryptedCountAfterInc,
      contractAddress, // The contract address
      signers.alice, // The user wallet
    );

    const clearCountBob = await fhevm.userDecryptEuint(
      FhevmType.euint32, // Specify the encrypted type
      encryptedCountAfterInc,
      contractAddress, // The contract address
      signers.bob, // The user wallet
    );

    expect(clearCountAlice).to.eq(0 + clearOne);
    expect(clearCountBob).to.eq(0 + clearOne);
  });
});
