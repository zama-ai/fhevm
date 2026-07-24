// demo-config — the runtime artifact the confidential-vault demo publishes after seeding, and the
// single source of truth the dApp (#1761), the smoke (deposit arc) and the rehearsal (#1762) read.
//
// It carries ROOTS ONLY (program ids, the two batchers + their per-batch settle lookup tables, the
// vault, the four mints, host/kms context, persona pubkeys, endpoints, chain id). Everything a
// caller needs to reach a vault-module action — including settle's full 34-account set — is DERIVED
// from these roots by the SDK's `deriveBatchAddresses`/`deriveSettleAccounts` and its on-chain reads
// (`getCurrentBatch`, the generated `EncryptedValue` decoder). An address dump would be a confession
// that the SDK cannot serve a real integrator, so nothing derivable belongs here.
//
// `depositRoots`/`redeemRoots` project the config onto the SDK's normative `VaultDemoRoots` for each
// batcher direction: the SAME two confidential mints and the SAME two underlyings serve both, with
// join/payout swapped. That swap is the one thing a reviewer must eyeball (review focus #6), so it
// lives in exactly one place here rather than being re-threaded at every call site.

import fs from "node:fs/promises";
import path from "node:path";

import { address, type Address } from "@solana/kit";

/**
 * Structural mirror of the SDK's normative `VaultDemoRoots` (`@fhevm/sdk/solana/vault`).
 *
 * test-suite reaches the SDK only through runtime dynamic imports (the SDK's `_types` are not built
 * at `tsc` check time — see `src/solana/current-user-decrypt.ts`), so a static type import would not
 * resolve here. The 12-field shape is restated so `depositRoots`/`redeemRoots` stay typed; the seed
 * passes these objects straight into `deriveBatchAddresses`, where compatibility is structural. Keep
 * this in lockstep with the SDK interface (names/shape are fixed there).
 */
export type VaultDemoRoots = {
  readonly batcherProgram: Address;
  readonly tokenProgram: Address;
  readonly vaultProgram: Address;
  readonly hostProgram: Address;
  readonly batcher: Address;
  readonly vault: Address;
  readonly joinConfidentialMint: Address;
  readonly payoutConfidentialMint: Address;
  readonly joinUnderlyingMint: Address;
  readonly payoutUnderlyingMint: Address;
  readonly hostConfig: Address;
  readonly kmsContext: Address;
};

/** Where `demo-seed` writes the artifact and every consumer reads it from, unless overridden. */
export const DEMO_CONFIG_DEFAULT_PATH = ".fhevm/runtime/solana-demo.json";

/**
 * The config path every producer/consumer honors: `DEMO_CONFIG_PATH` if set, else the CWD-relative
 * default. demo-up.sh and the solana-e2e workflow export an ABSOLUTE `DEMO_CONFIG_PATH` ($ROOT-based)
 * so the path is stable regardless of which directory a step runs from (the seed runs from
 * test-suite/fhevm, later steps from the repo root). Resolved at call time so the env is read live.
 */
export const resolveDemoConfigPath = (): string => process.env.DEMO_CONFIG_PATH ?? DEMO_CONFIG_DEFAULT_PATH;

/** Roots for one batcher instance: its account plus the settle lookup table `open_batch` created. */
export type DemoBatcher = {
  /** The `Batcher` account address (`initialize_batcher`). */
  readonly batcher: Address;
  /** The per-batch settle address lookup table `open_batch` created for the current batch. */
  readonly lookupTable: Address;
};

/** Persona pubkeys, labeled by demo role. Keys sign from committed keypairs, not from this file. */
export type DemoPersonas = {
  /** The operator/keeper — plays dispatch + settle. #1761 must present settle as an operator action. */
  readonly keeper: Address;
  /** End-user personas that deposit and redeem. */
  readonly alice: Address;
  readonly bob: Address;
};

/**
 * The demo-config artifact. Deliberately keeps `relayerUrl`/`proofServiceUrl` OUTSIDE any
 * `FhevmSolanaChain` shape — they are operator endpoints, not chain identity.
 */
export type SolanaDemoConfig = {
  readonly source: "demo-config";
  /** RFC-021 Solana host chain id, as an unsigned decimal string (`9223372036854788153`). */
  readonly chainId: string;
  readonly rpcUrl: string;
  readonly wsUrl: string;
  readonly relayerUrl: string;
  readonly proofServiceUrl: string;
  readonly gatewayRpcUrl: string;
  /** zama-host program id as bytes32 hex — the Solana ACL identity. */
  readonly aclProgram: `0x${string}`;
  /** KMS/gateway user-decrypt context id, unsigned decimal string. */
  readonly userDecryptContextId: string;
  /**
   * Lamports the settle caller funds the batch authority with (unsigned decimal string). The amount
   * must suffice to cover the rent settle's CPIs charge to this batch's authority; the seed records
   * the `open_batch` value as a known-good amount.
   */
  readonly authorityFundingLamports: string;
  readonly programs: {
    readonly batcher: Address;
    readonly token: Address;
    readonly vault: Address;
    readonly host: Address;
  };
  readonly hostConfig: Address;
  readonly kmsContext: Address;
  /** The `demo_vault` vault PDA (underlying = mock USDC; share mint created by the program). */
  readonly vault: Address;
  readonly mints: {
    /** Mock USDC (SPL, 6 decimals) — the deposit underlying. */
    readonly joinUnderlying: Address;
    /** The vault share mint — the payout underlying on deposit / the join underlying on redeem. */
    readonly payoutUnderlying: Address;
    /** cUSDC — the confidential mint wrapping USDC. */
    readonly joinConfidential: Address;
    /** cShares — the confidential mint wrapping the vault share mint. */
    readonly payoutConfidential: Address;
  };
  readonly batchers: {
    /** Deposit direction: join = cUSDC, payout = cShares. */
    readonly deposit: DemoBatcher;
    /** Redeem direction: join = cShares, payout = cUSDC. */
    readonly redeem: DemoBatcher;
  };
  /** The SPL mint authority for the mock-USDC faucet (committed demo keypair; pubkey only here). */
  readonly mintAuthority: Address;
  readonly personas: DemoPersonas;
};

const asAddress = (value: unknown, field: string): Address => {
  if (typeof value !== "string") throw new Error(`demo-config: ${field} must be a base58 address string`);
  try {
    return address(value);
  } catch {
    throw new Error(`demo-config: ${field} is not a valid Solana address: ${value}`);
  }
};

const asString = (value: unknown, field: string): string => {
  if (typeof value !== "string" || value === "") throw new Error(`demo-config: ${field} must be a non-empty string`);
  return value;
};

const asDecimal = (value: unknown, field: string): string => {
  const s = asString(value, field);
  if (!/^\d+$/.test(s)) throw new Error(`demo-config: ${field} must be an unsigned decimal integer, got ${s}`);
  return s;
};

const asBytes32Hex = (value: unknown, field: string): `0x${string}` => {
  const s = asString(value, field);
  if (!/^0x[0-9a-f]{64}$/i.test(s)) throw new Error(`demo-config: ${field} must be 0x-prefixed 32-byte hex, got ${s}`);
  return s as `0x${string}`;
};

/**
 * Parses and fully validates a demo-config object. Every field is checked so a malformed artifact
 * fails at load with a named field, never later inside an SDK call with an opaque base58 error.
 */
export const parseDemoConfig = (raw: unknown): SolanaDemoConfig => {
  if (typeof raw !== "object" || raw === null) throw new Error("demo-config: root must be an object");
  const o = raw as Record<string, unknown>;
  const obj = (value: unknown, field: string): Record<string, unknown> => {
    if (typeof value !== "object" || value === null) throw new Error(`demo-config: ${field} must be an object`);
    return value as Record<string, unknown>;
  };
  const programs = obj(o.programs, "programs");
  const mints = obj(o.mints, "mints");
  const batchers = obj(o.batchers, "batchers");
  const deposit = obj(batchers.deposit, "batchers.deposit");
  const redeem = obj(batchers.redeem, "batchers.redeem");
  const personas = obj(o.personas, "personas");
  return {
    source: "demo-config",
    chainId: asDecimal(o.chainId, "chainId"),
    rpcUrl: asString(o.rpcUrl, "rpcUrl"),
    wsUrl: asString(o.wsUrl, "wsUrl"),
    relayerUrl: asString(o.relayerUrl, "relayerUrl"),
    proofServiceUrl: asString(o.proofServiceUrl, "proofServiceUrl"),
    gatewayRpcUrl: asString(o.gatewayRpcUrl, "gatewayRpcUrl"),
    aclProgram: asBytes32Hex(o.aclProgram, "aclProgram"),
    userDecryptContextId: asDecimal(o.userDecryptContextId, "userDecryptContextId"),
    authorityFundingLamports: asDecimal(o.authorityFundingLamports, "authorityFundingLamports"),
    programs: {
      batcher: asAddress(programs.batcher, "programs.batcher"),
      token: asAddress(programs.token, "programs.token"),
      vault: asAddress(programs.vault, "programs.vault"),
      host: asAddress(programs.host, "programs.host"),
    },
    hostConfig: asAddress(o.hostConfig, "hostConfig"),
    kmsContext: asAddress(o.kmsContext, "kmsContext"),
    vault: asAddress(o.vault, "vault"),
    mints: {
      joinUnderlying: asAddress(mints.joinUnderlying, "mints.joinUnderlying"),
      payoutUnderlying: asAddress(mints.payoutUnderlying, "mints.payoutUnderlying"),
      joinConfidential: asAddress(mints.joinConfidential, "mints.joinConfidential"),
      payoutConfidential: asAddress(mints.payoutConfidential, "mints.payoutConfidential"),
    },
    batchers: {
      deposit: { batcher: asAddress(deposit.batcher, "batchers.deposit.batcher"), lookupTable: asAddress(deposit.lookupTable, "batchers.deposit.lookupTable") },
      redeem: { batcher: asAddress(redeem.batcher, "batchers.redeem.batcher"), lookupTable: asAddress(redeem.lookupTable, "batchers.redeem.lookupTable") },
    },
    mintAuthority: asAddress(o.mintAuthority, "mintAuthority"),
    personas: {
      keeper: asAddress(personas.keeper, "personas.keeper"),
      alice: asAddress(personas.alice, "personas.alice"),
      bob: asAddress(personas.bob, "personas.bob"),
    },
  };
};

/** Reads and validates the demo-config JSON from disk. */
export const readDemoConfig = async (configPath = resolveDemoConfigPath()): Promise<SolanaDemoConfig> => {
  const text = await fs.readFile(configPath, "utf8");
  return parseDemoConfig(JSON.parse(text));
};

/** Writes the demo-config JSON to disk, creating the runtime directory. Round-trips through the parser. */
export const writeDemoConfig = async (config: SolanaDemoConfig, configPath = resolveDemoConfigPath()): Promise<void> => {
  const validated = parseDemoConfig(config); // never persist an artifact that would not re-load
  await fs.mkdir(path.dirname(configPath), { recursive: true });
  await fs.writeFile(configPath, `${JSON.stringify(validated, null, 2)}\n`, "utf8");
};

const commonRoots = (config: SolanaDemoConfig): Pick<VaultDemoRoots, "batcherProgram" | "tokenProgram" | "vaultProgram" | "hostProgram" | "vault" | "hostConfig" | "kmsContext"> => ({
  batcherProgram: config.programs.batcher,
  tokenProgram: config.programs.token,
  vaultProgram: config.programs.vault,
  hostProgram: config.programs.host,
  vault: config.vault,
  hostConfig: config.hostConfig,
  kmsContext: config.kmsContext,
});

/** Projects the config onto the deposit-direction `VaultDemoRoots`: join = cUSDC → payout = cShares. */
export const depositRoots = (config: SolanaDemoConfig): VaultDemoRoots => ({
  ...commonRoots(config),
  batcher: config.batchers.deposit.batcher,
  joinConfidentialMint: config.mints.joinConfidential,
  payoutConfidentialMint: config.mints.payoutConfidential,
  joinUnderlyingMint: config.mints.joinUnderlying,
  payoutUnderlyingMint: config.mints.payoutUnderlying,
});

/** Projects the config onto the redeem-direction `VaultDemoRoots`: join = cShares → payout = cUSDC. */
export const redeemRoots = (config: SolanaDemoConfig): VaultDemoRoots => ({
  ...commonRoots(config),
  batcher: config.batchers.redeem.batcher,
  joinConfidentialMint: config.mints.payoutConfidential,
  payoutConfidentialMint: config.mints.joinConfidential,
  joinUnderlyingMint: config.mints.payoutUnderlying,
  payoutUnderlyingMint: config.mints.joinUnderlying,
});
