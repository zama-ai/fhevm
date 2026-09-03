// `hardhat fhevm public-decrypt <type> <handle>`: prints the cleartext of a publicly decryptable handle.
// Opens the connection `--network` selects (a bare `getOrCreate()` honours it).

import type { NewTaskActionFunction } from 'hardhat/types/tasks';

import { isFhevmEuint } from '../internal/fheType.js';
import { FhevmType, type HardhatFhevmRuntimeEnvironment } from '../types.js';
import { parseHandleArg, parseTypeArg } from './args.js';

type Args = { type: string; handle: string };

const publicDecryptAction: NewTaskActionFunction<Args> = async ({ type, handle }, hre) => {
  const fhevmType = parseTypeArg(type);
  const handleBytes32 = parseHandleArg(handle);
  const { fhevm } = await hre.network.getOrCreate();
  const value = await publicDecrypt(fhevm, fhevmType, handleBytes32);
  console.log(String(value));
  return value;
};

async function publicDecrypt(
  fhevm: HardhatFhevmRuntimeEnvironment,
  fhevmType: FhevmType,
  handle: `0x${string}`,
): Promise<bigint | boolean | string> {
  if (isFhevmEuint(fhevmType)) return fhevm.publicDecryptEuint(fhevmType, handle);
  if (fhevmType === FhevmType.ebool) return fhevm.publicDecryptEbool(handle);
  // Every euint and ebool handled above: what is left is eaddress.
  return fhevm.publicDecryptEaddress(handle);
}

export default publicDecryptAction;
