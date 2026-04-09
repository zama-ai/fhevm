#!/usr/bin/env bun
// ---------------------------------------------------------------------------
// deploy-solana-verifier.ts
//
// Deploys the SolanaEd25519Verifier (+ its library dependencies) onto the
// EVM gateway chain and writes SOLANA_ED25519_VERIFIER_ADDRESS to the
// test-suite env file so e2e tests can read it via process.env.
//
// Called by fhevm-cli's `up` flow (gateway-deploy step) when a Solana host
// chain is present.  Not meant to be run manually.
//
// Required env vars:
//   GATEWAY_RPC_URL              — HTTP RPC of the gateway chain
//   DEPLOYER_PRIVATE_KEY         — hex private key for the deployer
//   TEST_SUITE_ENV_FILE          — absolute path to the test-suite .env file
//   REPO_ROOT                    — absolute path to the fhevm monorepo root
// ---------------------------------------------------------------------------

import fs from 'node:fs';
import path from 'node:path';

import { ethers } from 'ethers';

const GATEWAY_RPC_URL = process.env.GATEWAY_RPC_URL ?? '';
const DEPLOYER_PRIVATE_KEY = process.env.DEPLOYER_PRIVATE_KEY ?? '';
const TEST_SUITE_ENV_FILE = process.env.TEST_SUITE_ENV_FILE ?? '';
const REPO_ROOT = process.env.REPO_ROOT ?? '';

if (!GATEWAY_RPC_URL) {
  throw new Error('GATEWAY_RPC_URL is required');
}
if (!DEPLOYER_PRIVATE_KEY) {
  throw new Error('DEPLOYER_PRIVATE_KEY is required');
}
if (!TEST_SUITE_ENV_FILE) {
  throw new Error('TEST_SUITE_ENV_FILE is required');
}
if (!REPO_ROOT) {
  throw new Error('REPO_ROOT is required');
}

function loadVerifierArtifact(name: string): { abi: ethers.InterfaceAbi; bytecode: string; linkReferences: Record<string, Record<string, Array<{ start: number; length: number }>>> } {
  const artifactPath = path.join(
    REPO_ROOT,
    'gateway-contracts/artifacts/contracts/verifiers',
    `${name}.sol`,
    `${name}.json`,
  );
  return JSON.parse(fs.readFileSync(artifactPath, 'utf8'));
}

function linkArtifactBytecode(
  artifact: ReturnType<typeof loadVerifierArtifact>,
  libraries: Record<string, string> = {},
): string {
  if (typeof artifact?.bytecode !== 'string') {
    throw new Error('gateway verifier artifact is missing bytecode');
  }

  let bytecode = artifact.bytecode.replace(/^0x/i, '');
  for (const [, contracts] of Object.entries(artifact.linkReferences ?? {})) {
    for (const [contractName, references] of Object.entries(contracts)) {
      const address = libraries[contractName];
      if (!address) {
        throw new Error(`missing library address for ${contractName}`);
      }
      const normalizedAddress = ethers.getAddress(address).replace(/^0x/i, '');
      for (const reference of references) {
        const start = reference.start * 2;
        const length = reference.length * 2;
        bytecode =
          `${bytecode.slice(0, start)}${normalizedAddress}${bytecode.slice(start + length)}`;
      }
    }
  }
  return `0x${bytecode}`;
}

async function deployArtifact(
  name: string,
  signer: ethers.Wallet,
  libraries: Record<string, string>,
  nonce: number,
): Promise<ethers.BaseContract> {
  const artifact = loadVerifierArtifact(name);
  const bytecode = linkArtifactBytecode(artifact, libraries);
  const factory = new ethers.ContractFactory(artifact.abi, bytecode, signer);
  // Use an explicit gasLimit to bypass estimateGas — the Sha512 and Ed25519
  // library contracts are very large and local test nodes may reject the
  // estimate even though the deployment succeeds within the block gas limit.
  const contract = await factory.deploy({ nonce, gasLimit: 30_000_000n });
  await contract.waitForDeployment();
  return contract;
}

async function main() {
  const rpcUrl = (() => {
    const parsed = new URL(GATEWAY_RPC_URL);
    if (parsed.hostname === '0.0.0.0' || parsed.hostname === 'gateway-node') {
      parsed.hostname = '127.0.0.1';
    }
    return parsed.toString();
  })();

  const signer = new ethers.Wallet(
    DEPLOYER_PRIVATE_KEY.startsWith('0x') ? DEPLOYER_PRIVATE_KEY : `0x${DEPLOYER_PRIVATE_KEY}`,
    new ethers.JsonRpcProvider(rpcUrl),
  );
  let nonce = await signer.getNonce('pending');

  console.log(`[deploy-solana-verifier] deploying on ${rpcUrl} with nonce ${nonce}`);

  const [sha512, ed25519Pow] = await Promise.all([
    deployArtifact('Sha512', signer, {}, nonce),
    deployArtifact('Ed25519_pow', signer, {}, nonce + 1),
  ]);
  nonce += 2;
  const ed25519Library = await deployArtifact('Ed25519', signer, {
    Sha512: await sha512.getAddress(),
    Ed25519_pow: await ed25519Pow.getAddress(),
  }, nonce);
  nonce += 1;
  const verifier = await deployArtifact('SolanaEd25519Verifier', signer, {
    Ed25519: await ed25519Library.getAddress(),
  }, nonce);

  const verifierAddress = await verifier.getAddress();
  console.log(`[deploy-solana-verifier] SOLANA_ED25519_VERIFIER_ADDRESS=${verifierAddress}`);

  // Append or update the key in the test-suite env file.
  let existing = '';
  try {
    existing = fs.readFileSync(TEST_SUITE_ENV_FILE, 'utf8');
  } catch (error: unknown) {
    if ((error as NodeJS.ErrnoException).code !== 'ENOENT') throw error;
  }
  const lines = existing.split('\n').filter(
    (line) => !line.startsWith('SOLANA_ED25519_VERIFIER_ADDRESS='),
  );
  lines.push(`SOLANA_ED25519_VERIFIER_ADDRESS=${verifierAddress}`);
  fs.writeFileSync(TEST_SUITE_ENV_FILE, lines.filter(Boolean).join('\n') + '\n');
  console.log(`[deploy-solana-verifier] written to ${TEST_SUITE_ENV_FILE}`);
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
