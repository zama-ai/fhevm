import type { KmsSigncryptedShare } from '../../types/kms-p.js';
import type { FetchUserDecryptResult } from '../../types/relayer.js';
import { remove0x } from '../../base/string.js';

export function userDecryptResultToKmsSigncryptedShares(
  result: FetchUserDecryptResult,
): readonly KmsSigncryptedShare[] {
  const shares: KmsSigncryptedShare[] = result.map((r) => {
    const share: KmsSigncryptedShare = {
      signature: r.signature,
      payload: r.payload,
      extraData: remove0x(r.extraData),
    };
    return share;
  });
  return shares;
}
