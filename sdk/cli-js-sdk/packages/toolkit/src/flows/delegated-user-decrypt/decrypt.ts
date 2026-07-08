import type { Hex } from "viem";

import type { WalletContext } from "../../config";
import { decryptUserValues } from "../../fhevm/user-decrypt";
import type { ProgressReporter } from "../../shared/progress";
import type { NetworkName, UserDecryptResult } from "../../types";

export const decryptDelegatedHandles = (
  delegateContext: WalletContext,
  options: {
    encryptedValues: readonly Hex[];
    delegatorAddress: Hex;
    durationDays: number;
    network: NetworkName;
    includeValidationArtifact?: boolean;
    onProgress?: ProgressReporter;
  },
): Promise<UserDecryptResult> =>
  decryptUserValues(delegateContext, {
    encryptedValues: options.encryptedValues,
    signer: delegateContext.account,
    ownerAddress: options.delegatorAddress,
    durationDays: options.durationDays,
    network: options.network,
    includeValidationArtifact: options.includeValidationArtifact,
    onProgress: options.onProgress,
  });
