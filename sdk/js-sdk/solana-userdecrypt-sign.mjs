// Solana V2 user-decrypt SIGNER: builds the canonical V2 request via the js-sdk
// `buildSolanaUserDecryptRequest` (ed25519 over the `zama-solana-user-decrypt-v1` preimage)
// and prints the resulting request object as JSON on stdout. The ML-KEM re-encryption keypair
// lives in the Rust caller (it must, to de-signcrypt the response), so the ML-KEM *public key*
// is passed in here and bound into the signed preimage. No ML-KEM crypto runs in this script.
//
// All inputs via env (hex unless noted):
//   UD_CONTRACTS_CHAIN_ID   decimal u64 (the Solana host chain id embedded in the handle)
//   UD_PUBLIC_KEY           ML-KEM re-encryption public key bytes (0x-hex), bound in the preimage
//   UD_HANDLE               single 32-byte ciphertext handle (0x-hex)
//   UD_IDENTITY             user's 32-byte ed25519 pubkey (0x-hex)
//   UD_SECRET_KEY           user's 32-byte ed25519 seed (0x-hex)
//   UD_CONTEXT_ID           32-byte big-endian context id (0x-hex)
//   UD_NONCE                32-byte anti-replay nonce (0x-hex)
//   UD_ALLOWED_DOMAIN_KEYS  comma-separated 32-byte ACL domain keys (0x-hex)
//   UD_START_TIMESTAMP      decimal u64 (unix seconds)
//   UD_DURATION_SECONDS     decimal u64
import { buildSolanaUserDecryptRequest } from './src/_esm/core/coprocessor/SolanaUserDecrypt-p.js';

function hexToBytes(hex) {
  const s = hex.startsWith('0x') ? hex.slice(2) : hex;
  if (s.length % 2 !== 0) throw new Error(`odd-length hex: ${hex}`);
  return Uint8Array.from(Buffer.from(s, 'hex'));
}

function req(name) {
  const v = process.env[name];
  if (v === undefined || v === '') throw new Error(`missing env ${name}`);
  return v;
}

const allowedDomainKeys = req('UD_ALLOWED_DOMAIN_KEYS')
  .split(',')
  .map((s) => s.trim())
  .filter((s) => s.length > 0)
  .map(hexToBytes);

const request = buildSolanaUserDecryptRequest(
  {
    contractsChainId: BigInt(req('UD_CONTRACTS_CHAIN_ID')),
    publicKey: hexToBytes(req('UD_PUBLIC_KEY')),
    handles: [hexToBytes(req('UD_HANDLE'))],
    identity: hexToBytes(req('UD_IDENTITY')),
    contextId: hexToBytes(req('UD_CONTEXT_ID')),
    nonce: hexToBytes(req('UD_NONCE')),
    allowedAclDomainKeys: allowedDomainKeys,
    startTimestamp: BigInt(req('UD_START_TIMESTAMP')),
    durationSeconds: BigInt(req('UD_DURATION_SECONDS')),
  },
  hexToBytes(req('UD_SECRET_KEY')),
);

process.stdout.write(JSON.stringify(request) + '\n');
