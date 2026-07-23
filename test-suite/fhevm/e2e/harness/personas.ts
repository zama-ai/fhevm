// personas — named actors the scenarios act as.
//
// Zero protocol knowledge: a persona is just a Solana keypair on disk plus a capability-gated
// funding helper. It never touches the fhevm protocol; scenarios drive the protocol through
// `@fhevm/sdk` actions using these actors' keys.

import fs from "node:fs/promises";

import { createKeyPairSignerFromBytes } from "@solana/kit";

import { run } from "../../src/utils/process";
import type { TestEnv } from "./loadEnv";

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
   * Tops a persona up with SOL. Gated on the `faucet` capability: on a live network (no faucet) it
   * throws rather than silently no-op, so a scenario that assumes funding fails loudly.
   */
  fund(persona: Persona, sol?: number): Promise<void>;
};

/** Resolves the personas available in this environment. Only the deployer is loaded from disk. */
export const loadPersonas = async (env: TestEnv): Promise<Personas> => {
  const deployer = await readKeypair("deployer", env.roots.deployerKeypairPath);
  return {
    deployer,
    async fund(persona, sol = 5) {
      if (!env.capabilities.faucet) {
        throw new Error(`cannot fund ${persona.name}: environment "${env.source}" has no faucet capability`);
      }
      await run(["solana", "airdrop", String(sol), persona.address, "--url", env.rpcUrl]);
    },
  };
};
