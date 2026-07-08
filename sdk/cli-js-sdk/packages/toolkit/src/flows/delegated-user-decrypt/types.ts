import type { Hex } from "viem";

import type { DelegationStatus } from "../../acl/delegation";
import type { ClientOptions } from "../../config";
import type { ProgressReporter } from "../../shared/progress";
import type { FheValueType, UserDecryptResult } from "../../types";

/**
 * Shared options for delegated user decryption.
 *
 * `privateKey`/`mnemonic` identify the delegate. Delegator credentials are
 * optional only for flows that can use an existing ACL delegation and a
 * supplied delegator address.
 */
export type DelegatedUserDecryptBaseOptions = ClientOptions &
  Readonly<{
    type: FheValueType;
    contractAddress?: Hex;
    delegatorAddress?: Hex;
    delegatorPrivateKey?: Hex;
    delegatorMnemonic?: string;
    privateKey?: Hex;
    mnemonic?: string;
    durationDays: number;
    delegationDurationDays: number;
    includeValidationArtifact?: boolean;
    onProgress?: ProgressReporter;
  }>;

/** User-decrypt result augmented with the ACL delegation state used by the flow. */
export type DelegatedUserDecryptResult = UserDecryptResult &
  Readonly<{
    delegatorAddress: Hex;
    delegateAddress: Hex;
    delegation: DelegationStatus;
  }>;
