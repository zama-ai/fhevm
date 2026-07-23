// seed — the `demo:seed` entrypoint (#1760). Brings a freshly-deployed demo stack to the state the
// dApp (#1761), the deposit-arc smoke and the rehearsal (#1762) expect, then writes the demo-config
// JSON that every consumer reads.
//
// STATUS: live-only, UNVERIFIED here. It cannot run without the two demo programs deployed (their
// keypairs are classifier-gated in this environment — see solana/scripts/demo/demo-keypairs/README)
// and a running local stack. It is exercised only by the `solana-demo-acceptance` workflow (manual
// dispatch), which deploys the programs and runs `demo-up.sh` (which calls this) before the smoke.
// The SDK is reached through the runtime dynamic-import seam (string module specifier) the rest of
// test-suite uses, because the SDK's generated `_types` are not built at `tsc` check time
// (see `src/solana/current-user-decrypt.ts`); that import is therefore untyped here by construction.
//
// Seeding sequence (writes nothing until every step below has produced a real on-chain address):
//   1. provision the vault topology — initialize the demo vault, create the four mints (mock USDC +
//      the share underlying, cUSDC + cShares confidential), initialize the two batchers.
//   2. openBatch on each batcher (SDK `openBatch`, EXPORTED from @fhevm/sdk/solana/vault): creates
//      the per-batch settle Address Lookup Table off-chain and returns its address.
//   3. fund the personas (keeper/alice/bob) with SOL for fees.
//   4. write the demo-config JSON (config.ts `writeDemoConfig`, which re-parses before persisting).
//
// PROVISIONING GAP (step 1): `initialize_vault`, the SPL/confidential mint creation, and
// `initialize_batcher` are NOT part of the stable `@fhevm/sdk/solana/vault` surface (that subpath
// exports the batch lifecycle — openBatch/joinBatch/dispatch/settle/claim/decrypt + derive/reads —
// not the one-time provisioning builders, which live under the SDK's generated internal modules).
// The confidential-token wrap has the same status: the existing two-holder scenario provisions the
// confidential mint + wrap through the Rust live-client (`poc-live-client`), documented there as an
// SDK gap. This seed leaves that provisioning boundary explicit rather than inventing calls whose
// behavior cannot be verified here; wiring it is the remaining live task, tracked in the PR report.

import {
  DEMO_CONFIG_DEFAULT_PATH,
  writeDemoConfig,
  type SolanaDemoConfig,
} from "./config";

/** The batch lifecycle actions the seed uses from the vault subpath (untyped: runtime dynamic import). */
const loadVaultModule = async (): Promise<Record<string, (...args: unknown[]) => unknown>> => {
  const vaultModule = "@fhevm/sdk/solana/vault";
  return (await import(vaultModule)) as Record<string, (...args: unknown[]) => unknown>;
};

/**
 * Provisions the vault topology and returns every root address the demo-config carries. This is the
 * live provisioning boundary (see the PROVISIONING GAP note above): until the vault/mint/batcher
 * initialization is wired (SDK builders or live-client), it cannot yield real addresses, so it fails
 * loudly rather than fabricating a config that would not correspond to on-chain state.
 */
const provisionTopology = async (): Promise<never> => {
  await loadVaultModule(); // proves the vault subpath resolves at runtime before we rely on it
  throw new Error(
    "demo:seed provisioning is not wired: initialize_vault + the four mints + initialize_batcher " +
      "must be executed (SDK generated builders or poc-live-client) before openBatch and before a " +
      "real demo-config can be written. This is the remaining live task (see PR report); it is " +
      "exercised end-to-end only by the solana-demo-acceptance workflow.",
  );
};

const main = async (): Promise<void> => {
  const configPath = process.env.DEMO_CONFIG_PATH ?? DEMO_CONFIG_DEFAULT_PATH;

  // Once provisioning is wired, `provisionTopology` returns the seeded roots and this assembles them
  // into the validated artifact. The shape below is the config.ts contract (roots only, everything
  // else derived by the SDK); `writeDemoConfig` re-parses before persisting, so a malformed assembly
  // fails at write with a named field rather than surfacing later inside an SDK call.
  const roots = (await provisionTopology()) as unknown as SolanaDemoConfig;
  await writeDemoConfig(roots, configPath);
  console.log(`demo-config written to ${configPath}`);
};

await main();
