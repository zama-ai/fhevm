// loadDemoEnv — the demo-config JSON as the harness's second `loadEnv` source (#1760).
//
// It reads the seeded runtime artifact and projects it onto the harness `TestEnv` via
// `resolveEnv(overrides, "demo-config")`. The endpoint/id fields map onto `TestEnvOverrides`; the
// vault-specific roots stay in the returned `SolanaDemoConfig` (the harness carries no protocol
// knowledge, so vault roots never enter `TestEnv`). Scenarios read both: `env` for endpoints +
// funding, `config` for the vault roots they feed to the SDK derivation helpers.

import path from "node:path";

import { resolveEnv, type TestEnv } from "../e2e/harness/loadEnv";
import { DEMO_CONFIG_DEFAULT_PATH, readDemoConfig, type SolanaDemoConfig } from "./config";

/** Repo root, resolved from this file's location (test-suite/fhevm/demo → repo root). */
const REPO_ROOT = path.resolve(import.meta.dir, "../../..");

/**
 * Committed demo keypairs (low-value, local-only; see `solana/scripts/e2e/test-keypairs/README.md`
 * for the committed-key policy). Personas sign from these files; the demo-config JSON carries only
 * their pubkeys, so a scenario can cross-check the loaded key against the published address.
 */
export const DEMO_KEYPAIRS = {
  keeper: path.join(REPO_ROOT, "solana/scripts/demo/demo-keypairs/keeper.json"),
  alice: path.join(REPO_ROOT, "solana/scripts/demo/demo-keypairs/alice.json"),
  bob: path.join(REPO_ROOT, "solana/scripts/demo/demo-keypairs/bob.json"),
  mintAuthority: path.join(REPO_ROOT, "solana/scripts/demo/demo-keypairs/mint-authority.json"),
} as const;

/** Maps the demo-config onto the harness's `TestEnvOverrides` (endpoint + identity fields only). */
const toOverrides = (config: SolanaDemoConfig) => ({
  rpcUrl: config.rpcUrl,
  wsUrl: config.wsUrl,
  relayerUrl: config.relayerUrl,
  proofServiceUrl: config.proofServiceUrl,
  gatewayRpcUrl: config.gatewayRpcUrl,
  chainId: config.chainId,
  aclProgram: config.aclProgram,
  userDecryptContextId: config.userDecryptContextId,
});

/** Loads the demo runtime: the harness `TestEnv` (source "demo-config") plus the full vault config. */
export const loadDemoEnv = async (
  configPath = DEMO_CONFIG_DEFAULT_PATH,
): Promise<{ env: TestEnv; config: SolanaDemoConfig }> => {
  const config = await readDemoConfig(configPath);
  const env = resolveEnv(toOverrides(config), "demo-config");
  return { env, config };
};
