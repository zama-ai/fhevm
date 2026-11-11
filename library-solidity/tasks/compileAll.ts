// Compiles every contract library-solidity uses: lib/, config/, the host contracts (fhevmTemp/contracts) and
// examples/. It is the only way to prepare fhevmTemp/, which the `build` override in hardhat.config.ts requires.
import { execFileSync } from 'node:child_process';
import path from 'node:path';

import type { HardhatRuntimeEnvironment } from 'hardhat/types/hre';

import { FHEVM_HOST_ADDRESSES_SOL, copyHostContracts } from './hostContracts.js';
import { assertHostAddressesMatchZamaConfig } from './zamaConfigAddresses.js';

export default async function compileAll(_taskArguments: object, hre: HardhatRuntimeEnvironment): Promise<void> {
  copyHostContracts();

  // fhevmTemp/addresses/FHEVMHostAddresses.sol, required to compile the host contracts and examples/. Until the host
  // deployment is ported (step 1.5), it is the host-contracts copy, generated if missing (see the Makefile).
  execFileSync('make', ['ensure-addresses'], {
    cwd: path.resolve(path.dirname(FHEVM_HOST_ADDRESSES_SOL), '../..'),
    stdio: 'inherit',
  });

  // The addresses must be the ones ZamaConfig.sol hardcodes for the chain the tests run on (the in-process network),
  // or contracts using ZamaConfig would not reach the host contracts.
  const defaultNetwork = hre.config.networks.default;
  if (!('chainId' in defaultNetwork) || defaultNetwork.chainId === undefined) {
    throw new Error('The default network has no chain id: cannot check the host addresses against ZamaConfig.sol');
  }
  assertHostAddressesMatchZamaConfig(FHEVM_HOST_ADDRESSES_SOL, BigInt(defaultNetwork.chainId));

  // One full build: every source folder is compiled, so all artifacts and types stay consistent.
  await hre.tasks.getTask('build').run({});
}
