// Scenario: a transaction that its estimate simulation accepts but the chain rejects must reject
// the send. The Kit client skips the preflight simulation (the estimate already ran one), so only
// the confirmation can report this failure.

import { describe, expect, test } from "bun:test";
import { address, isSolanaError, SOLANA_ERROR__INSTRUCTION_ERROR__CUSTOM } from "@solana/kit";
import { getTransferSolInstruction } from "@solana-program/system";

import { loadEnv, loadPersonas, openRunWallets } from "../harness";
import { openProvisioning } from "../harness/solana/provisioning";

// Each transfer is above the rent-exempt minimum, and two of them exceed the funding.
const FUNDING_SOL = 0.01;
const TRANSFER_LAMPORTS = 6_000_000n;
const SCENARIO_TIMEOUT_MS = 3 * 60_000;

describe("solana transaction failure scenario", () => {
  test(
    "a transaction that fails on chain rejects the send",
    async () => {
      const env = loadEnv();
      const deployer = address((await loadPersonas(env)).deployer.address);
      const provisioning = await openProvisioning(env);
      const wallets = openRunWallets(env, provisioning);
      const { signer: payer } = await wallets.fresh(FUNDING_SOL);
      const client = await provisioning.client(payer);
      // Different amounts keep the two transactions distinct when they share a blockhash.
      const transfer = (amount: bigint) => getTransferSolInstruction({ source: payer, destination: deployer, amount });

      // Signed while the balance still covers it, so its estimate passes; the first send then leaves
      // too little for it, and the System program rejects it on chain.
      const doomed = await client.signTransaction([transfer(TRANSFER_LAMPORTS)]);
      await client.sendTransaction([transfer(TRANSFER_LAMPORTS + 1n)]);
      const error = await client.sendSignedTransaction(doomed.context.transaction).catch((caught: unknown) => caught);
      expect(isSolanaError(error, SOLANA_ERROR__INSTRUCTION_ERROR__CUSTOM)).toBe(true);

      await wallets.sweep();
    },
    SCENARIO_TIMEOUT_MS,
  );
});
