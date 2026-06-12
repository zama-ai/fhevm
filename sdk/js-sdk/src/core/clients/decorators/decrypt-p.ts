import type { FhevmBase } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import { asFhevmClientWith, setResolvedTkmsVersion } from '../../runtime/CoreFhevm-p.js';
import { ensureResolvedProtocolVersion, resolveFhevmTkmsVersion } from '../../runtime/resolveFhevmVersions-p.js';

////////////////////////////////////////////////////////////////////////////////

export async function _initDecrypt(fhevm: FhevmBase<FhevmChain>): Promise<void> {
  const f = asFhevmClientWith(fhevm, 'decrypt');

  await ensureResolvedProtocolVersion(f);

  const tkmsVersion = await resolveFhevmTkmsVersion(f);

  await f.runtime.decrypt.initTkmsModule({ tkmsVersion });

  setResolvedTkmsVersion(f, tkmsVersion);
}
