import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { WithEncrypt } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { TfheVersion } from '../../../wasm/tfhe/TfheApi.js';
import { hyperWasmResolveTfheModuleVersion } from '../../runtime/HyperWasmSolver-p.js';

////////////////////////////////////////////////////////////////////////////////

export type GetTfheVersionReturnType = TfheVersion;

////////////////////////////////////////////////////////////////////////////////

export async function getTfheVersion(fhevm: Fhevm<FhevmChain, WithEncrypt>): Promise<GetTfheVersionReturnType> {
  return hyperWasmResolveTfheModuleVersion(fhevm);
}
