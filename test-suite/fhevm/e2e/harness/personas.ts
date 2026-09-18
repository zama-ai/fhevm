// personas — named actors the scenarios act as.
//
// Zero protocol knowledge: a persona is just a Solana keypair on disk plus a capability-gated
// funding helper. It never touches the fhevm protocol; scenarios drive the protocol through
// `@fhevm/sdk` actions using these actors' keys.

import fs from "node:fs/promises";

import { address, createKeyPairSignerFromBytes } from "@solana/kit";

import type { TestEnv } from "./loadEnv";
import { openProvisioning } from "./solana/provisioning";

export type Persona = {
  readonly name: string;
  readonly keypairPath: string;
  readonly address: string;
};

const readKeypair = async (name: string, keypairPath: string): Promise<Persona> => {
  const bytes = JSON.parse(await fs.readFile(keypairPath, "utf8")) as unknown;
  if (!Array.isArray(bytes) || bytes.length !== 64 || bytes.some((b) => !Number.isInteger(b) || b < 0 || b > 255)) {
    throw new Error(`persona ${name}: ${keypairPath} is not a 64-byte Solana keypair`);
  }
  // Derive the address from the keypair bytes directly (no `solana` CLI / PATH dependency); the SDK
  // also validates the key material as it decodes it.
  const signer = await createKeyPairSignerFromBytes(Uint8Array.from(bytes as number[]));
  return { name, keypairPath, address: signer.address };
};

export type Personas = {
  /** The stack deployer wallet — the actor whose ACL grants the default handle authorizations. */
  readonly deployer: Persona;
  /**
   * Named actors loaded from disk beyond the deployer. The confidential-vault demo (#1760) uses this
   * to load its `keeper` (the operator who plays dispatch + settle — settle must read as an operator
   * action, not a user button) alongside the end-user personas that deposit and redeem.
   */
  readonly roles: Readonly<Record<string, Persona>>;
  /**
   * Tops a persona up with SOL: an airdrop where the environment has a faucet, otherwise a
   * transfer from the deployer wallet. Defaults to the environment's primary funding amount.
   */
  fund(persona: Persona, sol?: number): Promise<void>;
};

/**
 * Resolves the personas available in this environment. The deployer is always loaded from disk;
 * `extraRoles` (name → keypair path) loads any additional named actors a scenario needs — e.g. the
 * demo's `{ keeper, alice, bob }`.
 */
export const loadPersonas = async (
  env: TestEnv,
  extraRoles: Readonly<Record<string, string>> = {},
): Promise<Personas> => {
  const deployer = await readKeypair("deployer", env.roots.deployerKeypairPath);
  const roles: Record<string, Persona> = {};
  for (const [name, keypairPath] of Object.entries(extraRoles)) {
    roles[name] = await readKeypair(name, keypairPath);
  }
  return {
    deployer,
    roles,
    async fund(persona, sol = env.funding.primarySol) {
      const context = await openProvisioning(env);
      await context.fundSol(address(persona.address), sol);
    },
  };
};
