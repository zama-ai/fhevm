import { DEFAULT_COPROCESSOR_ADDRESSES } from './signers/defaultCoprocessorSigners.js';
import { DEFAULT_KMS_NODE_ADDRESSES } from './signers/defaultKmsSigners.js';
import { DEFAULT_KMS_NODE_TX_SENDER_ADDRESSES } from './signers/defaultKmsTxSenderSigners.js';
import type { BootstrapConfigV13, KmsNode, KmsThresholds } from './types/public.js';
// Every scalar the cleartext stack is configured with comes from here, and this module is a byte-for-byte
// copy of internal/cleartext-config.ts — see its header. The values are deliberately NOT re-exported
// under `DEFAULT_*` aliases: an alias is a second name for one value, which is how the two copies of this
// config drifted in the first place.
import {
  CLEARTEXT_COPROCESSOR_COUNT,
  CLEARTEXT_COPROCESSOR_THRESHOLD,
  CLEARTEXT_DECRYPTION_ADDRESS,
  CLEARTEXT_GATEWAY_CHAIN_ID,
  CLEARTEXT_HCU_CAP_PER_BLOCK,
  CLEARTEXT_INPUT_VERIFICATION_ADDRESS,
  CLEARTEXT_KMS_NODE_COUNT,
  CLEARTEXT_KMS_NODE_IP_ADDRESS_PREFIX,
  CLEARTEXT_KMS_NODE_STORAGE_URL_PREFIX,
  CLEARTEXT_MAX_HCU_DEPTH_PER_TX,
  CLEARTEXT_MAX_HCU_PER_TX,
} from './cleartext-config.js';

/**
 * The four KMS thresholds, each defaulting to the node count.
 *
 * `CLEARTEXT_KMS_NODE_COUNT` is a plain number — it is a count, and the harness renders it into Solidity
 * as one — whereas the on-chain struct takes `uint256`, so it is widened here rather than stored twice in
 * two types.
 */
export const DEFAULT_KMS_THRESHOLDS: KmsThresholds = {
  publicDecryption: BigInt(CLEARTEXT_KMS_NODE_COUNT),
  userDecryption: BigInt(CLEARTEXT_KMS_NODE_COUNT),
  kmsGen: BigInt(CLEARTEXT_KMS_NODE_COUNT),
  mpc: BigInt(CLEARTEXT_KMS_NODE_COUNT),
};

function generateDefaultKmsNodes(num: number): KmsNode[] {
  if (num > DEFAULT_KMS_NODE_ADDRESSES.length) {
    throw new Error('Too many kms nodes');
  }
  const nodes: KmsNode[] = [];
  for (let i = 0; i < num; ++i) {
    const n: KmsNode = {
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      txSenderAddress: DEFAULT_KMS_NODE_TX_SENDER_ADDRESSES[i]!,
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      signerAddress: DEFAULT_KMS_NODE_ADDRESSES[i]!,
      ipAddress: `${CLEARTEXT_KMS_NODE_IP_ADDRESS_PREFIX}${i + 1}`,
      storageUrl: `${CLEARTEXT_KMS_NODE_STORAGE_URL_PREFIX}${i + 1}`,
    };
    nodes.push(n);
  }

  return nodes;
}

// Module scope — built once, not per call.
const KMS_SIGNER_INDEX = new Map(DEFAULT_KMS_NODE_ADDRESSES.map((a, i) => [a.toLowerCase(), i]));

export function generateFromExistingDefaultKmsNodes(existingSigners: string[]): KmsNode[] {
  if (existingSigners.length > DEFAULT_KMS_NODE_ADDRESSES.length) {
    throw new Error('Too many kms nodes');
  }
  return existingSigners.map((signer) => {
    const j = KMS_SIGNER_INDEX.get(signer.toLowerCase());
    if (j === undefined) {
      throw new Error(`Unknown kms signer: ${signer}`);
    }
    return {
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      txSenderAddress: DEFAULT_KMS_NODE_TX_SENDER_ADDRESSES[j]!,
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      signerAddress: DEFAULT_KMS_NODE_ADDRESSES[j]!,
      ipAddress: `${CLEARTEXT_KMS_NODE_IP_ADDRESS_PREFIX}${j + 1}`,
      storageUrl: `${CLEARTEXT_KMS_NODE_STORAGE_URL_PREFIX}${j + 1}`,
    };
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

export const DEFAUT_BOOTSTRAP_CONFIG_V13: BootstrapConfigV13 = {
  hcuLimit: {
    hcuCapPerBlock: CLEARTEXT_HCU_CAP_PER_BLOCK,
    maxHCUDepthPerTx: CLEARTEXT_MAX_HCU_DEPTH_PER_TX,
    maxHCUPerTx: CLEARTEXT_MAX_HCU_PER_TX,
  },
  inputVerifier: {
    chainIDSource: CLEARTEXT_GATEWAY_CHAIN_ID,
    initialSigners: generateDefaultCoprocessors(CLEARTEXT_COPROCESSOR_COUNT),
    initialThreshold: BigInt(CLEARTEXT_COPROCESSOR_THRESHOLD),
    verifyingContractSource: CLEARTEXT_INPUT_VERIFICATION_ADDRESS,
  },
  protocolConfig: {
    initialKmsNodes: generateDefaultKmsNodes(CLEARTEXT_KMS_NODE_COUNT),
    initialThresholds: DEFAULT_KMS_THRESHOLDS,
  },
  kmsVerifier: {
    chainIDSource: CLEARTEXT_GATEWAY_CHAIN_ID,
    verifyingContractSource: CLEARTEXT_DECRYPTION_ADDRESS,
  },
};
