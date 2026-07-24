// personas — named actors the scenarios act as.
//
// Zero protocol knowledge: a persona is just a Solana keypair on disk plus a capability-gated
// funding helper. It never touches the fhevm protocol; scenarios drive the protocol through
// `@fhevm/sdk` actions using these actors' keys.

import fs from "node:fs/promises";

import { address, createKeyPairSignerFromBytes, createSolanaRpc, lamports } from "@solana/kit";

import type { TestEnv } from "./loadEnv";
import { until } from "./until";

const LAMPORTS_PER_SOL = 1_000_000_000n;

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
   * Tops a persona up with SOL. Gated on the `faucet` capability: on a live network (no faucet) it
   * throws rather than silently no-op, so a scenario that assumes funding fails loudly.
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
    async fund(persona, sol = 5) {
      if (!env.capabilities.faucet) {
        throw new Error(`cannot fund ${persona.name}: environment "${env.source}" has no faucet capability`);
      }
      // Airdrop over the validator RPC (kit's requestAirdrop), not the `solana` CLI: no PATH or
      // ambient-config dependency, and the confirmation wait is explicit rather than the CLI's.
      const rpc = createSolanaRpc(env.rpcUrl);
      const signature = await rpc
        .requestAirdrop(address(persona.address), lamports(BigInt(sol) * LAMPORTS_PER_SOL), { commitment: "confirmed" })
        .send();
      await until(
        async () => {
          const { value } = await rpc.getSignatureStatuses([signature]).send();
          const status = value[0];
          if (status?.err) throw new Error(`airdrop for ${persona.name} failed: ${JSON.stringify(status.err)}`);
          const level = status?.confirmationStatus;
          return level === "confirmed" || level === "finalized";
        },
        { description: `airdrop confirmation for ${persona.name}`, timeoutMs: 30_000 },
      );
    },
  };
};
