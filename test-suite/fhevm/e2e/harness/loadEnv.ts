// loadEnv — builds the TestEnv that the Solana e2e scenarios run against.
//
// Zero protocol knowledge: it only assembles endpoints, chain identifiers, on-disk roots, and
// capability flags. It never encodes/decodes protocol bytes — scenarios reach the protocol solely
// through `@fhevm/sdk` Solana actions.
//
// Source for NOW: the local clean-e2e stack. Every value below is exactly what the current e2e
// runtime provides, traced to where it lands:
//   - urls/ids: the clean-e2e bring-up (validator RPC/WS, relayer, the
//     RFC-021 host chain id, ACL program, KMS context ids) and
//     `test-suite/fhevm/src/solana/two-holder-transfer.ts` (RPC/WS/relayer/ACL constants).
//   - coprocessor DB container: `test-suite/fhevm/src/layout.ts` (COPROCESSOR_DB_CONTAINER).
//   - deployer keypair: `~/.config/solana/id.json`, the wallet the side-stack setup deploys with.
//   - gateway addresses (optional, for future phases): generated at `fhevm-cli up` time into
//     `.fhevm/runtime/addresses/gateway/.env.gateway`.
// Env vars override any field so a run can point at a non-default local stack.
//
// Source "devnet": the same programs deployed on Solana devnet behind a preview-env namespace.
// Selected with `SOLANA_E2E_SOURCE=devnet`. No airdrop exists there, so scenarios fund fresh
// wallets by transfer from the deployer wallet, with the small amounts in `FUNDING_BY_SOURCE`
// (and sweep them back at the end, see `wallets.ts`), and
// the SNS probe reaches the coprocessor Postgres through whatever command `COPROCESSOR_DB_PSQL`
// names (a `kubectl exec ... psql` prefix) instead of the local `docker exec`.
//
// A second source (the confidential-vault demo-config JSON, #1760) plugs in here: it reads the
// runtime artifact and calls `resolveEnv(overrides, "demo-config")` with a `Partial<TestEnvOverrides>`
// mapped from the file (see `demo/loadDemoEnv.ts`). The mapping lives with the demo package, not
// here, so this module keeps its zero-protocol-knowledge stance — it only learns a new source label.

import os from "node:os";
import path from "node:path";

import { SOLANA_LEAF_PROOF_API_KEY } from "../../src/generate/solana";
import { coprocessorDbPsql, SOLANA_ACL_PROGRAM } from "../../src/layout";
import { LOCAL_SOLANA_ENDPOINTS } from "../../src/solana/endpoints";

export type Capabilities = {
  /** Can fund actors with SOL (local validator airdrop). Local: true. Devnet/mainnet: false. */
  readonly faucet: boolean;
  /** Can create brand-new SPL / confidential mints for a scenario. Local & devnet: true. */
  readonly freshMints: boolean;
  /** Slots advance on demand (local validator). Live networks: false. */
  readonly fastSlots: boolean;
};

export type TestEnv = {
  /** Which runtime the env was assembled from. */
  readonly source: TestEnvSource;
  /** The Solana cluster the programs run on; decides funding and slot behavior. */
  readonly network: SolanaNetwork;
  readonly rpcUrl: string;
  readonly wsUrl: string;
  readonly relayerUrl: string;
  readonly gatewayRpcUrl: string;
  /** Primary EVM host chain RPC — where the deployed `ProtocolConfig` declares the active KMS pair. */
  readonly hostRpcUrl: string;
  /** RFC-021 Solana host chain id (72057594037940281); type byte `0x01` marks it a Solana chain. */
  readonly chainId: bigint;
  /** zama-host program id as a bytes32 hex — the Solana ACL identity. */
  readonly aclProgram: `0x${string}`;
  /**
   * KMS/gateway user-decrypt context id override, as an unsigned decimal string. There is no
   * static default: when absent, helpers read the active pair from the deployed `ProtocolConfig`
   * (`readActiveKmsPair`) so the permit names a pair the Connector actually serves.
   */
  readonly userDecryptContextId: string | undefined;
  /** Command prefix that runs `psql` against the coprocessor DB (for ciphertext-materialization waits). */
  readonly coprocessorDbPsql: readonly string[];
  /** The first coprocessor's leaf-proof endpoint (`/v1/solana/leaf-proofs`) and its bearer token. */
  readonly leafProof: LeafProofEndpoint;
  readonly roots: { readonly deployerKeypairPath: string };
  readonly capabilities: Capabilities;
  readonly funding: Funding;
};

export type LeafProofEndpoint = { readonly url: string; readonly apiKey: string };

/** "local" and "devnet" assemble from process env and defaults; "demo-config" from a seed's artifact. */
export type TestEnvSource = "local" | "demo-config" | "devnet";
export type SolanaNetwork = "localnet" | "devnet";

/**
 * SOL a scenario gives each actor it creates. Local airdrops are free, so the amounts are generous;
 * on devnet they come out of the deployer wallet by transfer and cover rent plus fees with margin.
 */
export type Funding = {
  /** The actor that pays provisioning rent (mints, escrow, token accounts) and the arc's fees. */
  readonly primarySol: number;
  /** An actor that pays only its own token account and a few fees. */
  readonly secondarySol: number;
};

type TestEnvOverrides = {
  rpcUrl: string;
  wsUrl: string;
  relayerUrl: string;
  gatewayRpcUrl: string;
  hostRpcUrl: string;
  chainId: string;
  aclProgram: string;
  userDecryptContextId: string;
  coprocessorDbPsql: readonly string[];
  leafProofUrl: string;
  leafProofApiKey: string;
  deployerKeypairPath: string;
};

// The local clean-e2e stack. Endpoints come from the one local definition (`src/solana/endpoints.ts`);
// the protocol identities (ACL program, coprocessor DB container) from the CLI's config module, so
// there is exactly one source of truth shared with the transfer orchestrator.
const LOCAL_DEFAULTS = {
  rpcUrl: LOCAL_SOLANA_ENDPOINTS.validatorRpc,
  wsUrl: LOCAL_SOLANA_ENDPOINTS.validatorWs,
  relayerUrl: LOCAL_SOLANA_ENDPOINTS.relayer,
  gatewayRpcUrl: LOCAL_SOLANA_ENDPOINTS.gatewayRpc,
  hostRpcUrl: LOCAL_SOLANA_ENDPOINTS.hostRpc,
  chainId: "72057594037940281",
  aclProgram: SOLANA_ACL_PROGRAM,
  coprocessorDbPsql: coprocessorDbPsql(),
  leafProofUrl: LOCAL_SOLANA_ENDPOINTS.leafProof,
  leafProofApiKey: SOLANA_LEAF_PROOF_API_KEY,
} as const;

// A local validator airdrops and advances slots on demand; devnet does neither. The demo-config
// source differs from the bare sources solely in provenance: mints/batchers are pre-seeded, not
// created per scenario — the demo smoke reuses the seeded mints rather than minting fresh ones.
const capabilitiesFor = (source: TestEnvSource, network: SolanaNetwork): Capabilities => ({
  faucet: network === "localnet",
  freshMints: source !== "demo-config",
  fastSlots: network === "localnet",
});

const FUNDING_BY_NETWORK: Record<SolanaNetwork, Funding> = {
  localnet: { primarySol: 10, secondarySol: 5 },
  devnet: { primarySol: 0.5, secondarySol: 0.2 },
};

const bytes32Hex = (value: string): `0x${string}` => {
  if (!/^0x[0-9a-f]{64}$/i.test(value)) throw new Error(`expected a 0x-prefixed 32-byte hex value, got ${value}`);
  return value as `0x${string}`;
};

const solanaChainId = (value: string): bigint => {
  if (!/^\d+$/.test(value)) throw new Error(`chainId must be an unsigned decimal integer, got ${value}`);
  const id = BigInt(value);
  if (((id >> 56n) & 0xffn) !== 0x01n) {
    throw new Error(`chainId ${value} is not a Solana type-byte chain id`);
  }
  return id;
};

const decimalString = (value: string, name: string): string => {
  if (!/^\d+$/.test(value)) throw new Error(`${name} must be an unsigned decimal integer, got ${value}`);
  return value;
};

/** Reads TestEnv overrides from the process environment (the "now" source). */
export const envOverrides = (env: NodeJS.ProcessEnv): Partial<TestEnvOverrides> => {
  const pick = <K extends keyof TestEnvOverrides>(key: K, name: string): Partial<Pick<TestEnvOverrides, K>> => {
    const value = env[name];
    return value === undefined || value === "" ? {} : ({ [key]: value } as Pick<TestEnvOverrides, K>);
  };
  return {
    ...pick("rpcUrl", "SOLANA_RPC_URL"),
    ...pick("wsUrl", "SOLANA_WS_URL"),
    ...pick("relayerUrl", "SOLANA_RELAYER_URL"),
    ...pick("gatewayRpcUrl", "GW_RPC"),
    ...pick("hostRpcUrl", "HOST_RPC"),
    ...pick("chainId", "SOLANA_HOST_CHAIN_ID"),
    ...pick("aclProgram", "SOLANA_ACL_PROGRAM"),
    ...pick("userDecryptContextId", "SOLANA_UD_CONTEXT_ID"),
    ...psqlOverride(env),
    ...pick("leafProofUrl", "SOLANA_LEAF_PROOF_URL"),
    ...pick("leafProofApiKey", "SOLANA_LEAF_PROOF_API_KEY"),
    ...pick("deployerKeypairPath", "SOLANA_DEPLOYER_KEYPAIR"),
  };
};

// `COPROCESSOR_DB_PSQL` is a whole command prefix, split on whitespace (its arguments are pod and
// role names, never paths with spaces); `COPROCESSOR_DB_CONTAINER` keeps naming a local container.
const psqlOverride = (env: NodeJS.ProcessEnv): Partial<Pick<TestEnvOverrides, "coprocessorDbPsql">> => {
  if (env.COPROCESSOR_DB_PSQL) return { coprocessorDbPsql: env.COPROCESSOR_DB_PSQL.trim().split(/\s+/) };
  if (env.COPROCESSOR_DB_CONTAINER) return { coprocessorDbPsql: coprocessorDbPsql(env.COPROCESSOR_DB_CONTAINER) };
  return {};
};

const sourceFromEnv = (env: NodeJS.ProcessEnv): "local" | "devnet" => {
  const value = env.SOLANA_E2E_SOURCE ?? "local";
  if (value !== "local" && value !== "devnet") {
    throw new Error(`SOLANA_E2E_SOURCE must be "local" or "devnet", got ${value}`);
  }
  return value;
};

/** Assembles a validated TestEnv from `defaults <- overrides`. Exported for the scenario tests. */
export const resolveEnv = (
  overrides: Partial<TestEnvOverrides> = {},
  source: TestEnvSource = "local",
  network: SolanaNetwork = source === "devnet" ? "devnet" : "localnet",
): TestEnv => {
  const merged = { ...LOCAL_DEFAULTS, ...overrides };
  const deployerKeypairPath =
    overrides.deployerKeypairPath ?? path.join(os.homedir(), ".config/solana/id.json");
  if (source === "devnet" && network !== "devnet") throw new Error('source "devnet" runs on network "devnet"');
  return {
    source,
    network,
    rpcUrl: merged.rpcUrl,
    wsUrl: merged.wsUrl,
    relayerUrl: merged.relayerUrl,
    gatewayRpcUrl: merged.gatewayRpcUrl,
    hostRpcUrl: merged.hostRpcUrl,
    chainId: solanaChainId(merged.chainId),
    aclProgram: bytes32Hex(merged.aclProgram),
    userDecryptContextId:
      merged.userDecryptContextId === undefined
        ? undefined
        : decimalString(merged.userDecryptContextId, "userDecryptContextId"),
    coprocessorDbPsql: merged.coprocessorDbPsql,
    leafProof: { url: merged.leafProofUrl, apiKey: merged.leafProofApiKey },
    roots: { deployerKeypairPath },
    capabilities: capabilitiesFor(source, network),
    funding: FUNDING_BY_NETWORK[network],
  };
};

/** Builds the TestEnv the scenarios run against, from the current e2e runtime. */
export const loadEnv = (env: NodeJS.ProcessEnv = process.env): TestEnv =>
  resolveEnv(envOverrides(env), sourceFromEnv(env));
