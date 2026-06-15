// Solana input client: generates a REAL ZK input proof via the js-sdk buildSolana()
// (128-byte aux, bytes32 identities) and submits it to the live relayer /v2/input-proof.
// Run from sdk/js-sdk (imports the built _esm, which resolves the js-sdk node_modules).
// The relayer's keyurl points at the docker-internal host `minio:9000`; rewrite it to
// the host-published port so this host-side client can fetch the FHE public key.
const _fetch = globalThis.fetch;
globalThis.fetch = (url, opts) =>
  _fetch(typeof url === 'string' ? url.replace('//minio:9000', '//127.0.0.1:9000') : url, opts);

import { ethers } from 'ethers';
import { createFhevmEncryptClient, setFhevmRuntimeConfig } from './src/_esm/ethers/index.js';
import { createZkProofBuilder } from './src/_esm/core/coprocessor/ZkProofBuilder-p.js';
import { createTypedValue } from './src/_esm/core/base/typedValue.js';

const SID = 9223372036854788153n; // RFC-021 Solana host chain id (high bit | 12345)
const ACL = '0x9c7da263cccb5084844e292a2ce0db0e51bbf310100656aa4572b83dfe35fca5'; // zama-host program (bytes32)
const CONTRACT = '0x7d6c42046bfdeae9834fa3e94370d5fcb819025ce76ec90e99eb057dc54f2c9e'; // confidential-token (bytes32)
const USER = '0x1f6f8fbf847ad9e4ebad6dcabd9529035a622d6ba245ef25fbd6e17e850f6e36'; // deployer (bytes32)
const CONTRACT_B58 = '9Sbhx7VF6vAGdYApPikwHgJ68Z367Au2tTyw9FpBxnAh';
const USER_B58 = '37iJeLFz4Gfm3qRKQrY5ULnkuo67vXUEhYjXC9mC1CE9';
const RELAYER = 'http://127.0.0.1:3000';

const chain = {
  id: SID,
  fhevm: {
    contracts: {
      acl: { address: ACL },
      inputVerifier: { address: '0x0000000000000000000000000000000000000000' },
      kmsVerifier: { address: '0x0000000000000000000000000000000000000000' },
    },
    relayerUrl: RELAYER,
    gateway: {
      id: 54321,
      contracts: {
        decryption: { address: '0xF0bFB159C7381F7CB332586004d8247252C5b816' },
        inputVerification: { address: '0x35760912360E875DA50D40a74305575c23D55783' },
      },
    },
  },
};

setFhevmRuntimeConfig({
  auth: { type: 'ApiKeyHeader', value: process.env.ZAMA_FHEVM_API_KEY ?? 'local' },
  logger: { debug: (m) => console.log('[sdk]', m), error: (m) => console.log('[sdk:err]', m) },
});

const provider = new ethers.JsonRpcProvider('http://127.0.0.1:8546');
const fhevm = createFhevmEncryptClient({ chain, provider });

const builder = createZkProofBuilder();
builder.addTypedValue(createTypedValue({ type: 'uint64', value: 42n }));

console.log('--- building Solana ZK proof (value=42, uint64) ...');
// zama-host verify_coprocessor_input verifies the coprocessor's EIP-712 attestation on-chain; the
// proof attests the deployer as both the user and the contract identity. No persistent input ACL is
// created (EVM parity) — durable permission on an input-derived handle is a separate app grant.
const proof = await builder.buildSolana(fhevm, { contractAddress: USER, userAddress: USER });
console.log('proof prototype methods:', Object.getOwnPropertyNames(Object.getPrototypeOf(proof)));

// Inspect the proof's accessors (discover the ciphertext + handles API at runtime).
const ct = proof.ciphertextWithZkProof ?? proof.ciphertext ?? (typeof proof.toBytes === 'function' ? proof.toBytes() : undefined);
const handles = proof.handles ?? proof.inputHandles ?? undefined;
console.log('ciphertext bytes len:', ct?.length, 'handles:', handles);

const ctHex = Buffer.from(ct).toString('hex');
const body = {
  contractChainId: SID.toString(),
  contractAddress: USER_B58, // attested contract == deployer (bound into the deployer's ACL domain)
  userAddress: USER_B58,
  ciphertextWithInputVerification: ctHex,
  extraData: '0x00',
};
console.log('--- POST', RELAYER + '/v2/input-proof', 'ct len', ctHex.length / 2, 'bytes');
const res = await fetch(RELAYER + '/v2/input-proof', {
  method: 'POST',
  headers: { 'content-type': 'application/json' },
  body: JSON.stringify(body),
});
console.log('relayer status:', res.status);
console.log('relayer body:', await res.text());

// The TFHE WASM prover spins up worker threads that keep the event loop alive,
// so node never exits on its own; exit explicitly now that the proof is submitted.
process.exit(res.ok ? 0 : 1);
