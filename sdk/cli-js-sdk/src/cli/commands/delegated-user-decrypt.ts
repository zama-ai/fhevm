import type { Command } from "@commander-js/extra-typings";

import {
  delegatedUserDecrypt,
  freshDelegatedUserDecrypt,
} from "../../flows";
import { FHE_VALUE_TYPES } from "../../types";
import { parseClearValue, serializeValue } from "../../values";
import { getGlobalOptions } from "../options";
import { printJson } from "../output";
import {
  collectHandle,
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
      `Delegated user decrypt flows. Supported types: ${supportedValueTypes}`,
    );

  delegatedCommand
    .command("cached")
    .description(
      "Delegated user decrypt an FHETest handle from delegator/type, or direct handles",
    )
    .option(
      "-t, --type <type>",
      `value type (${supportedValueTypes})`,
      parseValueType,
      "bool",
    )
    .option(
      "--contract <address>",
      "FHETest contract address override",
      parseAddress,
    )
    .option("--delegator <address>", "encrypted data owner", parseAddress)
    .option(
      "--handle <handle>",
      "encrypted handle to decrypt directly; repeat for multiple",
      collectHandle,
      [],
    )
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
      const globals = getGlobalOptions(command);
      const result = await delegatedUserDecrypt({
        network: globals.network,
        relayerUrl: globals.relayerUrl,
        rpcUrl: globals.rpcUrl,
        type: options.type,
        contractAddress: options.contract,
        delegatorAddress: options.delegator,
        handles: options.handle,
        durationDays: options.durationDays,
        delegationDurationDays: options.delegationDurationDays,
        privateKey: options.privateKey,
        mnemonic: options.mnemonic,
        delegatorPrivateKey: options.delegatorPrivateKey,
        delegatorMnemonic: options.delegatorMnemonic,
        onProgress: createProgressReporter(),
      });

      printJson(result);
    });

  delegatedCommand
    .command("fresh")
    .description(
      "Encrypt a new delegator value, store it in FHETest, then delegated user decrypt it",
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
        onProgress: createProgressReporter(),
      });

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
