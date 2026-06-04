import type { Hex } from "viem";

import type { DelegationStatus } from "../../acl/delegation";
import type { ClientOptions } from "../../config";
import type { ProgressReporter } from "../../shared/progress";
import type { FheValueType, UserDecryptResult } from "../../types";

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
    onProgress?: ProgressReporter;
  }>;

export type DelegatedUserDecryptResult = UserDecryptResult &
  Readonly<{
    delegatorAddress: Hex;
    delegateAddress: Hex;
    delegation: DelegationStatus;
  }>;
