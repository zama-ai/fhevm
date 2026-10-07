import { DEFAULT_COPROCESSOR_ADDRESSES } from './signers/defaultCoprocessorSigners.js';
import { DEFAULT_KMS_NODE_ADDRESSES } from './signers/defaultKmsSigners.js';
import type { BootstrapConfig } from './types/public.js';
// Every scalar the cleartext stack is configured with comes from here — the generated TypeScript face of
// sdk/cleartext-config.json, synced from common-vendored/src. The values are deliberately NOT re-exported
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
  CLEARTEXT_MAX_HCU_DEPTH_PER_TX,
  CLEARTEXT_MAX_HCU_PER_TX,
} from './cleartext-config.js';

function generateDefaultKmsSigners(num: number): string[] {
  if (num > DEFAULT_KMS_NODE_ADDRESSES.length) {
    throw new Error('Too many kms nodes');
  }
  // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
  return Array.from({ length: num }, (_unused, i) => DEFAULT_KMS_NODE_ADDRESSES[i]!);
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

export const DEFAULT_BOOTSTRAP_CONFIG: BootstrapConfig = {
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
  kmsVerifier: {
    chainIDSource: CLEARTEXT_GATEWAY_CHAIN_ID,
    verifyingContractSource: CLEARTEXT_DECRYPTION_ADDRESS,
    // This generation's KMSVerifier holds its own signer set + threshold; 0.13 moves them to
    // ProtocolConfig. The signers are the KMS *node* signers either way, so the same pool is used.
    initialSigners: generateDefaultKmsSigners(CLEARTEXT_KMS_NODE_COUNT),
    initialThreshold: BigInt(CLEARTEXT_KMS_NODE_COUNT),
  },
};
