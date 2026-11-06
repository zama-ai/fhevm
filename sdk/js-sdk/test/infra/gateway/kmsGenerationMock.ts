/**
 * KMSGeneration key reads answered by the gateway's RPC proxy.
 *
 * The cleartext stack deploys a real KMSGeneration, but no key generation ever
 * runs on it, so it has no active key and the SDK (which checks every FHE public
 * key and CRS against KMSGeneration before using it) would refuse the slot's key.
 * The gateway answers the four reads that check a key with the slot key's
 * digests (`test/keys/key.<ver>.digests.json`), so the SDK still checks the key
 * bytes for real. Every other call, including `getVersion()`, reaches anvil.
 */

import { readFileSync } from 'node:fs';
import { decodeFunctionData, encodeFunctionResult, parseAbi, type Hex } from 'viem';

export type KmsGenerationMockConfig = {
  /** The KMSGeneration address on the slot's chain. */
  readonly address: string;
  /** Path to the `test/keys/key.<ver>.digests.json` file of the slot's key. */
  readonly digestsFilePath: string;
};

/** Any non-zero id: the test keys carry label ids, which the SDK does not match against. */
const ACTIVE_KEY_ID = 1n;
const ACTIVE_CRS_ID = 1n;
/** `KeyType.Public` in KMSGeneration's key digests. */
const KEY_TYPE_PUBLIC = 1;

const KMS_GENERATION_ABI = parseAbi([
  'function getActiveKeyId() view returns (uint256)',
  'function getActiveCrsId() view returns (uint256)',
  'function getKeyMaterials(uint256 keyId) view returns (string[], (uint8 keyType, bytes digest)[])',
  'function getCrsMaterials(uint256 crsId) view returns (string[], bytes)',
]);

type Digests = { readonly publicKeyDigest: Hex; readonly crsDigest: Hex };

const digestsCache = new Map<string, Digests>();

function _loadDigests(path: string): Digests {
  let digests = digestsCache.get(path);
  if (digests === undefined) {
    digests = JSON.parse(readFileSync(path, 'utf-8')) as Digests;
    digestsCache.set(path, digests);
  }
  return digests;
}

type JsonRpcRequest = {
  readonly id?: unknown;
  readonly method?: unknown;
  readonly params?: unknown;
};

/**
 * The JSON-RPC response to `request` when it is a KMSGeneration key read,
 * `undefined` when it must go to anvil.
 */
export function answerKmsGenerationCall(config: KmsGenerationMockConfig, request: unknown): unknown {
  const r = request as JsonRpcRequest;
  if (typeof r !== 'object' || r === null || r.method !== 'eth_call' || !Array.isArray(r.params)) {
    return undefined;
  }
  const tx = r.params[0] as { readonly to?: unknown; readonly data?: unknown; readonly input?: unknown } | undefined;
  const data = tx?.data ?? tx?.input;
  if (typeof tx?.to !== 'string' || tx.to.toLowerCase() !== config.address.toLowerCase() || typeof data !== 'string') {
    return undefined;
  }

  let call;
  try {
    call = decodeFunctionData({ abi: KMS_GENERATION_ABI, data: data as Hex });
  } catch {
    return undefined; // getVersion() and any other call: the deployed contract answers.
  }

  const digests = _loadDigests(config.digestsFilePath);
  let result: Hex;
  switch (call.functionName) {
    case 'getActiveKeyId':
      result = encodeFunctionResult({
        abi: KMS_GENERATION_ABI,
        functionName: call.functionName,
        result: ACTIVE_KEY_ID,
      });
      break;
    case 'getActiveCrsId':
      result = encodeFunctionResult({
        abi: KMS_GENERATION_ABI,
        functionName: call.functionName,
        result: ACTIVE_CRS_ID,
      });
      break;
    case 'getKeyMaterials':
      if (call.args[0] !== ACTIVE_KEY_ID) {
        return undefined;
      }
      result = encodeFunctionResult({
        abi: KMS_GENERATION_ABI,
        functionName: call.functionName,
        result: [[], [{ keyType: KEY_TYPE_PUBLIC, digest: digests.publicKeyDigest }]],
      });
      break;
    case 'getCrsMaterials':
      if (call.args[0] !== ACTIVE_CRS_ID) {
        return undefined;
      }
      result = encodeFunctionResult({
        abi: KMS_GENERATION_ABI,
        functionName: call.functionName,
        result: [[], digests.crsDigest],
      });
      break;
  }
  return { jsonrpc: '2.0', id: r.id ?? null, result };
}
