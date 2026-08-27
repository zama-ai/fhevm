// Constants every generation shares — the local (anvil) stack's deploy identity: v12 and v13 deploy
// from the same account onto the same ZamaConfig addresses. Decided in sdk/cleartext-config.json, which
// each generation's test/cleartext-config-mirror.test.ts checks these against.

import { dirname } from 'node:path';
import { fileURLToPath } from 'node:url';
import { workspaceTarballsDirAbsPath } from './paths.ts';

/**
 * The one directory every workspace member packs its tarball into. Members only add to it; only the
 * workspace root clears it, so no caller may sweep `*.tgz` here.
 * @example
 * TARBALL_DIR_ABS_PATH; // '/repo/sdk/tarballs'
 */
export const TARBALL_DIR_ABS_PATH = workspaceTarballsDirAbsPath(dirname(fileURLToPath(import.meta.url)));

/**
 * Chain id of the local stack — anvil's default, and the one `ZamaConfig` returns its local branch on.
 * @example
 * LOCAL_CHAIN_ID; // 31337
 */
export const LOCAL_CHAIN_ID = 31337;

/**
 * Mnemonic the local stack is deployed from — deployer, admin accounts and anvil's funded set.
 * Not `FHEVM_MNEMONIC`, which derives the KMS and coprocessor signer pools.
 */
export const MNEMONIC =
  'adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer';

/**
 * Account index of the deployer within {@link MNEMONIC}, at HD path `m/44'/60'/0'/0/5`.
 * Every stack address is `CREATE(deployer, nonce)`, so changing it moves the whole stack.
 */
export const DEPLOYER_ADDRESS_INDEX = 5;

/**
 * The three addresses `library-solidity/config/ZamaConfig.sol` returns from `_getLocalConfig()` on chain
 * id 31337. Not ours to choose: dApps inherit ZamaConfig, so a local deploy must land on them.
 */
export const ZAMA_LOCAL_CONFIG = {
  aclAddress: '0x50157CFfD6bBFA2DECe204a89ec419c23ef5755D',
  // ZamaConfig calls this one `CoprocessorAddress`; the two names describe one contract.
  fhevmExecutorAddress: '0xe3a9105a3a932253A70F126eb1E3b589C643dD24',
  kmsVerifierAddress: '0x901F8942346f7AB3a01F6D7613119Bca447Bb030',
} as const;
