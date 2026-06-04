import type { Hex } from "viem";

import type { WalletContext } from "../../config";
import { decryptUserValues } from "../../fhevm/user-decrypt";
import type { ProgressReporter } from "../../shared/progress";
import type { UserDecryptResult } from "../../types";

export const decryptDelegatedHandles = (
  delegateContext: WalletContext,
  options: {
    encryptedValues: readonly Hex[];
    delegatorAddress: Hex;
    durationDays: number;
    onProgress?: ProgressReporter;
  },
): Promise<UserDecryptResult> =>
  decryptUserValues(delegateContext, {
    encryptedValues: options.encryptedValues,
    signer: delegateContext.account,
    ownerAddress: options.delegatorAddress,
    durationDays: options.durationDays,
    onProgress: options.onProgress,
  });
