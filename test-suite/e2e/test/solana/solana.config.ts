import path from 'node:path';

// REPO_ROOT is the fhevm monorepo root. Used only to locate the local-cli Cargo manifest,
// which must be cargo-run from the host machine against the docker-exposed Solana RPC.
export const REPO_ROOT = path.resolve(__dirname, '../../../../');

// local-cli is a Rust CLI that constructs and submits FHE-specific Solana transactions
// (encrypt, compute, allow, scenarios). It runs locally and talks to the docker Solana
// validator at SOLANA_RPC_URL. Not replaceable with Anchor CLI — domain-specific.
export const LOCAL_CLI_MANIFEST_PATH = path.join(
  REPO_ROOT,
  'solana-host-contracts',
  'local-cli',
  'Cargo.toml',
);

// Service URLs default to the docker-compose exposed ports but can be overridden
// (e.g. when running against a remote stack or a different local port mapping).
export const DB_CONTAINER = 'coprocessor-and-kms-db';
export const RELAYER_URL = process.env.RELAYER_URL ?? 'http://127.0.0.1:3000';
export const SOLANA_RPC_URL = process.env.SOLANA_HOST_RPC_URL ?? 'http://127.0.0.1:18999';
export const SOLANA_E2E_COMMITMENT = 'confirmed';

export const INPUT_PROOF_U64_VALUE = '18446744073709550042';
export const ADD42_INPUT_VALUE = '7';
export const TOKEN_TRANSFER_INPUT_VALUE = '1337';

export const SERIALIZED_SIZE_LIMIT_CIPHERTEXT = BigInt(1024 * 1024 * 512);
export const SERIALIZED_SIZE_LIMIT_PK = BigInt(1024 * 1024 * 512);
export const SERIALIZED_SIZE_LIMIT_CRS = BigInt(1024 * 1024 * 512);

export const RAW_CT_HASH_DOMAIN_SEPARATOR = 'ZK-w_rct';
export const HANDLE_HASH_DOMAIN_SEPARATOR = 'ZK-w_hdl';
export const INPUT_PROOF_EXTRA_DATA_VERSION = 0x01;
export const DECRYPTION_EXTRA_DATA_VERSION = 0x01;
export const MAX_UINT64 = BigInt('18446744073709551615');
export const INPUT_ENCRYPTION_TYPES: Record<number, number> = {
  2: 0,
  8: 2,
  16: 3,
  32: 4,
  64: 5,
  128: 6,
  160: 7,
  256: 8,
};
