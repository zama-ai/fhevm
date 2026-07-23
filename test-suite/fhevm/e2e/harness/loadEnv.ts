// loadEnv — builds the TestEnv that the Solana e2e scenarios run against.
//
// Zero protocol knowledge: it only assembles endpoints, chain identifiers, on-disk roots, and
// capability flags. It never encodes/decodes protocol bytes — scenarios reach the protocol solely
// through `@fhevm/sdk` Solana actions.
//
// Source for NOW: the local clean-e2e stack. Every value below is exactly what the current e2e
// runtime provides, traced to where it lands:
//   - urls/ids: `solana/scripts/e2e/full-vertical.sh` (RPC/GW_RPC/relayer/proof-service, SID, ACL,
//     CTX) and `test-suite/fhevm/src/solana/two-holder-transfer.ts` (RPC/WS/relayer/ACL constants).
//   - coprocessor DB container: `test-suite/fhevm/src/layout.ts` (COPROCESSOR_DB_CONTAINER).
//   - deployer keypair: `~/.config/solana/id.json`, the wallet full-vertical.sh derives USER from.
//   - gateway addresses (optional, for future legs): generated at `fhevm-cli up` time into
//     `.fhevm/runtime/addresses/gateway/.env.gateway`.
// Env vars override any field so a run can point at a non-default local stack.
//
// A second source (a demo-config JSON for local, or a devnet/mainnet manifest) will be added later.
// `resolveEnv` is written as `defaults <- overrides` so that second source only has to produce a
// `Partial<TestEnvOverrides>`; nothing else here changes.

import os from "node:os";
import path from "node:path";

import {
  COPROCESSOR_DB_CONTAINER,
  SOLANA_ACL_PROGRAM,
  SOLANA_DEFAULT_USER_DECRYPT_CONTEXT,
} from "../../src/layout";

export type Capabilities = {
  /** Can fund actors with SOL (local validator airdrop). Local: true. Devnet/mainnet: false. */
  readonly faucet: boolean;
  /** Can create brand-new SPL / confidential mints for a scenario. Local & devnet: true. */
  readonly freshMints: boolean;
  /** Slots advance on demand (local validator). Live networks: false. */
  readonly fastSlots: boolean;
};

export type TestEnv = {
  /** The only source implemented today. Future: "demo-config" | "devnet" | "mainnet". */
  readonly source: "local";
  readonly rpcUrl: string;
  readonly wsUrl: string;
  readonly relayerUrl: string;
  readonly proofServiceUrl: string;
  readonly gatewayRpcUrl: string;
  /** RFC-021 Solana host chain id (9223372036854788153); the high bit marks it a Solana chain. */
  readonly chainId: bigint;
  /** zama-host program id as a bytes32 hex — the Solana ACL identity. */
  readonly aclProgram: `0x${string}`;
  /** KMS/gateway user-decrypt context id, as an unsigned decimal string. */
  readonly userDecryptContextId: string;
  /** Docker container name of the coprocessor+KMS Postgres (for ciphertext-materialization waits). */
  readonly coprocessorDbContainer: string;
  readonly roots: { readonly deployerKeypairPath: string };
  readonly capabilities: Capabilities;
};

type TestEnvOverrides = {
  rpcUrl: string;
  wsUrl: string;
  relayerUrl: string;
  proofServiceUrl: string;
  gatewayRpcUrl: string;
  chainId: string;
  aclProgram: string;
  userDecryptContextId: string;
  coprocessorDbContainer: string;
  deployerKeypairPath: string;
};

// The local clean-e2e stack. The endpoints are local-stack facts; the protocol identities
// (ACL program, user-decrypt context, coprocessor DB container) are imported from the CLI's config
// module so there is exactly one source of truth shared with the transfer orchestrator.
const LOCAL_DEFAULTS = {
  rpcUrl: "http://127.0.0.1:8899",
  wsUrl: "ws://127.0.0.1:8900",
  relayerUrl: "http://127.0.0.1:3000",
  proofServiceUrl: "http://127.0.0.1:8088",
  gatewayRpcUrl: "http://127.0.0.1:8546",
  chainId: "9223372036854788153",
  aclProgram: SOLANA_ACL_PROGRAM,
  userDecryptContextId: SOLANA_DEFAULT_USER_DECRYPT_CONTEXT,
  coprocessorDbContainer: COPROCESSOR_DB_CONTAINER,
} as const;

const CAPABILITIES_BY_SOURCE: Record<TestEnv["source"], Capabilities> = {
  local: { faucet: true, freshMints: true, fastSlots: true },
};

const bytes32Hex = (value: string): `0x${string}` => {
  if (!/^0x[0-9a-f]{64}$/i.test(value)) throw new Error(`expected a 0x-prefixed 32-byte hex value, got ${value}`);
  return value as `0x${string}`;
};

const solanaChainId = (value: string): bigint => {
  if (!/^\d+$/.test(value)) throw new Error(`chainId must be an unsigned decimal integer, got ${value}`);
  const id = BigInt(value);
  if ((id & (1n << 63n)) === 0n) throw new Error(`chainId ${value} is not a Solana high-bit chain id`);
  return id;
};

const decimalString = (value: string, name: string): string => {
  if (!/^\d+$/.test(value)) throw new Error(`${name} must be an unsigned decimal integer, got ${value}`);
  return value;
};

/** Reads TestEnv overrides from the process environment (the "now" source). */
const envOverrides = (env: NodeJS.ProcessEnv): Partial<TestEnvOverrides> => {
  const pick = <K extends keyof TestEnvOverrides>(key: K, name: string): Partial<Pick<TestEnvOverrides, K>> => {
    const value = env[name];
    return value === undefined || value === "" ? {} : ({ [key]: value } as Pick<TestEnvOverrides, K>);
  };
  return {
    ...pick("rpcUrl", "SOLANA_RPC_URL"),
    ...pick("wsUrl", "SOLANA_WS_URL"),
    ...pick("relayerUrl", "SOLANA_RELAYER_URL"),
    ...pick("proofServiceUrl", "PROOF_SERVICE_URL"),
    ...pick("gatewayRpcUrl", "GW_RPC"),
    ...pick("chainId", "SOLANA_HOST_CHAIN_ID"),
    ...pick("aclProgram", "SOLANA_ACL_PROGRAM"),
    ...pick("userDecryptContextId", "SOLANA_UD_CONTEXT_ID"),
    ...pick("coprocessorDbContainer", "COPROCESSOR_DB_CONTAINER"),
    ...pick("deployerKeypairPath", "SOLANA_DEPLOYER_KEYPAIR"),
  };
};

/** Assembles a validated TestEnv from `defaults <- overrides`. Exported for the scenario tests. */
export const resolveEnv = (overrides: Partial<TestEnvOverrides> = {}): TestEnv => {
  const merged = { ...LOCAL_DEFAULTS, ...overrides };
  const deployerKeypairPath =
    overrides.deployerKeypairPath ?? path.join(os.homedir(), ".config/solana/id.json");
  return {
    source: "local",
    rpcUrl: merged.rpcUrl,
    wsUrl: merged.wsUrl,
    relayerUrl: merged.relayerUrl,
    proofServiceUrl: merged.proofServiceUrl,
    gatewayRpcUrl: merged.gatewayRpcUrl,
    chainId: solanaChainId(merged.chainId),
    aclProgram: bytes32Hex(merged.aclProgram),
    userDecryptContextId: decimalString(merged.userDecryptContextId, "userDecryptContextId"),
    coprocessorDbContainer: merged.coprocessorDbContainer,
    roots: { deployerKeypairPath },
    capabilities: CAPABILITIES_BY_SOURCE.local,
  };
};

/** Builds the TestEnv the scenarios run against, from the current e2e runtime. */
export const loadEnv = (env: NodeJS.ProcessEnv = process.env): TestEnv => resolveEnv(envOverrides(env));
