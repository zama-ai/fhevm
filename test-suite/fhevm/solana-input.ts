// Solana input launcher — builds a REAL ZK input proof through the PUBLIC `@fhevm/sdk/solana`
// encrypt client (RFC-021 bytes32 identities + 128-byte aux) and POSTs it to the live relayer's
// `/v2/input-proof` seam. Prints the relayer response (carrying the `jobId`) as JSON on stdout.
//
// Inputs via env (hex unless noted):
//   IN_RELAYER_URL          relayer base URL (e.g. http://127.0.0.1:3000)
//   IN_CONTRACTS_CHAIN_ID   decimal u64 — the Solana host chain id embedded in the handle
//   IN_ACL_PROGRAM          zama-host program id as bytes32 (0x-hex) — the Solana ACL identity
//   IN_CONTRACT             bound contract identity as bytes32 (0x-hex)
//   IN_USER                 bound user identity as bytes32 (0x-hex)
//   IN_CONTRACT_B58         the relayer-facing contract address (base58) for the POST body
//   IN_USER_B58             the relayer-facing user address (base58) for the POST body
//   IN_VALUE                decimal — the value to encrypt (default 42)
//   IN_TYPE                 FHE type name (default uint64)
//
// Output: the relayer's JSON response on stdout (the harness reads `jobId` from it).
//
// Run under node (e.g. `node solana-input.ts` on Node >= 22.6 / native on Node 24), NOT bun: the
// TFHE WASM prover resolves its worker/wasm via node's locate-file path, which bun's browser-like
// environment detection bypasses (throws "Missing locate file function").

import { createFhevmEncryptClient, setFhevmRuntimeConfig } from '@fhevm/sdk/solana';
import { defineFhevmSolanaChain } from '@fhevm/sdk/chains';
import type { Bytes32Hex } from '@fhevm/sdk/types';

// The relayer's keyurl points at the docker-internal host `minio:9000`; rewrite it to the
// host-published port so this host-side client can fetch the FHE public key.
const _fetch = globalThis.fetch;
globalThis.fetch = ((url: string | URL | Request, opts?: RequestInit) =>
  _fetch(typeof url === 'string' ? url.replace('//minio:9000', '//127.0.0.1:9000') : url, opts)) as typeof fetch;

function reqEnv(name: string): string {
  const v = process.env[name];
  if (v === undefined || v === '') throw new Error(`missing env ${name}`);
  return v;
}

const RELAYER = reqEnv('IN_RELAYER_URL');
const SID = BigInt(reqEnv('IN_CONTRACTS_CHAIN_ID'));
const value = BigInt(process.env.IN_VALUE ?? '42');
const type = process.env.IN_TYPE ?? 'uint64';

const chain = defineFhevmSolanaChain({
  id: SID,
  fhevm: {
    relayerUrl: RELAYER,
    // domainKeys are unused by the encrypt path; the ACL identity is the program id below.
    acl: { domainKeys: [reqEnv('IN_ACL_PROGRAM') as Bytes32Hex] },
  },
});

setFhevmRuntimeConfig({
  auth: { type: 'ApiKeyHeader', value: process.env.ZAMA_FHEVM_API_KEY ?? 'local' },
});

const client = createFhevmEncryptClient({
  chain,
  aclProgramAddress: reqEnv('IN_ACL_PROGRAM') as Bytes32Hex,
});

// The coprocessor's EIP-712 attestation is verified on-chain in-frame when the input is consumed as
// an fhe_eval VerifiedInput operand (the fromExternal path); the proof attests the deployer as both
// the user and the contract identity. No persistent input ACL is created (EVM parity) — the input is
// transient-allowed for that eval only, and derived durable outputs are ACL'd by the consuming app.
const proof = await client.buildInputProof({
  contractAddress: reqEnv('IN_CONTRACT'),
  userAddress: reqEnv('IN_USER'),
  values: [{ type, value }],
});

const ctHex = Buffer.from(proof.ciphertextWithZkProof).toString('hex');
const body = {
  contractChainId: SID.toString(),
  contractAddress: reqEnv('IN_CONTRACT_B58'),
  userAddress: reqEnv('IN_USER_B58'),
  ciphertextWithInputVerification: ctHex,
  extraData: '0x00',
};

const res = await fetch(RELAYER + '/v2/input-proof', {
  method: 'POST',
  headers: { 'content-type': 'application/json' },
  body: JSON.stringify(body),
});
const text = await res.text();
process.stdout.write(text + '\n');

// The TFHE WASM prover spins up worker threads that keep the event loop alive, so the process
// never exits on its own; exit explicitly now that the proof is submitted.
process.exit(res.ok ? 0 : 1);
