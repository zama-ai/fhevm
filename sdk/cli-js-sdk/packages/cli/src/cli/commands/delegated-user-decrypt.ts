import type { Command } from "@commander-js/extra-typings";

import { FHE_VALUE_TYPES } from "@cli-fhevm-sdk/toolkit/types";
import { parseClearValue, serializeValue } from "@cli-fhevm-sdk/toolkit/values";
import { getGlobalOptions } from "../options";
import { printJson, writeJsonFile } from "../output";
import {
  collectHandle,
  collectValueType,
  parseAddress,
  parsePositiveInteger,
  parsePrivateKey,
  parseValueType,
} from "../parsers";
import { createProgressReporter } from "../progress";

const DEFAULT_PERMIT_DURATION_DAYS = 1;
const DEFAULT_DELEGATION_DURATION_DAYS = 360;

/**
 * Registers delegated decrypt commands.
 *
 * Delegate credentials come from `PRIVATE_KEY`/`MNEMONIC`; encrypted data owner
 * credentials come from `DELEGATOR_PRIVATE_KEY`/`DELEGATOR_MNEMONIC`.
 */
export const registerDelegatedUserDecryptCommands = (program: Command): void => {
  const supportedValueTypes = FHE_VALUE_TYPES.join(", ");
  const delegatedCommand = program
    .command("delegated-user-decrypt")
    .description(
      `Decrypt existing handles as a delegate, from any contract. Supported types: ${supportedValueTypes}`,
    )
    .option(
      "--handle <handle>",
      "encrypted handle to decrypt directly; repeat for multiple",
      collectHandle,
      [],
    )
    .option(
      "--contract <address>",
      "contract address paired with the handles for ACL verification; defaults to the FHETest contract",
      parseAddress,
    )
    .option("--delegator <address>", "encrypted data owner", parseAddress)
    .option(
      "--duration-days <days>",
      "decryption permit duration in days",
      parsePositiveInteger,
      DEFAULT_PERMIT_DURATION_DAYS,
    )
    .option(
      "--delegation-duration-days <days>",
      "ACL delegation duration in days when creating delegation",
      parsePositiveInteger,
      DEFAULT_DELEGATION_DURATION_DAYS,
    )
    .option(
      "--artifact <path>",
      "write a sensitive user-decrypt validation artifact",
    )
    .option(
      "--private-key <privateKey>",
      "delegate private key; falls back to PRIVATE_KEY",
      parsePrivateKey,
    )
    .option("--mnemonic <mnemonic>", "delegate mnemonic; falls back to MNEMONIC")
    .option(
      "--delegator-private-key <privateKey>",
      "delegator private key; falls back to DELEGATOR_PRIVATE_KEY",
      parsePrivateKey,
    )
    .option(
      "--delegator-mnemonic <mnemonic>",
      "delegator mnemonic; falls back to DELEGATOR_MNEMONIC",
    )
    .action(async (options, command) => {
      if (options.handle.length === 0) {
        command.help();
        return;
      }
      const { delegatedUserDecrypt } = await import(
        "@cli-fhevm-sdk/toolkit/flows/delegated-user-decrypt/direct"
      );
      const globals = getGlobalOptions(command);
      const result = await delegatedUserDecrypt({
        network: globals.network,
        relayerUrl: globals.relayerUrl,
        rpcUrl: globals.rpcUrl,
        contractAddress: options.contract,
        delegatorAddress: options.delegator,
        handles: options.handle,
        durationDays: options.durationDays,
        delegationDurationDays: options.delegationDurationDays,
        privateKey: options.privateKey,
        mnemonic: options.mnemonic,
        delegatorPrivateKey: options.delegatorPrivateKey,
        delegatorMnemonic: options.delegatorMnemonic,
        includeValidationArtifact: options.artifact !== undefined,
        onProgress: createProgressReporter(),
      });

      if (options.artifact !== undefined) {
        await writeJsonFile(options.artifact, result.validationArtifact);
      }
      const { validationArtifact: _validationArtifact, ...publicResult } = result;
      printJson(publicResult);
    });

  delegatedCommand
    .command("stored")
    .description(
      "Demo: delegated user decrypt FHETest handles stored in the delegator's type slots",
    )
    .option(
      "-t, --type <type>",
      `stored value type to read; repeat for multiple (${supportedValueTypes})`,
      collectValueType,
    )
    .option(
      "--contract <address>",
      "FHETest contract address override",
      parseAddress,
    )
    .option("--delegator <address>", "encrypted data owner", parseAddress)
    .option(
      "--duration-days <days>",
      "decryption permit duration in days",
      parsePositiveInteger,
      DEFAULT_PERMIT_DURATION_DAYS,
    )
    .option(
      "--delegation-duration-days <days>",
      "ACL delegation duration in days when creating delegation",
      parsePositiveInteger,
      DEFAULT_DELEGATION_DURATION_DAYS,
    )
    .option(
      "--artifact <path>",
      "write a sensitive user-decrypt validation artifact",
    )
    .option(
      "--private-key <privateKey>",
      "delegate private key; falls back to PRIVATE_KEY",
      parsePrivateKey,
    )
    .option("--mnemonic <mnemonic>", "delegate mnemonic; falls back to MNEMONIC")
    .option(
      "--delegator-private-key <privateKey>",
      "delegator private key; falls back to DELEGATOR_PRIVATE_KEY",
      parsePrivateKey,
    )
    .option(
      "--delegator-mnemonic <mnemonic>",
      "delegator mnemonic; falls back to DELEGATOR_MNEMONIC",
    )
    .action(async (options, command) => {
      const { storedDelegatedUserDecrypt } = await import(
        "@cli-fhevm-sdk/toolkit/flows/delegated-user-decrypt/stored"
      );
      const globals = getGlobalOptions(command);
      const result = await storedDelegatedUserDecrypt({
        network: globals.network,
        relayerUrl: globals.relayerUrl,
        rpcUrl: globals.rpcUrl,
        types: options.type,
        contractAddress: options.contract,
        delegatorAddress: options.delegator,
        durationDays: options.durationDays,
        delegationDurationDays: options.delegationDurationDays,
        privateKey: options.privateKey,
        mnemonic: options.mnemonic,
        delegatorPrivateKey: options.delegatorPrivateKey,
        delegatorMnemonic: options.delegatorMnemonic,
        includeValidationArtifact: options.artifact !== undefined,
        onProgress: createProgressReporter(),
      });

      if (options.artifact !== undefined) {
        await writeJsonFile(options.artifact, result.validationArtifact);
      }
      const { validationArtifact: _validationArtifact, ...publicResult } = result;
      printJson(publicResult);
    });

  delegatedCommand
    .command("fresh")
    .description(
      "Demo: encrypt a new delegator value, store it in FHETest, then delegated user decrypt it",
    )
    .option(
      "-t, --type <type>",
      `value type to encrypt (${supportedValueTypes})`,
      parseValueType,
      "bool",
    )
    .option("--value <value>", "clear value to encrypt; defaults to random")
    .option(
      "--contract <address>",
      "FHETest contract address override",
      parseAddress,
    )
    .option("--delegator <address>", "encrypted data owner", parseAddress)
    .option(
      "--duration-days <days>",
      "decryption permit duration in days",
      parsePositiveInteger,
      DEFAULT_PERMIT_DURATION_DAYS,
    )
    .option(
      "--delegation-duration-days <days>",
      "ACL delegation duration in days when creating delegation",
      parsePositiveInteger,
      DEFAULT_DELEGATION_DURATION_DAYS,
    )
    .option(
      "--artifact <path>",
      "write a sensitive user-decrypt validation artifact",
    )
    .option(
      "--private-key <privateKey>",
      "delegate private key; falls back to PRIVATE_KEY",
      parsePrivateKey,
    )
    .option("--mnemonic <mnemonic>", "delegate mnemonic; falls back to MNEMONIC")
    .option(
      "--delegator-private-key <privateKey>",
      "delegator private key; falls back to DELEGATOR_PRIVATE_KEY",
      parsePrivateKey,
    )
    .option(
      "--delegator-mnemonic <mnemonic>",
      "delegator mnemonic; falls back to DELEGATOR_MNEMONIC",
    )
    .action(async (options, command) => {
      const { freshDelegatedUserDecrypt } = await import(
        "@cli-fhevm-sdk/toolkit/flows/delegated-user-decrypt/fresh"
      );
      const globals = getGlobalOptions(command);
      const value =
        options.value === undefined
          ? undefined
          : parseClearValue(options.type, options.value);
      const result = await freshDelegatedUserDecrypt({
        network: globals.network,
        relayerUrl: globals.relayerUrl,
        rpcUrl: globals.rpcUrl,
        type: options.type,
        value,
        contractAddress: options.contract,
        delegatorAddress: options.delegator,
        durationDays: options.durationDays,
        delegationDurationDays: options.delegationDurationDays,
        privateKey: options.privateKey,
        mnemonic: options.mnemonic,
        delegatorPrivateKey: options.delegatorPrivateKey,
        delegatorMnemonic: options.delegatorMnemonic,
        includeValidationArtifact: options.artifact !== undefined,
        onProgress: createProgressReporter(),
      });

      if (options.artifact !== undefined) {
        await writeJsonFile(options.artifact, result.validationArtifact);
      }
      printJson({
        transactionHash: result.transactionHash,
        inputValues: result.inputValues.map(serializeValue),
        inputProof: result.inputProof,
        handle: result.handle,
        contractAddress: result.contractAddress,
        ownerAddress: result.ownerAddress,
        signerAddress: result.signerAddress,
        isDelegated: result.isDelegated,
        delegatorAddress: result.delegatorAddress,
        delegateAddress: result.delegateAddress,
        delegation: result.delegation,
        encryptedValues: result.encryptedValues,
        clearValues: result.clearValues,
        permit: result.permit,
      });
    });
};
