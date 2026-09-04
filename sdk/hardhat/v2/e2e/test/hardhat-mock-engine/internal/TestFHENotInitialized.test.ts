import { expect } from 'chai';
import * as hre from 'hardhat';

import type { TestFHENotInitialized } from '../../../typechain-types';
import { type Signers, getSigners, initSigners } from '../signers';
import { deployTestFHENotInitializedFixture } from './TestFHENotInitialized.fixture';

describe('TestFHENotInitialized', function () {
  let signers: Signers;
  let testFHENotInitialized: TestFHENotInitialized;
  let testFHENotInitializedAddress: string;

  before(async function () {
    await initSigners();
    signers = await getSigners();
  });

  beforeEach(async function () {
    const contract = await deployTestFHENotInitializedFixture(signers.alice);
    testFHENotInitializedAddress = await contract.getAddress();
    testFHENotInitialized = contract;
  });

  it('Assertion should fail if the FHE contract address is uninitialized', async function () {
    // Error message without contract name
    await expect(hre.fhevm.assertCoprocessorInitialized(testFHENotInitializedAddress)).to.be.rejectedWith(
      new RegExp(
        '^Contract at (.+) is not initialized for FHE operations. Make sure it either inherits from @fhevm\\/solidity\\/config\\/ZamaConfig.sol:EthereumConfig or explicitly calls FHE.setCoprocessor\\(\\) in its constructor.',
      ),
    );

    // Error message including contract name
    await expect(
      hre.fhevm.assertCoprocessorInitialized(testFHENotInitializedAddress, 'TestFHENotInitialized'),
    ).to.be.rejectedWith(
      new RegExp(
        '^Contract TestFHENotInitialized at (.+) is not initialized for FHE operations. Make sure it either inherits from @fhevm\\/solidity\\/config\\/ZamaConfig.sol:EthereumConfig or explicitly calls FHE.setCoprocessor\\(\\) in its constructor.',
      ),
    );
  });

  it('Assertion should fail if the FHE contract is uninitialized', async function () {
    // Error message without contract name
    await expect(hre.fhevm.assertCoprocessorInitialized(testFHENotInitialized)).to.be.rejectedWith(
      new RegExp(
        '^Contract at (.+) is not initialized for FHE operations. Make sure it either inherits from @fhevm\\/solidity\\/config\\/ZamaConfig.sol:EthereumConfig or explicitly calls FHE.setCoprocessor\\(\\) in its constructor.',
      ),
    );

    // Error message including contract name
    await expect(
      hre.fhevm.assertCoprocessorInitialized(testFHENotInitialized, 'TestFHENotInitialized'),
    ).to.be.rejectedWith(
      new RegExp(
        '^Contract TestFHENotInitialized at (.+) is not initialized for FHE operations. Make sure it either inherits from @fhevm\\/solidity\\/config\\/ZamaConfig.sol:EthereumConfig or explicitly calls FHE.setCoprocessor\\(\\) in its constructor.',
      ),
    );
  });
});
