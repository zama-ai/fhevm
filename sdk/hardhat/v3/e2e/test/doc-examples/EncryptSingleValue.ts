import { FhevmType, type HardhatFhevmRuntimeEnvironment } from '@fhevm/hardhat-plugin-v3';
import type { HardhatEthersSigner } from '@nomicfoundation/hardhat-ethers/types';
import { expect } from 'chai';
import { network } from 'hardhat';
import type { LocalAccount } from 'viem';

import type { EncryptSingleValue, EncryptSingleValue__factory } from '../../types/ethers-contracts/index.ts';
import { expectRejectedWith } from '../utils/expect.ts';
import { getAccounts, getSigners } from '../utils/signers.ts';

const connection = await network.getOrCreate();
const { ethers } = connection;

type Hex = `0x${string}`;

// The doc example's cast: `owner` deploys, `alice` is the user (accounts #0 and #1); the user's viem
// account signs the decryption permit.
type Signers = { owner: HardhatEthersSigner; alice: HardhatEthersSigner };
type Accounts = { alice: LocalAccount };

async function deployFixture(): Promise<{
  readonly encryptSingleValue: EncryptSingleValue;
  readonly encryptSingleValueAddress: Hex;
}> {
  // Contracts are deployed using the first signer/account by default
  const factory: EncryptSingleValue__factory = await ethers.getContractFactory('EncryptSingleValue');
  const encryptSingleValue = await factory.deploy();
  const encryptSingleValueAddress = (await encryptSingleValue.getAddress()) as Hex;

  return { encryptSingleValue, encryptSingleValueAddress };
}

/**
 * This trivial example demonstrates the FHE encryption mechanism
 * and highlights a common pitfall developers may encounter.
 */
describe('EncryptSingleValue', function () {
  let contract: EncryptSingleValue;
  let contractAddress: Hex;
  let signers: Signers;
  let accounts: Accounts;

  before(async function () {
    // Check whether the tests are running against an FHEVM mock environment
    if (!connection.fhevm.isCleartext) {
      throw new Error(`This hardhat test suite can only run on a cleartext node`);
    }

    const suiteSigners = await getSigners(connection);
    const suiteAccounts = getAccounts();
    signers = { owner: suiteSigners.alice, alice: suiteSigners.bob };
    accounts = { alice: suiteAccounts.bob };
  });

  beforeEach(async function () {
    // Deploy a new contract each time we run a new test
    const deployment = await deployFixture();
    contractAddress = deployment.encryptSingleValueAddress;
    contract = deployment.encryptSingleValue;
  });

  // ✅ Test should succeed
  it('encryption should succeed', async function () {
    // Use the FHEVM Hardhat plugin runtime environment
    // to perform FHEVM input encryptions.
    const fhevm: HardhatFhevmRuntimeEnvironment = connection.fhevm;

    // 🔐 Encryption Process:
    // Values are encrypted locally and bound to a specific contract/user pair.
    // This grants the bound contract FHE permissions to receive and process the encrypted value,
    // but only when it is sent by the bound user.
    const input = fhevm.createEncryptedInput(contractAddress, signers.alice.address as Hex);

    // Add a uint32 value to the list of values to encrypt locally.
    input.add32(123456);

    // Perform the local encryption. This operation produces two components:
    // 1. `handles`: an array of FHEVM handles. In this case, a single handle associated with the
    //    locally encrypted uint32 value `123456`.
    // 2. `inputProof`: a zero-knowledge proof that attests the `handles` are cryptographically
    //    bound to the pair `[contractAddress, signers.alice.address]`.
    const enc = await input.encrypt();

    // a 32-bytes FHEVM handle that represents a future Solidity `euint32` value.
    const [inputEuint32] = enc.handles;
    if (inputEuint32 === undefined) throw new Error('encrypt() returned no handle');
    const inputProof = enc.inputProof;

    // Now `signers.alice.address` can send the encrypted value and its associated zero-knowledge proof
    // to the smart contract deployed at `contractAddress`.
    const tx = await contract.connect(signers.alice).initialize(inputEuint32, inputProof);
    await tx.wait();

    // Let's try to decrypt it to check that everything is ok!
    const encryptedUint32 = (await contract.encryptedUint32()) as Hex;

    const clearUint32 = await fhevm.userDecryptEuint(
      FhevmType.euint32, // Specify the encrypted type
      encryptedUint32,
      contractAddress, // The contract address
      accounts.alice, // The user account
    );

    expect(clearUint32).to.equal(123456n);
  });

  // ❌ This test illustrates a very common pitfall
  it('encryption should fail', async function () {
    const fhevm: HardhatFhevmRuntimeEnvironment = connection.fhevm;

    const enc = await fhevm
      .createEncryptedInput(contractAddress, signers.alice.address as Hex)
      .add32(123456)
      .encrypt();

    const [inputEuint32] = enc.handles;
    if (inputEuint32 === undefined) throw new Error('encrypt() returned no handle');
    const inputProof = enc.inputProof;

    // Here is a very common error !
    // `contract.initialize` will sign the Ethereum transaction using user `signers.owner`
    // instead of `signers.alice`.
    //
    // In the Solidity contract the following is checked:
    // - Is the contract allowed to manipulate `inputEuint32`? Answer is: ✅ yes!
    // - Is the sender allowed to manipulate `inputEuint32`? Answer is: ❌ no! Only `signers.alice` is!
    //
    // The plugin explains the revert: it is the InputVerifier's `InvalidSigner`.
    await expectRejectedWith(contract.initialize(inputEuint32, inputProof), /InvalidSigner/);
  });
});
