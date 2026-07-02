import type { Command } from "@commander-js/extra-typings";

import { getGlobalOptions } from "../options";
import { printJson } from "../output";
import { parseAddress, parsePrivateKey, parseTokenAmount } from "../parsers";
import { createProgressReporter } from "../progress";

/** Registers ERC-7984 confidential token commands for transfer and balance. */
export const registerTokenCommands = (program: Command): void => {
  const tokenCommand = program
    .command("token")
    .description("ERC-7984 confidential token utilities");

  tokenCommand
    .command("transfer")
    .description(
      "Confidential ERC-7984 transfer; amount is base units encrypted as euint64",
    )
    .requiredOption(
      "--contract <address>",
      "confidential token contract address",
      parseAddress,
    )
    .requiredOption("--to <address>", "recipient address", parseAddress)
    .requiredOption(
      "--amount <amount>",
      "amount in base units (0 < amount < 2^64)",
      parseTokenAmount,
    )
    .option(
      "--from <address>",
      "operator transferFrom source; spends an existing allowance",
      parseAddress,
    )
    .option(
      "--private-key <privateKey>",
      "wallet private key; falls back to PRIVATE_KEY",
      parsePrivateKey,
    )
    .option("--mnemonic <mnemonic>", "wallet mnemonic; falls back to MNEMONIC")
    .action(async (options, command) => {
      const { transferToken } = await import("../../flows/token/transfer");
      const globals = getGlobalOptions(command);
      const result = await transferToken({
        network: globals.network,
        relayerUrl: globals.relayerUrl,
        rpcUrl: globals.rpcUrl,
        contractAddress: options.contract,
        to: options.to,
        amount: options.amount,
        from: options.from,
        privateKey: options.privateKey,
        mnemonic: options.mnemonic,
        onProgress: createProgressReporter(),
      });

      printJson(result);
    });

  tokenCommand
    .command("balance")
    .description("Read the confidential ERC-7984 balance handle for an account")
    .requiredOption(
      "--contract <address>",
      "confidential token contract address",
      parseAddress,
    )
    .option(
      "--account <address>",
      "account to read; defaults to wallet address",
      parseAddress,
    )
    .option(
      "--private-key <privateKey>",
      "wallet private key for default account; falls back to PRIVATE_KEY",
      parsePrivateKey,
    )
    .option(
      "--mnemonic <mnemonic>",
      "wallet mnemonic for default account; falls back to MNEMONIC",
    )
    .action(async (options, command) => {
      const { balanceOfToken } = await import("../../flows/token/balance");
      const globals = getGlobalOptions(command);
      const result = await balanceOfToken({
        network: globals.network,
        relayerUrl: globals.relayerUrl,
        rpcUrl: globals.rpcUrl,
        contractAddress: options.contract,
        account: options.account,
        privateKey: options.privateKey,
        mnemonic: options.mnemonic,
        onProgress: createProgressReporter(),
      });

      printJson(result);
    });
};
