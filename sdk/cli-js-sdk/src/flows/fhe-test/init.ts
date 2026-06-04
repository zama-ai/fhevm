import type { Hex } from "viem";

import { createWalletContext, type ClientOptions } from "../../config";
import { hasFheTestHandle, readFheTestHandle } from "../../fhe-test/handles";
import {
  getSetClearFunctionName,
  simulateSetClearValue,
} from "../../fhe-test/writes";
import type { ProgressReporter } from "../../shared/progress";
import { sendAndWait } from "../../shared/transactions";
import type { FheTestHandle, FheValueType } from "../../types";
import { FHE_VALUE_TYPES } from "../../types";
import { createInitValue } from "../../values";

export type InitFheTestOptions = ClientOptions &
  Readonly<{
    contractAddress?: Hex;
    type?: FheValueType;
    force?: boolean;
    privateKey?: Hex;
    mnemonic?: string;
    onProgress?: ProgressReporter;
  }>;

export const initFheTest = async (
  options: InitFheTestOptions,
): Promise<{
  contractAddress: Hex;
  account: Hex;
  initialized: readonly FheTestHandle[];
  skipped: readonly FheTestHandle[];
}> => {
  options.onProgress?.("Loading wallet and creating clients");
  const { account, contractAddress, publicClient, walletClient } =
    createWalletContext(options);
  const types = options.type ? [options.type] : FHE_VALUE_TYPES;
  const initialized: FheTestHandle[] = [];
  const skipped: FheTestHandle[] = [];

  for (const valueType of types) {
    options.onProgress?.(`Checking existing ${valueType} handle`);
    const hasHandle = await hasFheTestHandle({
      publicClient,
      contractAddress,
      account: account.address,
      type: valueType,
    });

    if (hasHandle && !options.force) {
      skipped.push(
        await readFheTestHandle({
          publicClient,
          contractAddress,
          account: account.address,
          type: valueType,
          onProgress: options.onProgress,
        }),
      );
      continue;
    }

    const value = createInitValue(valueType);
    options.onProgress?.(
      `Simulating FHETest.${getSetClearFunctionName(valueType)}`,
    );
    const request = await simulateSetClearValue(
      { account, contractAddress, publicClient },
      { value, makePublic: true },
    );

    await sendAndWait({
      walletClient,
      publicClient,
      request,
      onProgress: options.onProgress,
    });
    initialized.push(
      await readFheTestHandle({
        publicClient,
        contractAddress,
        account: account.address,
        type: valueType,
        onProgress: options.onProgress,
      }),
    );
  }

  return {
    contractAddress,
    account: account.address,
    initialized,
    skipped,
  };
};
