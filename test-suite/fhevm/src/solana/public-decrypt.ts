import type { SolanaPublicDecryptCertificateClaim } from '@sdk-src/solana/actions/publicDecryptCertificate.js';

import { PreflightError } from '../errors';

export const SOLANA_PUBLIC_DECRYPT_PROFILE = 'solana-public-decrypt';
export const SOLANA_PUBLIC_DECRYPT_DESCRIPTION =
  'Request one Solana public-decrypt certificate through the public SDK.';

type Environment = Readonly<Record<string, string | undefined>>;
/** `(handle, account)` is all a public decrypt names: the Connector reads the account and proves the public leaf. */
type PublicDecryptRequest = {
  handle: string;
  contextId: Uint8Array;
  encryptedValueAccount: Uint8Array;
};
/**
 * The KMS public-decrypt certificate the SDK action returns — the glossary term is "certificate"
 * (the SDK's type name keeps its historical "claim" suffix). Type-only import: the SDK workspace
 * need not be materialized to run the offline suites.
 */
export type PublicDecryptCertificate = SolanaPublicDecryptCertificateClaim;
/**
 * Interprets the certificate cleartext as a number. `abiEncodedCleartext` is UNPREFIXED ABI hex
 * (a 32-byte big-endian uint256), so it must be parsed as hex explicitly — `BigInt(...)` on the
 * raw string reads all-digit hex like "46" as decimal 46 instead of 0x46 = 70.
 */
export const certificateCleartext = (certificate: Pick<PublicDecryptCertificate, 'abiEncodedCleartext'>): bigint =>
  BigInt(`0x${certificate.abiEncodedCleartext.replace(/^0x/, '')}`);

type PublicDecryptSdkInput = {
  chainId: bigint;
  relayerUrl: string;
  apiKey: string;
  request: PublicDecryptRequest;
};
type PublicDecryptSdkCall = (input: PublicDecryptSdkInput) => Promise<PublicDecryptCertificate>;

export type PublicDecryptDependencies = { publicDecryptCertificate?: PublicDecryptSdkCall };

const required = (environment: Environment, name: string): string => {
  const value = environment[name];
  if (value === undefined || value === '') throw new PreflightError(`missing env ${name}`);
  return value;
};

const bytes32 = (environment: Environment, name: string): Uint8Array => {
  const hex = bytes32Hex(environment, name).slice(2);
  return Uint8Array.from(Buffer.from(hex, 'hex'));
};

const bytes32Hex = (environment: Environment, name: string): string => {
  const value = required(environment, name);
  if (!/^0x[0-9a-f]{64}$/i.test(value)) throw new PreflightError(`${name} must be a 0x-prefixed 32-byte hex value`);
  return value;
};

// Keep the dynamic import seam narrow: clean CLI checkouts do not contain the SDK's generated
// `_types`, while the full vertical exercises this public package entry at runtime.
const runPublicSdkPublicDecrypt: PublicDecryptSdkCall = async (input) => {
  const solanaModule = '@fhevm/sdk/solana';
  const solana = await import(solanaModule);
  const chain = solana.defineFhevmSolanaChain({ id: input.chainId, fhevm: { relayerUrl: input.relayerUrl } });
  solana.setFhevmRuntimeConfig({ auth: { type: 'ApiKeyHeader', value: input.apiKey } });
  return solana.createFhevmPublicDecryptClient({ chain }).publicDecryptCertificate(input.request);
};

/** Runs the public-decrypt SDK action and prints the legacy JSON envelope used by consume steps. */
export const runSolanaPublicDecrypt = async (
  environment: Environment = process.env,
  dependencies: PublicDecryptDependencies = {},
): Promise<PublicDecryptCertificate> => {
  const request: PublicDecryptRequest = {
    handle: bytes32Hex(environment, 'PD_HANDLE'),
    contextId: bytes32(environment, 'PD_CONTEXT_ID'),
    encryptedValueAccount: bytes32(environment, 'PD_ENCRYPTED_VALUE_ACCOUNT'),
  };
  const call = dependencies.publicDecryptCertificate ?? runPublicSdkPublicDecrypt;
  const certificate = await call({
    chainId: BigInt(required(environment, 'PD_CONTRACTS_CHAIN_ID')),
    relayerUrl: required(environment, 'PD_RELAYER_URL'),
    apiKey: environment.ZAMA_FHEVM_API_KEY ?? 'local',
    request,
  });
  process.stdout.write(
    `${JSON.stringify({
      status: 'succeeded',
      result: {
        decryptedValue: certificate.abiEncodedCleartext,
        signatures: certificate.signatures,
        extraData: certificate.extraData,
      },
    })}\n`,
  );
  return certificate;
};
