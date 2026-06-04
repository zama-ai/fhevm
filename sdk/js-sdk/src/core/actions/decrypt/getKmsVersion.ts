import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { WithEncrypt } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { TkmsVersion } from '../../../wasm/tkms/KmsLibApi.js';
import { hyperWasmResolveTkmsModuleVersion } from '../../runtime/HyperWasmSolver-p.js';

////////////////////////////////////////////////////////////////////////////////

export type GetTkmsVersionReturnType = TkmsVersion;

////////////////////////////////////////////////////////////////////////////////

export async function getTkmsVersion(fhevm: Fhevm<FhevmChain, WithEncrypt>): Promise<GetTkmsVersionReturnType> {
  return hyperWasmResolveTkmsModuleVersion(fhevm);
}
