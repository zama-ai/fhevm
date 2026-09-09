// `fhevm.debugger` — reads cleartexts straight out of the chain, with no ACL check. The test-only
// escape hatch: `userDecrypt`/`publicDecrypt` enforce permissions, an operator test wants to assert
// that `FheAdd(a, b)` produced the right number without arranging for anyone to be allowed to see it.
// The cleartext stack keeps every value on-chain in `CleartextDB`, so this is one `eth_call`.

import { HardhatPluginError } from 'hardhat/plugins';
import { type Address, type Hex, getAddress, toHex } from 'viem';

import { type FhevmNetworkInfo, FhevmType, type FhevmTypeEuint, type HardhatFhevmRuntimeDebugger } from '../types.js';
import { PLUGIN_ID } from './constants.js';
import { type FhevmContractsRepository, isCleartextContractsRepository } from './contracts.js';
import { isFhevmEuint } from './fheType.js';
import { assertHandleIsInitialized, parseFhevmHandle } from './fhevmHandle.js';

export function createDebugger(
  repository: FhevmContractsRepository,
  network: FhevmNetworkInfo,
): HardhatFhevmRuntimeDebugger {
  // `CleartextDB` only exists on the cleartext stack; elsewhere values are really encrypted.
  if (!isCleartextContractsRepository(repository)) {
    throw new HardhatPluginError(
      PLUGIN_ID,
      `fhevm.debugger is only available on a cleartext network — on '${network.networkName}' values are really encrypted. Use fhevm.userDecryptE*() or fhevm.publicDecryptE*() instead.`,
    );
  }
  const db = repository.cleartextDb;

  // The raw cleartext behind a handle, after checking the handle really is of the expected type.
  async function read(handle: Hex, expected: FhevmType, method: string): Promise<bigint> {
    assertHandleIsInitialized(handle);
    const info = parseFhevmHandle(handle);
    if (info.fhevmType !== expected) {
      throw new HardhatPluginError(
        PLUGIN_ID,
        `fhevm.debugger.${method}: handle '${handle}' is a ${info.typeName}, not a ${FhevmType[expected]}.`,
      );
    }
    const value: unknown = await repository.client.readContract({
      address: db.address,
      abi: db.abi,
      functionName: 'get',
      args: [handle],
    });
    if (typeof value !== 'bigint') {
      throw new HardhatPluginError(
        PLUGIN_ID,
        `fhevm.debugger.${method}: CleartextDB.get('${handle}') did not return a uint256.`,
      );
    }
    return value;
  }

  return Object.freeze({
    async decryptEbool(handleBytes32: Hex): Promise<boolean> {
      return (await read(handleBytes32, FhevmType.ebool, 'decryptEbool')) === 1n;
    },
    async decryptEuint(fhevmType: FhevmTypeEuint, handleBytes32: Hex): Promise<bigint> {
      if (!isFhevmEuint(fhevmType)) {
        throw new HardhatPluginError(PLUGIN_ID, `fhevm.debugger.decryptEuint: expected an euint type.`);
      }
      return read(handleBytes32, fhevmType, 'decryptEuint');
    },
    async decryptEaddress(handleBytes32: Hex): Promise<Address> {
      const value = await read(handleBytes32, FhevmType.eaddress, 'decryptEaddress');
      return getAddress(toHex(value, { size: 20 }));
    },
  });
}
