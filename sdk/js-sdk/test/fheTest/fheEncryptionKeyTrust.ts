import type { FheEncryptionKeyDigests, FheEncryptionKeyMetadata, FheEncryptionKeyTrust } from '@fhevm/sdk/types';
import type { FhevmEncryptOptions, FhevmOptions } from '../../src/core/types/coreFhevmClient.js';
import type { FhevmChain } from '../../src/core/types/fhevmChain.js';
import type { FhevmModuleVersions } from '../../src/core/types/moduleVersions.js';
import { mainnet, sepolia } from '../../src/core/chains/index.js';
import { computeFheEncryptionKeyDigests } from '../../src/core/key/authenticateFheEncryptionKeyBytes.js';
import { devnet } from '../chains/devnet.js';
import { localcleartext } from '../chains/localcleartext.js';
import { localstack } from '../chains/localstack.js';
import { localstack_v11 } from '../chains/localstack_v11.js';
import { localstack_v12 } from '../chains/localstack_v12.js';
import { localstack_v13 } from '../chains/localstack_v13.js';
import { localstack_v14 } from '../chains/localstack_v14.js';
import { polygon_devnet } from '../chains/polygon_devnet.js';

const DIGEST_PATTERN = /^0x[0-9a-fA-F]{64}$/;
const LOCALSTACK_FHE_ENCRYPTION_KEY_ID = '0400000000000000000000000000000000000000000000000000000000000001';
const LOCALSTACK_FHE_ENCRYPTION_CRS_ID = '0500000000000000000000000000000000000000000000000000000000000001';
const LOCALSTACK_KMS_PUBLIC_VAULT_URL = 'http://localhost:9000/kms-public';
const LOCALSTACK_KMS_PUBLIC_VAULT_PREFIXES = ['PUB/PUB', 'PUB'] as const;

const FHE_TEST_CHAINS: Readonly<Record<string, FhevmChain>> = {
  devnet,
  localcleartext,
  localcleartext_v12: localcleartext,
  localcleartext_v13: localcleartext,
  localstack,
  localstack_v11,
  localstack_v12,
  localstack_v13,
  localstack_v14,
  mainnet,
  polygon_devnet,
  sepolia,
  testnet: sepolia,
};

type FheEncryptionKeyTrustResolver = Exclude<FheEncryptionKeyTrust, FheEncryptionKeyDigests>;

export function usesConfiguredKmsGeneration(chainName: string): boolean {
  return FHE_TEST_CHAINS[chainName]?.fhevm.contracts.kmsGeneration !== undefined;
}

export function isCleartextFheTestChain(chainName: string): boolean {
  return chainName === 'localcleartext' || chainName.startsWith('localcleartext_') || chainName.endsWith('_cleartext');
}

export function parseFheEncryptionKeyDigests(
  chainEnv: Readonly<Record<string, string | undefined>>,
  sharedEnv: Readonly<Record<string, string | undefined>>,
  processEnv: Readonly<Record<string, string | undefined>> = process.env,
): FheEncryptionKeyDigests | undefined {
  const publicKeyDigest =
    chainEnv.FHEVM_PUBLIC_KEY_DIGEST ?? sharedEnv.FHEVM_PUBLIC_KEY_DIGEST ?? processEnv.FHEVM_PUBLIC_KEY_DIGEST;
  const crsDigest = chainEnv.FHEVM_CRS_DIGEST ?? sharedEnv.FHEVM_CRS_DIGEST ?? processEnv.FHEVM_CRS_DIGEST;
  if (publicKeyDigest === undefined && crsDigest === undefined) {
    return undefined;
  }
  if (!DIGEST_PATTERN.test(publicKeyDigest ?? '') || !DIGEST_PATTERN.test(crsDigest ?? '')) {
    throw new Error('FHEVM_PUBLIC_KEY_DIGEST and FHEVM_CRS_DIGEST must both be 0x-prefixed 32-byte digests.');
  }
  return {
    publicKeyDigest: publicKeyDigest as FheEncryptionKeyDigests['publicKeyDigest'],
    crsDigest: crsDigest as FheEncryptionKeyDigests['crsDigest'],
  };
}

export function resolveFheEncryptionKeyTrust(
  chainName: string,
  chainEnv: Readonly<Record<string, string | undefined>>,
  sharedEnv: Readonly<Record<string, string | undefined>>,
): FheEncryptionKeyTrust | undefined {
  if (isCleartextFheTestChain(chainName) || usesConfiguredKmsGeneration(chainName)) {
    return undefined;
  }

  const configured = parseFheEncryptionKeyDigests(chainEnv, sharedEnv);
  if (configured !== undefined) {
    return configured;
  }
  return chainName.startsWith('localstack') ? createLocalstackFheEncryptionKeyTrust() : undefined;
}

export function createLocalstackFheEncryptionKeyTrust(
  fetcher: typeof globalThis.fetch = globalThis.fetch,
): FheEncryptionKeyTrustResolver {
  let digestsPromise: Promise<FheEncryptionKeyDigests> | undefined;

  return (metadata) => {
    if (metadata.chainId !== localstack.id) {
      return Promise.reject(
        new Error(
          `Localstack FHE encryption-key trust expected chain ${localstack.id.toString()}, received ${metadata.chainId.toString()}.`,
        ),
      );
    }

    if (digestsPromise === undefined) {
      const attempt = loadLocalstackFheEncryptionKeyDigests(fetcher, metadata);
      digestsPromise = attempt;
      void attempt.catch(() => {
        if (digestsPromise === attempt) {
          digestsPromise = undefined;
        }
      });
    }
    return digestsPromise;
  };
}

export function createFheTestClientOptions(parameters: {
  readonly chainName: string;
  readonly fheEncryptionKeyTrust?: FheEncryptionKeyTrust | undefined;
  readonly moduleVersions?: FhevmModuleVersions | undefined;
}): FhevmOptions | undefined {
  const fheEncryptionKeyTrust = usesConfiguredKmsGeneration(parameters.chainName)
    ? undefined
    : parameters.fheEncryptionKeyTrust;
  if (parameters.moduleVersions === undefined) {
    return fheEncryptionKeyTrust === undefined ? undefined : { fheEncryptionKeyTrust };
  }
  return fheEncryptionKeyTrust === undefined
    ? { moduleVersions: parameters.moduleVersions }
    : { moduleVersions: parameters.moduleVersions, fheEncryptionKeyTrust };
}

export function createFheTestEncryptClientOptions(parameters: {
  readonly chainName: string;
  readonly fheEncryptionKeyTrust?: FheEncryptionKeyTrust | undefined;
  readonly moduleVersions?: FhevmModuleVersions | undefined;
}): FhevmEncryptOptions | undefined {
  const fheEncryptionKeyTrust = optionalFheEncryptionKeyTrust(parameters);
  const moduleVersions = parameters.moduleVersions;
  if (moduleVersions === undefined) {
    return fheEncryptionKeyTrust === undefined ? undefined : { fheEncryptionKeyTrust };
  }
  if (moduleVersions === 'auto') {
    return fheEncryptionKeyTrust === undefined ? { moduleVersions } : { moduleVersions, fheEncryptionKeyTrust };
  }
  if (moduleVersions.tfhe === undefined && moduleVersions.checkCompatibility === undefined) {
    return fheEncryptionKeyTrust === undefined ? undefined : { fheEncryptionKeyTrust };
  }
  const encryptModuleVersions = {
    tfhe: moduleVersions.tfhe,
    checkCompatibility: moduleVersions.checkCompatibility,
  };
  return fheEncryptionKeyTrust === undefined
    ? { moduleVersions: encryptModuleVersions }
    : {
        fheEncryptionKeyTrust,
        moduleVersions: encryptModuleVersions,
      };
}

function optionalFheEncryptionKeyTrust(parameters: {
  readonly chainName: string;
  readonly fheEncryptionKeyTrust?: FheEncryptionKeyTrust | undefined;
}): FheEncryptionKeyTrust | undefined {
  if (isCleartextFheTestChain(parameters.chainName) || usesConfiguredKmsGeneration(parameters.chainName)) {
    return undefined;
  }
  return parameters.fheEncryptionKeyTrust;
}

async function loadLocalstackFheEncryptionKeyDigests(
  fetcher: typeof globalThis.fetch,
  metadata: FheEncryptionKeyMetadata,
): Promise<FheEncryptionKeyDigests> {
  const failures: string[] = [];
  for (const prefix of LOCALSTACK_KMS_PUBLIC_VAULT_PREFIXES) {
    try {
      const [publicKeyBytes, crsBytes] = await Promise.all([
        fetchTrustedLocalstackBytes(
          fetcher,
          `${LOCALSTACK_KMS_PUBLIC_VAULT_URL}/${prefix}/PublicKey/${LOCALSTACK_FHE_ENCRYPTION_KEY_ID}`,
          'public key',
        ),
        fetchTrustedLocalstackBytes(
          fetcher,
          `${LOCALSTACK_KMS_PUBLIC_VAULT_URL}/${prefix}/CRS/${LOCALSTACK_FHE_ENCRYPTION_CRS_ID}`,
          'CRS',
        ),
      ]);

      return computeFheEncryptionKeyDigests({
        publicKeyBytes: { id: LOCALSTACK_FHE_ENCRYPTION_KEY_ID, bytes: publicKeyBytes },
        crsBytes: { id: LOCALSTACK_FHE_ENCRYPTION_CRS_ID, capacity: 2048, bytes: crsBytes },
        metadata,
      });
    } catch (error) {
      failures.push(error instanceof Error ? error.message : String(error));
    }
  }
  throw new Error(`Failed to read trusted localstack FHE encryption material: ${failures.join(' ')}`);
}

async function fetchTrustedLocalstackBytes(
  fetcher: typeof globalThis.fetch,
  url: string,
  material: string,
): Promise<Uint8Array> {
  const response = await fetcher(url);
  if (!response.ok) {
    throw new Error(
      `Failed to read the trusted localstack FHE encryption ${material} from ${url}: HTTP ${response.status.toString()}.`,
    );
  }

  const bytes = new Uint8Array(await response.arrayBuffer());
  if (bytes.byteLength === 0) {
    throw new Error(`The trusted localstack FHE encryption ${material} at ${url} is empty.`);
  }
  return bytes;
}
