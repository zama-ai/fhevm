import { readValidations } from '@openzeppelin/hardhat-upgrades/dist/utils/validations';
import { StorageLayout, getStorageLayout, getVersion } from '@openzeppelin/upgrades-core';
import { expect } from 'chai';
import hre, { ethers } from 'hardhat';

import { STORAGE_EXCEPTIONS, storageExceptionFor } from '../../tasks/utils/storageExceptions';

const FIXTURES = 'test/upgrades/storageExceptionFixtures';

function minorVersion(version: string): string {
  return version.replace(/\.\d+$/, '');
}

async function fixtureLayout(name: string): Promise<StorageLayout> {
  const factory = await ethers.getContractFactory(`${FIXTURES}/${name}/ACL.sol:ACL`);
  return getStorageLayout(await readValidations(hre), getVersion(factory.bytecode));
}

describe('STORAGE_EXCEPTIONS', function () {
  // An exception stays while a chain may still upgrade from the layout before it. Once the contract's minor version
  // moves past the one that added it, either every chain runs that version or later and the exception is deleted, or
  // the exception is kept on purpose by moving `addedIn`.
  it('lists only changes made by the current minor version of each contract', async function () {
    for (const { addedIn } of STORAGE_EXCEPTIONS) {
      const contractName = addedIn.split(' ')[0];
      const implementation = await ethers.deployContract(contractName);
      const version: string = await implementation.getFunction('getVersion')();
      expect(
        minorVersion(version),
        `${contractName} is now ${version}: delete the exception added in ${addedIn}, or move it`,
      ).to.equal(minorVersion(addedIn));
    }
  });

  describe('storageExceptionFor', function () {
    before(async function () {
      await hre.run('compile:specific', { contract: FIXTURES });
    });

    it('accepts the bool to Permission change', async function () {
      const exception = storageExceptionFor(await fixtureLayout('Bool'), await fixtureLayout('Struct'));
      expect(exception?.field).to.equal('persistedAllowedPairs');
    });

    it('applies to no other upgrade, so OpenZeppelin checks it as usual', async function () {
      expect(storageExceptionFor(await fixtureLayout('Struct'), await fixtureLayout('Struct'))).to.equal(undefined);
      expect(storageExceptionFor(await fixtureLayout('Struct'), await fixtureLayout('Bool'))).to.equal(undefined);
    });

    it('refuses the change together with any other incompatible change', async function () {
      const [current, updated] = [await fixtureLayout('Bool'), await fixtureLayout('StructAndOtherChange')];
      expect(() => storageExceptionFor(current, updated)).to.throw(/allowedForDecryption/);
    });

    it('refuses a Permission whose bool is not its first member', async function () {
      const [current, updated] = [await fixtureLayout('Bool'), await fixtureLayout('StructWrongMembers')];
      expect(() => storageExceptionFor(current, updated)).to.throw(/must be allowed:t_bool, depth:t_uint48/);
    });
  });
});
