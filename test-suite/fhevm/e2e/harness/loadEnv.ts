// loadEnv — builds the TestEnv that the Solana e2e scenarios run against.
//
// Zero protocol knowledge: it only assembles endpoints, chain identifiers, on-disk roots, and
// capability flags. It never encodes/decodes protocol bytes — scenarios reach the protocol solely
// through `@fhevm/sdk` Solana actions.
//
// Source for NOW: the local clean-e2e stack. Every value below is exactly what the current e2e
// runtime provides, traced to where it lands:
//   - urls/ids: the clean-e2e bring-up (validator RPC/WS, relayer, the
//     RFC-021 host chain id, ACL program, KMS context ids), as named by
//     `src/solana/endpoints.ts` (LOCAL_SOLANA_ENDPOINTS) and `src/layout.ts` (SOLANA_ACL_PROGRAM).
//   - coprocessor DB and Merkle record containers: `test-suite/fhevm/src/layout.ts`
//     (COPROCESSOR_DB_CONTAINER, SOLANA_MERKLE_DB_CONTAINER).
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
// names (a `kubectl exec ... psql` prefix) instead of the local `docker exec`, and the Merkle record
// check reaches the record through `MERKLE_DB_PSQL`.
//
// Source "cleartext": the cleartext stack (`src/solana/cleartext-stack.ts`), a local validator
// whose zama-host keeps every plaintext in its accounts. Selected with `SOLANA_E2E_SOURCE=cleartext`.
// No relayer, gateway, coprocessor or KMS serves it (`protocolServices: false`), so the fields
// naming those services are unused there.
//
// A second source (the confidential-vault demo-config JSON, #1760) plugs in here: it reads the
// runtime artifact and calls `resolveEnv(overrides, "demo-config")` with a `Partial<TestEnvOverrides>`
// mapped from the file (see `demo/loadDemoEnv.ts`). The mapping lives with the demo package, not
// here, so this module keeps its zero-protocol-knowledge stance — it only learns a new source label.

import os from "node:os";
import path from "node:path";

import { isSolanaHostChainId } from "../../../../sdk/js-sdk/src/core/chains/hostChainId";
import {
  coprocessorDbPsql,
  SOLANA_ACL_PROGRAM,
  SOLANA_HOST_CHAIN_ID,
  solanaCleartextDeployerPath,
  solanaMerkleDbPsql,
} from "../../src/layout";
import { CLEARTEXT_SOLANA_ENDPOINTS, LOCAL_SOLANA_ENDPOINTS } from "../../src/solana/endpoints";
import { solanaE2eSource } from "../../src/solana/target";

export type Capabilities = {
  /** Can fund actors with SOL (local validator airdrop). Local: true. Devnet/mainnet: false. */
  readonly faucet: boolean;
  /** Can create brand-new SPL / confidential mints for a scenario. Local & devnet: true. */
  readonly freshMints: boolean;
  /** Slots advance on demand (local validator). Live networks: false. */
  readonly fastSlots: boolean;
  /** A relayer, coprocessors and KMS serve the chain. Cleartext: false. */
  readonly protocolServices: boolean;
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
  /** Command prefix that runs `psql` against the coprocessor DB (for ciphertext-materialization waits). */
  readonly coprocessorDbPsql: readonly string[];
  /** Command prefix that runs `psql` against the Merkle proof service's database. */
  readonly merkleDbPsql: readonly string[];
  readonly roots: { readonly deployerKeypairPath: string };
  readonly capabilities: Capabilities;
  readonly funding: Funding;
};

/** "local", "devnet" and "cleartext" assemble from process env and defaults; "demo-config" from a seed's artifact. */
export type TestEnvSource = "local" | "demo-config" | "devnet" | "cleartext";
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
  coprocessorDbPsql: readonly string[];
  merkleDbPsql: readonly string[];
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
  chainId: SOLANA_HOST_CHAIN_ID.toString(),
  aclProgram: SOLANA_ACL_PROGRAM,
  coprocessorDbPsql: coprocessorDbPsql(),
  merkleDbPsql: solanaMerkleDbPsql(),
} as const;

// A local validator airdrops and advances slots on demand; devnet does neither. The demo-config
// source differs from the bare sources solely in provenance: mints/batchers are pre-seeded, not
// created per scenario — the demo smoke reuses the seeded mints rather than minting fresh ones.
const capabilitiesFor = (source: TestEnvSource, network: SolanaNetwork): Capabilities => ({
  faucet: network === "localnet",
  freshMints: source !== "demo-config",
  fastSlots: network === "localnet",
  protocolServices: source !== "cleartext",
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
  if (!isSolanaHostChainId(id)) {
    throw new Error(`chainId ${value} is not a Solana type-byte chain id`);
  }
  return id;
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
    ...psqlOverride(env),
    ...pick("deployerKeypairPath", "SOLANA_DEPLOYER_KEYPAIR"),
  };
};

// `COPROCESSOR_DB_PSQL` and `MERKLE_DB_PSQL` are whole command prefixes, split on whitespace (their
// arguments are pod and role names, never paths with spaces); `COPROCESSOR_DB_CONTAINER` keeps
// naming a local container.
const psqlOverride = (env: NodeJS.ProcessEnv): Partial<Pick<TestEnvOverrides, "coprocessorDbPsql" | "merkleDbPsql">> => ({
  ...(env.COPROCESSOR_DB_PSQL
    ? { coprocessorDbPsql: env.COPROCESSOR_DB_PSQL.trim().split(/\s+/) }
    : env.COPROCESSOR_DB_CONTAINER
      ? { coprocessorDbPsql: coprocessorDbPsql(env.COPROCESSOR_DB_CONTAINER) }
      : {}),
  ...(env.MERKLE_DB_PSQL ? { merkleDbPsql: env.MERKLE_DB_PSQL.trim().split(/\s+/) } : {}),
});

const CLEARTEXT_DEFAULTS = {
  rpcUrl: CLEARTEXT_SOLANA_ENDPOINTS.validatorRpc,
  wsUrl: CLEARTEXT_SOLANA_ENDPOINTS.validatorWs,
  deployerKeypairPath: solanaCleartextDeployerPath,
} as const;

/** Assembles a validated TestEnv from `defaults <- overrides`. Exported for the scenario tests. */
export const resolveEnv = (
  overrides: Partial<TestEnvOverrides> = {},
  source: TestEnvSource = "local",
  network: SolanaNetwork = source === "devnet" ? "devnet" : "localnet",
): TestEnv => {
  const merged = {
    ...LOCAL_DEFAULTS,
    deployerKeypairPath: path.join(os.homedir(), ".config/solana/id.json"),
    ...(network === "devnet" ? { chainId: "130140237723663404" } : {}),
    ...(source === "cleartext" ? CLEARTEXT_DEFAULTS : {}),
    ...overrides,
  };
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
    coprocessorDbPsql: merged.coprocessorDbPsql,
    merkleDbPsql: merged.merkleDbPsql,
    roots: { deployerKeypairPath: merged.deployerKeypairPath },
    capabilities: capabilitiesFor(source, network),
    funding: FUNDING_BY_NETWORK[network],
  };
};

/** Builds the TestEnv the scenarios run against, from the current e2e runtime. */
export const loadEnv = (env: NodeJS.ProcessEnv = process.env): TestEnv =>
  resolveEnv(envOverrides(env), solanaE2eSource(env));
