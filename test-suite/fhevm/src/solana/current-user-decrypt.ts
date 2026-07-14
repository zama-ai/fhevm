import { PreflightError } from "../errors";

export const SOLANA_CURRENT_USER_DECRYPT_PROFILE = "solana-current-user-decrypt";
export const SOLANA_CURRENT_USER_DECRYPT_DESCRIPTION =
  "Decrypt one current Solana handle through the public SDK and assert its plaintext.";

type Environment = Readonly<Record<string, string | undefined>>;
type CurrentUserDecryptRequest = {
  handles: readonly string[];
  allowedAclDomainKeys: readonly string[];
  contextId: Uint8Array;
  aclValueKey: Uint8Array;
  nonce?: Uint8Array;
  validity?: { startTimestamp: bigint; durationSeconds: bigint };
};
type CurrentUserDecryptSdkInput = {
  chainId: bigint;
  relayerUrl: string;
  apiKey: string;
  secretKey: Uint8Array;
  request: CurrentUserDecryptRequest;
};
type CurrentUserDecryptSdkCall = (
  input: CurrentUserDecryptSdkInput,
) => Promise<readonly { value: bigint | number | boolean | string }[]>;

export type CurrentUserDecryptDependencies = {
  userDecrypt?: CurrentUserDecryptSdkCall;
};

const required = (environment: Environment, name: string): string => {
  const value = environment[name];
  if (value === undefined || value === "") {
    throw new PreflightError(`missing env ${name}`);
  }
  return value;
};

const bytes = (value: string, name: string): Uint8Array => {
  const hex = value.startsWith("0x") ? value.slice(2) : value;
  if (hex.length % 2 !== 0 || !/^[0-9a-f]*$/i.test(hex)) {
    throw new PreflightError(`${name} must be an even-length hex string`);
  }
  return Uint8Array.from(Buffer.from(hex, "hex"));
};

const bytes32 = (environment: Environment, name: string): Uint8Array => {
  const value = bytes(required(environment, name), name);
  if (value.length !== 32) {
    throw new PreflightError(`${name} must be 32 bytes`);
  }
  return value;
};

const bytes32Hex = (value: string, name: string): string => {
  if (!/^0x[0-9a-f]{64}$/i.test(value)) {
    throw new PreflightError(`${name} must be a 0x-prefixed 32-byte hex value`);
  }
  return value;
};

const optionalValidity = (environment: Environment): CurrentUserDecryptRequest["validity"] => {
  const startTimestamp = environment.UD_START_TIMESTAMP;
  const durationSeconds = environment.UD_DURATION_SECONDS;
  if (startTimestamp === undefined && durationSeconds === undefined) {
    return undefined;
  }
  if (!startTimestamp || !durationSeconds) {
    throw new PreflightError("UD_START_TIMESTAMP and UD_DURATION_SECONDS must be provided together");
  }
  return { startTimestamp: BigInt(startTimestamp), durationSeconds: BigInt(durationSeconds) };
};

// The source-file SDK dependency exports types from generated `_types`, which is absent in clean
// CLI checkouts. Keep this structural seam narrow; the real vertical checks the public SDK call.
const runPublicSdkUserDecrypt: CurrentUserDecryptSdkCall = async (input) => {
  const solanaModule = "@fhevm/sdk/solana";
  const solana = await import(solanaModule);
  const chain = solana.defineFhevmSolanaChain({
    id: input.chainId,
    fhevm: { relayerUrl: input.relayerUrl, acl: { domainKeys: input.request.allowedAclDomainKeys } },
  });
  solana.setFhevmRuntimeConfig({ auth: { type: "ApiKeyHeader", value: input.apiKey } });
  const signer = solana.solanaSignerFromSecretKey(input.secretKey);
  return solana.createFhevmDecryptClient({ signer, chain }).userDecrypt(input.request);
};

/** Runs the current-handle Solana user-decrypt flow through the public SDK. */
export const runSolanaCurrentUserDecrypt = async (
  environment: Environment = process.env,
  dependencies: CurrentUserDecryptDependencies = {},
): Promise<bigint> => {
  const allowedAclDomainKeys = required(environment, "UD_ALLOWED_DOMAIN_KEYS")
    .split(",")
    .map((value) => value.trim())
    .filter(Boolean)
    .map((value) => bytes32Hex(value, "UD_ALLOWED_DOMAIN_KEYS"));
  if (allowedAclDomainKeys.length === 0) {
    throw new PreflightError("UD_ALLOWED_DOMAIN_KEYS must contain at least one key");
  }

  const handle = required(environment, "UD_HANDLE");
  bytes32Hex(handle, "UD_HANDLE");
  const expected = BigInt(required(environment, "UD_EXPECTED"));
  const validity = optionalValidity(environment);
  const request: CurrentUserDecryptRequest = {
    handles: [handle],
    allowedAclDomainKeys,
    contextId: bytes32(environment, "UD_CONTEXT_ID"),
    aclValueKey: bytes32(environment, "UD_ACL_VALUE_KEY"),
    ...(environment.UD_NONCE ? { nonce: bytes32(environment, "UD_NONCE") } : {}),
    ...(validity ? { validity } : {}),
  };

  const userDecrypt = dependencies.userDecrypt ?? runPublicSdkUserDecrypt;
  const clearValues = await userDecrypt({
    chainId: BigInt(required(environment, "UD_CONTRACTS_CHAIN_ID")),
    relayerUrl: required(environment, "UD_RELAYER_URL"),
    apiKey: environment.ZAMA_FHEVM_API_KEY ?? "local",
    secretKey: bytes32(environment, "UD_SECRET_KEY"),
    request,
  });
  if (clearValues.length !== 1) {
    throw new Error(`user-decrypt returned ${clearValues.length} clear values; expected exactly 1`);
  }

  const value = BigInt(clearValues[0].value);
  if (value !== expected) {
    throw new Error(`user-decrypt cleartext ${value} != expected ${expected}`);
  }
  console.log(`[solana-current-user-decrypt] cleartext=${value}`);
  return value;
};
