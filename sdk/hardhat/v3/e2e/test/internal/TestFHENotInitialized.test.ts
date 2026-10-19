import { expect } from 'chai';
import { network } from 'hardhat';

import type { TestFHENotInitialized, TestFHENotInitialized__factory } from '../../types/ethers-contracts/index.ts';
import { type Signers, getSigners } from '../utils/signers.ts';

const connection = await network.getOrCreate();
const { ethers, fhevm } = connection;

const NOT_INITIALIZED =
  /^Contract (.+) is not initialized for FHE operations. Make sure it either inherits from @fhevm\/solidity\/config\/ZamaConfig.sol:ZamaEthereumConfig or explicitly calls FHE.setCoprocessor\(\) in its constructor./;

// chai-as-promised is not typed in this suite; the assertion is spelled out instead.
async function expectRejectedWith(promise: Promise<unknown>, pattern: RegExp): Promise<string> {
  let message: string | undefined;
  try {
    await promise;
  } catch (e) {
    message = e instanceof Error ? e.message : String(e);
  }
  if (message === undefined) throw new Error('expected the promise to reject');
  expect(message).to.match(pattern);
  return message;
}

describe('TestFHENotInitialized', function () {
  let signers: Signers;
  let testFHENotInitialized: TestFHENotInitialized;
  let testFHENotInitializedAddress: `0x${string}`;

  before(async function () {
    signers = await getSigners(connection);
  });

  beforeEach(async function () {
    const factory: TestFHENotInitialized__factory = await ethers.getContractFactory('TestFHENotInitialized');
    testFHENotInitialized = await factory.connect(signers.alice).deploy();
    await testFHENotInitialized.waitForDeployment();
    testFHENotInitializedAddress = (await testFHENotInitialized.getAddress()) as `0x${string}`;
  });

  it('Assertion should fail if the FHE contract address is uninitialized', async function () {
    // Error message without contract name
    const bare = await expectRejectedWith(
      fhevm.assertCoprocessorInitialized(testFHENotInitializedAddress),
      NOT_INITIALIZED,
    );
    expect(bare.startsWith(`Contract at ${testFHENotInitializedAddress}`)).to.eq(true);

    // Error message including contract name
    const named = await expectRejectedWith(
      fhevm.assertCoprocessorInitialized(testFHENotInitializedAddress, 'TestFHENotInitialized'),
      NOT_INITIALIZED,
    );
    expect(named.startsWith(`Contract TestFHENotInitialized at ${testFHENotInitializedAddress}`)).to.eq(true);
  });

  it('Assertion should fail if the FHE contract is uninitialized', async function () {
    // The contract object itself resolves through `getAddress()`.
    await expectRejectedWith(fhevm.assertCoprocessorInitialized(testFHENotInitialized), NOT_INITIALIZED);
    await expectRejectedWith(
      fhevm.assertCoprocessorInitialized(testFHENotInitialized, 'TestFHENotInitialized'),
      NOT_INITIALIZED,
    );
  });
});
