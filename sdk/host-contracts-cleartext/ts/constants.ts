import { DEFAULT_COPROCESSOR_ADDRESSES } from './signers/defaultCoprocessorSigners.js';
import { DEFAULT_KMS_NODE_ADDRESSES } from './signers/defaultKmsSigners.js';
import { DEFAULT_KMS_NODE_TX_SENDER_ADDRESSES } from './signers/defaultKmsTxSenderSigners.js';
import type { BootstrapConfigV14, KmsNodeParams, KmsThresholds, PcrValues } from './types/public.js';

// Calculated as `address(uint160(uint256(keccak256("fhevm.cheat.address cleartext input verification"))))`.
export const DEFAULT_INPUT_VERIFICATION_ADDRESS = '0x6189F6c0c3E40B4a3c72ec86262295D78d845297';

// Calculated as `address(uint160(uint256(keccak256("fhevm.cheat.address cleartext decryption"))))`.
export const DEFAULT_DECRYPTION_ADDRESS = '0xEaaA2FC6BC259dF015Aa7Dc8e59e0B67df622721';

export const DEFAULT_HCU_CAP_PER_BLOCK = 281474976710655n;
export const DEFAULT_MAX_HCU_DEPTH_PER_TX = 5000000n;
export const DEFAULT_MAX_HCU_PER_TX = 20000000n;

export const DEFAULT_CHAIN_ID_GATEWAY = 654321n;

export const FHEVM_MNEMONIC = 'test test test test test test test future home engine virtual motion';

export const DEFAULT_COPROCESSORS_MNEMONIC = FHEVM_MNEMONIC;
export const DEFAULT_COPROCESSORS_MNEMONIC_PATH = "m/44'/60'/0'/2/";
export const DEFAULT_COPROCESSORS_MNEMONIC_INDEX = 0;

export const DEFAULT_KMS_NODES_MNEMONIC = FHEVM_MNEMONIC;
export const DEFAULT_KMS_NODES_MNEMONIC_PATH = "m/44'/60'/0'/3/";
export const DEFAULT_KMS_NODES_MNEMONIC_INDEX = 0;

export const DEFAULT_KMS_NODES_TX_SENDER_MNEMONIC = FHEVM_MNEMONIC;
export const DEFAULT_KMS_NODES_TX_SENDER_MNEMONIC_PATH = "m/44'/60'/0'/4/";
export const DEFAULT_KMS_NODES_TX_SENDER_MNEMONIC_INDEX = 0;

export const DEFAULT_COPROCESSOR_THRESHOLD = 4n;

export const DEFAULT_NUM_COPROCESSORS = 4n;
export const DEFAULT_NUM_KMS_NODES = 4n;

export const DEFAULT_KMS_THRESHOLDS: KmsThresholds = {
  publicDecryption: DEFAULT_NUM_KMS_NODES,
  userDecryption: DEFAULT_NUM_KMS_NODES,
  kmsGen: DEFAULT_NUM_KMS_NODES,
  mpc: DEFAULT_NUM_KMS_NODES,
};

/**
 * KMS Core software version recorded on the initial epoch. The cleartext stack runs no real KMS, so
 * this is a well-formed placeholder.
 */
export const DEFAULT_KMS_SOFTWARE_VERSION = '0.0.0-mock';

/** Enclave PCR measurements for the initial epoch. Unused by the cleartext stack. */
export const DEFAULT_PCR_VALUES: readonly PcrValues[] = [];

/**
 * Build the v14 `KmsNodeParams` entry for pool index `j`. v14 extends v13's `KmsNode` with MPC
 * connection metadata (`partyId`/`mpcIdentity`/`caCert`/`storagePrefix`); the cleartext stack never
 * talks to a real KMS, so those are deterministic placeholders derived from the pool index.
 */
function defaultKmsNodeParamsAt(j: number): KmsNodeParams {
  return {
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    txSenderAddress: DEFAULT_KMS_NODE_TX_SENDER_ADDRESSES[j]!,
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    signerAddress: DEFAULT_KMS_NODE_ADDRESSES[j]!,
    ipAddress: `127.0.0.${j + 1}`,
    storageUrl: `s3://kms-bucket-${j + 1}`,
    partyId: j + 1,
    mpcIdentity: `kms-node-${j + 1}`,
    caCert: '0x',
    storagePrefix: '',
  };
}

function generateDefaultKmsNodes(num: number): KmsNodeParams[] {
  if (num > DEFAULT_KMS_NODE_ADDRESSES.length) {
    throw new Error('Too many kms nodes');
  }
  const nodes: KmsNodeParams[] = [];
  for (let i = 0; i < num; ++i) {
    nodes.push(defaultKmsNodeParamsAt(i));
  }

  return nodes;
}

// Module scope — built once, not per call.
const KMS_SIGNER_INDEX = new Map(DEFAULT_KMS_NODE_ADDRESSES.map((a, i) => [a.toLowerCase(), i]));

export function generateFromExistingDefaultKmsNodes(existingSigners: string[]): KmsNodeParams[] {
  if (existingSigners.length > DEFAULT_KMS_NODE_ADDRESSES.length) {
    throw new Error('Too many kms nodes');
  }
  return existingSigners.map((signer) => {
    const j = KMS_SIGNER_INDEX.get(signer.toLowerCase());
    if (j === undefined) {
      throw new Error(`Unknown kms signer: ${signer}`);
    }
    return defaultKmsNodeParamsAt(j);
  });
}

/**
 * Rotate a KMS signer set to the next window of the default pool.
 *
 * The default signer pool is a fixed list of {@link DEFAULT_KMS_NODE_ADDRESSES.length} addresses. A KMS
 * context always uses a consecutive, circularly-wrapping window of it: `currentSigners` must be exactly
 * `[pool[i], pool[(i + 1) % N], …, pool[(i + n - 1) % N]]` for some start `i` and length `n`. This
 * returns the next window of the same length — `[pool[(i + n) % N], …, pool[(i + 2n - 1) % N]]`.
 *
 * @throws if `currentSigners` is empty, longer than the pool, contains an unknown signer, or is not a
 *         consecutive window (wrong order or a gap).
 */
export function nextDefaultKmsSignerWindow(currentSigners: readonly string[]): string[] {
  const poolSize = DEFAULT_KMS_NODE_ADDRESSES.length;
  const n = currentSigners.length;
  if (n === 0) {
    throw new Error('Empty kms signer set');
  }
  if (n > poolSize) {
    throw new Error('Too many kms signers');
  }

  const indices = currentSigners.map((signer) => {
    const index = KMS_SIGNER_INDEX.get(signer.toLowerCase());
    if (index === undefined) {
      throw new Error(`Unknown kms signer: ${signer}`);
    }
    return index;
  });

  const [start] = indices;
  if (start === undefined) {
    throw new Error('Empty kms signer set');
  }
  indices.forEach((index, k) => {
    if (index !== (start + k) % poolSize) {
      throw new Error(`Kms signers are not a consecutive window of the default pool (position ${k})`);
    }
  });

  return Array.from({ length: n }, (_unused, k) => {
    const address = DEFAULT_KMS_NODE_ADDRESSES[(start + n + k) % poolSize];
    if (address === undefined) {
      throw new Error('Unreachable: window index out of pool bounds');
    }
    return address;
  });
}

function generateDefaultCoprocessors(num: number): string[] {
  if (num > DEFAULT_COPROCESSOR_ADDRESSES.length) {
    throw new Error('Too many coprocessors');
  }
  const signers: string[] = [];
  for (let i = 0; i < num; ++i) {
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    signers.push(DEFAULT_COPROCESSOR_ADDRESSES[i]!);
  }

  return signers;
}

export const DEFAULT_BOOTSTRAP_CONFIG_V14: BootstrapConfigV14 = {
  hcuLimit: {
    hcuCapPerBlock: DEFAULT_HCU_CAP_PER_BLOCK,
    maxHCUDepthPerTx: DEFAULT_MAX_HCU_DEPTH_PER_TX,
    maxHCUPerTx: DEFAULT_MAX_HCU_PER_TX,
  },
  inputVerifier: {
    chainIDSource: DEFAULT_CHAIN_ID_GATEWAY,
    initialSigners: generateDefaultCoprocessors(Number(DEFAULT_NUM_COPROCESSORS)),
    initialThreshold: DEFAULT_COPROCESSOR_THRESHOLD,
    verifyingContractSource: DEFAULT_INPUT_VERIFICATION_ADDRESS,
  },
  protocolConfig: {
    initialKmsNodeParams: generateDefaultKmsNodes(Number(DEFAULT_NUM_KMS_NODES)),
    initialThresholds: DEFAULT_KMS_THRESHOLDS,
    softwareVersion: DEFAULT_KMS_SOFTWARE_VERSION,
    pcrValues: DEFAULT_PCR_VALUES,
  },
  kmsVerifier: {
    chainIDSource: DEFAULT_CHAIN_ID_GATEWAY,
    verifyingContractSource: DEFAULT_DECRYPTION_ADDRESS,
  },
};
