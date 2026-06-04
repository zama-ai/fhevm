import type { Hex } from "viem";

import { createWalletContext, type ClientOptions } from "../../config";
import { hasFheTestHandle, readFheTestHandle } from "../../fhe-test/handles";
import {
  getSetClearFunctionName,
  simulateInitFheTest,
  simulateSetClearValue,
} from "../../fhe-test/writes";
import type { ProgressReporter } from "../../shared/progress";
import { sendAndWait } from "../../shared/transactions";
import type { FheTestHandle, FheValueType } from "../../types";
import { FHE_VALUE_TYPES } from "../../types";
import { createInitValue } from "../../values";

/**
 * Options for initializing FHETest handles for the wallet account.
 *
 * `bulk` uses FHETest's all-types initializer and is mutually exclusive with
 * `type`; without `bulk`, each type is initialized through its clear setter.
 */
export type InitFheTestOptions = ClientOptions &
  Readonly<{
    contractAddress?: Hex;
    bulk?: boolean;
    type?: FheValueType;
    force?: boolean;
    privateKey?: Hex;
    mnemonic?: string;
    onProgress?: ProgressReporter;
  }>;

/** Ensures FHETest has stored handles for one or more types. */
export const initFheTest = async (
  options: InitFheTestOptions,
): Promise<{
  contractAddress: Hex;
  account: Hex;
  initialized: readonly FheTestHandle[];
  skipped: readonly FheTestHandle[];
  transactionHash?: Hex;
}> => {
  options.onProgress?.("Loading wallet and creating clients");
  const { account, contractAddress, publicClient, walletClient } =
    createWalletContext(options);
  if (options.bulk && options.type) {
    throw new Error("fhe-test init --bulk cannot be used with --type.");
  }

  const types = options.type ? [options.type] : FHE_VALUE_TYPES;
  const initialized: FheTestHandle[] = [];
  const skipped: FheTestHandle[] = [];

  if (options.bulk) {
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
      }
    }

    const typesToInitialize = options.force
      ? types
      : types.filter(
          (valueType) =>
            !skipped.some((handle) => handle.type === valueType),
        );

    if (typesToInitialize.length === 0) {
      return {
        contractAddress,
        account: account.address,
        initialized,
        skipped,
      };
    }

    options.onProgress?.("Simulating FHETest.initFheTest");
    const request = await simulateInitFheTest(
      { account, contractAddress, publicClient },
      options.force ?? false,
    );
    const transactionHash = await sendAndWait({
      walletClient,
      publicClient,
      request,
      onProgress: options.onProgress,
    });

    for (const valueType of typesToInitialize) {
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
      transactionHash,
    };
  }

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
