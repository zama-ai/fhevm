// Spike S4 (phase-1b.md §13): a minimal client of the fhEVM SDK, shared by the scenario steps
// and the exit probe. The FHE module replaces it once the spike has a verdict.
import { defineFhevmChain } from '@fhevm/sdk/chains';
import { createFhevmClient, hasFhevmRuntimeConfig, setFhevmRuntimeConfig } from '@fhevm/sdk/ethers';
import { type Provider, type Signer, getBytes } from 'ethers';

type Address = `0x${string}`;

/** `true` for the values a profile setting or variable uses to mean yes. */
export const isYes = (value: unknown): boolean => ['1', 'true', 'yes'].includes(String(value).toLowerCase());

export interface FheSettings {
  relayerUrl: string;
  chainId: number;
  gatewayChainId: number;
  decryption: Address;
  inputVerification: Address;
  acl: Address;
  inputVerifier: Address;
  kmsVerifier: Address;
  protocolConfig: Address;
}

export type FheClient = ReturnType<typeof createFhevmClient>;

/** The SDK takes the relayer base URL; `test-suite.env` gives it with the API version. */
const relayerBaseUrl = (url: string): string => url.replace(/\/v[12]\/?$/, '');

/**
 * Creates an SDK client for the host chain of `provider` and waits until it is ready.
 * `singleThread` runs the SDK's WASM on the main thread; the e2e suite uses its thread pool.
 * The SDK runtime is configured once per process, by the first client.
 */
export async function createFheClient(
  settings: FheSettings,
  provider: Provider,
  singleThread: boolean,
): Promise<FheClient> {
  if (!hasFhevmRuntimeConfig()) setFhevmRuntimeConfig({ singleThread });
  const chain = defineFhevmChain({
    id: settings.chainId,
    fhevm: {
      contracts: {
        acl: { address: settings.acl },
        inputVerifier: { address: settings.inputVerifier },
        kmsVerifier: { address: settings.kmsVerifier },
        protocolConfig: { address: settings.protocolConfig },
      },
      relayerUrl: relayerBaseUrl(settings.relayerUrl),
      gateway: {
        id: settings.gatewayChainId,
        contracts: {
          decryption: { address: settings.decryption },
          inputVerification: { address: settings.inputVerification },
        },
      },
    },
  });
  const client = createFhevmClient({ provider: provider as never, chain });
  await client.ready;
  return client;
}

/** Encrypts one uint64 for `contractAddress`, usable by `userAddress`, with its input proof. */
export async function encryptUint64(
  client: FheClient,
  value: bigint,
  contractAddress: string,
  userAddress: string,
): Promise<{ handle: Uint8Array; inputProof: Uint8Array }> {
  const result = await client.encryptValue({
    contractAddress: contractAddress as Address,
    userAddress: userAddress as Address,
    value: { type: 'uint64', value },
  });
  return { handle: getBytes(result.encryptedValue), inputProof: getBytes(result.inputProof) };
}

/** User decryption of one handle through the legacy permit, as the e2e helper does. */
export async function userDecrypt(
  client: FheClient,
  handle: string,
  contractAddress: string,
  signer: Signer,
): Promise<unknown> {
  const transportKeyPair = await client.generateTransportKeyPair();
  const signerAddress = (await signer.getAddress()) as Address;
  const signedPermit = await client.signLegacyDecryptionPermit({
    contractAddresses: [contractAddress as Address],
    durationSeconds: 24 * 3600,
    startTimestamp: Math.floor(Date.now() / 1000),
    transportKeyPair,
    signer: signer as never,
    signerAddress,
  });
  const result = await client.decryptValue({
    contractAddress: contractAddress as Address,
    transportKeyPair,
    signedPermit,
    encryptedValue: handle as Address,
  });
  return result.value;
}

/** Public decryption of one handle (it must have been made publicly decryptable). */
export async function publicDecrypt(client: FheClient, handle: string): Promise<unknown> {
  const result = await client.decryptPublicValuesWithSignatures({ encryptedValues: [handle as Address] });
  return result.clearValues[0]?.value;
}

/** Resource types alive now (`process.getActiveResourcesInfo`), counted by type. */
export function activeResources(): Record<string, number> {
  const counts: Record<string, number> = {};
  for (const type of process.getActiveResourcesInfo()) counts[type] = (counts[type] ?? 0) + 1;
  return counts;
}

/** What `after` has more of than `before`. */
export function resourcesAdded(before: Record<string, number>, after: Record<string, number>): Record<string, number> {
  const added: Record<string, number> = {};
  for (const [type, count] of Object.entries(after)) {
    const extra = count - (before[type] ?? 0);
    if (extra > 0) added[type] = extra;
  }
  return added;
}
