import { expect } from 'chai';
import { network } from 'hardhat';

import type { FHECounterPublicDecrypt__factory } from '../../types/ethers-contracts/index.ts';

// The ZamaConfig-trio smoke test deferred since B1c: a contract inheriting `ZamaEthereumConfig`, compiled
// against the addresses `@fhevm/solidity` ships, points at the stack this connection deployed.

const connection = await network.getOrCreate();
const { ethers, fhevm } = connection;

describe('CoprocessorConfig', function () {
  it('a ZamaEthereumConfig contract is initialized against the deployed stack', async function () {
    if (!fhevm.isCleartext) {
      throw new Error(`This hardhat test suite can only run on a cleartext node`);
    }
    const factory: FHECounterPublicDecrypt__factory = await ethers.getContractFactory('FHECounterPublicDecrypt');
    const counter = await factory.deploy();

    await fhevm.assertCoprocessorInitialized(counter, 'FHECounterPublicDecrypt');

    const config = await fhevm.getCoprocessorConfig(counter);
    expect(config.ACLAddress).to.not.eq(ethers.ZeroAddress);
    expect(config.CoprocessorAddress).to.not.eq(ethers.ZeroAddress);
    expect(config.KMSVerifierAddress).to.not.eq(ethers.ZeroAddress);
  });
});
