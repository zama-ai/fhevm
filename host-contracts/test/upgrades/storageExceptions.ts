import { StorageLayout } from '@openzeppelin/upgrades-core';
import { expect } from 'chai';
import hre, { ethers } from 'hardhat';

import { STORAGE_EXCEPTION, layoutOf, majorMinorOf, makesStorageException } from '../../tasks/utils/storageExceptions';

const FIXTURES = 'test/upgrades/storageExceptionFixtures';

async function fixtureLayout(name: string): Promise<StorageLayout> {
  return layoutOf(hre, await ethers.getContractFactory(`${FIXTURES}/${name}/ACL.sol:${name}ACL`));
}

describe('STORAGE_EXCEPTION', function () {
  // The exception stays while a chain may still upgrade from the layout before it. Once the contract's minor version
  // moves past the one that added it, either every chain runs that version or later and the exception is deleted, or
  // the exception is kept on purpose by moving `addedIn`.
  it('is made by the current minor version of its contract', async function () {
    const { addedIn } = STORAGE_EXCEPTION;
    const contractName = addedIn.split(' ')[0];
    const implementation = await ethers.deployContract(contractName);
    const version: string = await implementation.getFunction('getVersion')();
    expect(
      majorMinorOf(version),
      `${contractName} is now ${version}: delete the exception added in ${addedIn}, or move it`,
    ).to.deep.equal(majorMinorOf(addedIn));
  });

  describe('makesStorageException', function () {
    before(async function () {
      await hre.run('compile:specific', { contract: FIXTURES });
    });

    it('accepts the bool to Permission change', async function () {
      expect(makesStorageException(await fixtureLayout('Bool'), await fixtureLayout('Struct'))).to.equal(true);
    });

    it('applies to no other upgrade, so OpenZeppelin checks it as usual', async function () {
      expect(makesStorageException(await fixtureLayout('Struct'), await fixtureLayout('Struct'))).to.equal(false);
      expect(makesStorageException(await fixtureLayout('Struct'), await fixtureLayout('Bool'))).to.equal(false);
    });

    it('refuses the change together with any other incompatible change', async function () {
      const [current, updated] = [await fixtureLayout('Bool'), await fixtureLayout('StructAndOtherChange')];
      expect(() => makesStorageException(current, updated)).to.throw(/allowedForDecryption/);
    });

    it('refuses a Permission whose bool is not its first member', async function () {
      const [current, updated] = [await fixtureLayout('Bool'), await fixtureLayout('StructWrongMembers')];
      expect(() => makesStorageException(current, updated)).to.throw(/must be allowed:t_bool, depth:t_uint48/);
    });
  });
});
