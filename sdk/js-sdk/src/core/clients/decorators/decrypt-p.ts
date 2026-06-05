import type { FhevmBase } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import { asFhevmClientWith, resolveFhevmTkmsVersion, setResolvedTkmsVersion } from '../../runtime/CoreFhevm-p.js';

////////////////////////////////////////////////////////////////////////////////

export async function _initDecrypt(fhevm: FhevmBase<FhevmChain>): Promise<void> {
  const f = asFhevmClientWith(fhevm, 'decrypt');

  const tkmsVersion = await resolveFhevmTkmsVersion(f);

  await f.runtime.decrypt.initTkmsModule({ tkmsVersion });

  setResolvedTkmsVersion(f, tkmsVersion);
}
