import { PreflightError } from '../errors';

export const SOLANA_PUBLIC_DECRYPT_PROFILE = 'solana-public-decrypt';
export const SOLANA_PUBLIC_DECRYPT_DESCRIPTION =
  'Request one witness-bound Solana public-decrypt certificate through the public SDK.';

type Environment = Readonly<Record<string, string | undefined>>;
type PublicDecryptRequest = {
  handle: string;
  contextId: Uint8Array;
  aclValueKey: Uint8Array;
  proofSlot: bigint;
  encryptedValueAccount: Uint8Array;
  peaks: readonly Uint8Array[];
  leafCount: bigint;
  mmrProofBytes: Uint8Array;
};
type PublicDecryptClaim = {
  handle: string;
  abiEncodedCleartext: string;
  signatures: readonly string[];
  extraData: string;
};
type PublicDecryptSdkInput = {
  chainId: bigint;
  relayerUrl: string;
  apiKey: string;
  request: PublicDecryptRequest;
};
type PublicDecryptSdkCall = (input: PublicDecryptSdkInput) => Promise<PublicDecryptClaim>;

export type PublicDecryptDependencies = { publicDecryptCertificate?: PublicDecryptSdkCall };

const required = (environment: Environment, name: string): string => {
  const value = environment[name];
  if (value === undefined || value === '') throw new PreflightError(`missing env ${name}`);
  return value;
};

const bytes = (value: string, name: string): Uint8Array => {
  const hex = value.startsWith('0x') ? value.slice(2) : value;
  if (hex.length % 2 !== 0 || !/^[0-9a-f]*$/i.test(hex)) {
    throw new PreflightError(`${name} must be an even-length hex value`);
  }
  return Uint8Array.from(Buffer.from(hex, 'hex'));
};

const bytes32 = (environment: Environment, name: string): Uint8Array => {
  const value = bytes(required(environment, name), name);
  if (value.length !== 32) throw new PreflightError(`${name} must be 32 bytes`);
  return value;
};

const bytes32Hex = (environment: Environment, name: string): string => {
  const value = required(environment, name);
  if (!/^0x[0-9a-f]{64}$/i.test(value)) throw new PreflightError(`${name} must be a 0x-prefixed 32-byte hex value`);
  return value;
};

const hexCsv = (environment: Environment, name: string): readonly Uint8Array[] =>
  required(environment, name)
    .split(',')
    .map((value) => value.trim())
    .filter(Boolean)
    .map((value) => {
      const decoded = bytes(value, name);
      if (decoded.length !== 32) throw new PreflightError(`${name} entries must be 32 bytes`);
      return decoded;
    });

// Keep the dynamic import seam narrow: clean CLI checkouts do not contain the SDK's generated
// `_types`, while the full vertical exercises this public package entry at runtime.
const runPublicSdkPublicDecrypt: PublicDecryptSdkCall = async (input) => {
  const solanaModule = '@fhevm/sdk/solana';
  const solana = await import(solanaModule);
  const chain = solana.defineFhevmSolanaChain({
    id: input.chainId,
    fhevm: { relayerUrl: input.relayerUrl, acl: { domainKeys: [] } },
  });
  solana.setFhevmRuntimeConfig({ auth: { type: 'ApiKeyHeader', value: input.apiKey } });
  return solana.createFhevmPublicDecryptClient({ chain }).publicDecryptCertificate(input.request);
};

/** Runs the public-decrypt SDK action and prints the legacy JSON envelope used by consume steps. */
export const runSolanaPublicDecrypt = async (
  environment: Environment = process.env,
  dependencies: PublicDecryptDependencies = {},
): Promise<PublicDecryptClaim> => {
  const request: PublicDecryptRequest = {
    handle: bytes32Hex(environment, 'PD_HANDLE'),
    contextId: bytes32(environment, 'PD_CONTEXT_ID'),
    aclValueKey: bytes32(environment, 'PD_ACL_VALUE_KEY'),
    proofSlot: BigInt(required(environment, 'PD_MMR_PROOF_SLOT')),
    encryptedValueAccount: bytes32(environment, 'PD_MMR_ENCRYPTED_VALUE_ACCOUNT'),
    peaks: hexCsv(environment, 'PD_MMR_PEAKS'),
    leafCount: BigInt(required(environment, 'PD_MMR_LEAF_COUNT')),
    mmrProofBytes: bytes(required(environment, 'PD_MMR_PROOF_BYTES'), 'PD_MMR_PROOF_BYTES'),
  };
  const call = dependencies.publicDecryptCertificate ?? runPublicSdkPublicDecrypt;
  const claim = await call({
    chainId: BigInt(required(environment, 'PD_CONTRACTS_CHAIN_ID')),
    relayerUrl: required(environment, 'PD_RELAYER_URL'),
    apiKey: environment.ZAMA_FHEVM_API_KEY ?? 'local',
    request,
  });
  process.stdout.write(
    `${JSON.stringify({
      status: 'succeeded',
      result: {
        decryptedValue: claim.abiEncodedCleartext,
        signatures: claim.signatures,
        extraData: claim.extraData,
      },
    })}\n`,
  );
  return claim;
};
