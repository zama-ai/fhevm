import { expect } from 'chai';
import { ethers } from 'hardhat';

import { STORAGE_EXCEPTIONS } from '../../tasks/utils/storageExceptions';

function minorVersion(version: string): string {
  return version.replace(/\.\d+$/, '');
}

describe('STORAGE_EXCEPTIONS', function () {
  // A chain cannot skip a contract version (each reinitializer runs in turn), so once a contract is past the minor
  // version that added an exception, every chain has gone through that change and the exception only weakens the check.
  it('lists only changes made by the current minor version of each contract', async function () {
    for (const { addedIn } of STORAGE_EXCEPTIONS) {
      const contractName = addedIn.split(' ')[0];
      const implementation = await ethers.deployContract(contractName);
      const version: string = await implementation.getFunction('getVersion')();
      expect(
        minorVersion(version),
        `${contractName} is now ${version}: delete the exception added in ${addedIn}`,
      ).to.equal(minorVersion(addedIn));
    }
  });
});
