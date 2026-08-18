import { PreflightError } from "../errors";

export const SOLANA_CURRENT_USER_DECRYPT_PROFILE = "solana-current-user-decrypt";
export const SOLANA_CURRENT_USER_DECRYPT_DESCRIPTION =
  "Decrypt one current Solana handle through the public SDK and assert its plaintext.";

type Environment = Readonly<Record<string, string | undefined>>;

type CurrentUserDecryptSdkInput = {
  chainId: bigint;
  relayerUrl: string;
  rpcUrl: string;
  proofServiceUrl: string;
  verifyingProgramId: string;
  allowedAclDomainKeys: readonly string[];
  apiKey: string;
  secretKey: Uint8Array;
  trust: {
    kmsSigners: readonly { partyId: number; address: string }[];
    kmsContextId: string;
    kmsEpochId: string;
    fheParameter: string;
    gatewayEip712Domain: { name: string; version: string; chainId: bigint; verifyingContract: string };
  };
  request: {
    handle: Uint8Array;
    encryptedValueId: Uint8Array;
    durationSeconds: bigint;
  };
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

const evmAddress = (value: string, name: string): string => {
  if (!/^0x[0-9a-f]{40}$/i.test(value)) {
    throw new PreflightError(`${name} must be a 0x-prefixed 20-byte hex address`);
  }
  return value;
};

const ZERO_EPOCH = `0x${"0".repeat(64)}`;

// The source-file SDK dependency exports types from generated `_types`, which is absent in clean
// CLI checkouts. Keep this structural seam narrow; the real vertical checks the public SDK call.
const runPublicSdkUserDecrypt: CurrentUserDecryptSdkCall = async (input) => {
  const solanaModule = "@fhevm/sdk/solana";
  const solana = await import(solanaModule);
  const chain = solana.defineFhevmSolanaChain({
    id: input.chainId,
    fhevm: {
      relayerUrl: input.relayerUrl,
      acl: { domainKeys: input.allowedAclDomainKeys },
      rpcUrl: input.rpcUrl,
      proofServiceUrl: input.proofServiceUrl,
      verifyingProgramId: input.verifyingProgramId,
    },
  });
  solana.setFhevmRuntimeConfig({ auth: { type: "ApiKeyHeader", value: input.apiKey } });
  const client = solana.createFhevmDecryptClient({ chain, trust: input.trust });
  // The permit path: one wallet signature mints a session, the request runs under it.
  const wallet = solana.solanaPermitWalletFromSecretKey(input.secretKey);
  const session = await client.signPermit({ wallet, durationSeconds: input.request.durationSeconds });
  return client.userDecrypt({
    session,
    entries: [{ handle: input.request.handle, encryptedValueId: input.request.encryptedValueId }],
  });
};

/** Runs the current-handle Solana user-decrypt flow through the public SDK's permit path. */
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

  // The trust configuration: whom the client believes. Signer party ids follow the registry order,
  // the same first-is-party-one assumption the EVM SDK path makes.
  const kmsSigners = required(environment, "UD_KMS_SIGNERS")
    .split(",")
    .map((value) => value.trim())
    .filter(Boolean)
    .map((value, index) => ({ partyId: index + 1, address: evmAddress(value, "UD_KMS_SIGNERS") }));
  if (kmsSigners.length === 0) {
    throw new PreflightError("UD_KMS_SIGNERS must contain at least one address");
  }

  const userDecrypt = dependencies.userDecrypt ?? runPublicSdkUserDecrypt;
  const clearValues = await userDecrypt({
    chainId: BigInt(required(environment, "UD_CONTRACTS_CHAIN_ID")),
    relayerUrl: required(environment, "UD_RELAYER_URL"),
    rpcUrl: required(environment, "UD_RPC_URL"),
    proofServiceUrl: required(environment, "UD_PROOF_SERVICE_URL"),
    verifyingProgramId: bytes32Hex(required(environment, "UD_VERIFYING_PROGRAM_ID"), "UD_VERIFYING_PROGRAM_ID"),
    allowedAclDomainKeys,
    apiKey: environment.ZAMA_FHEVM_API_KEY ?? "local",
    secretKey: bytes32(environment, "UD_SECRET_KEY"),
    trust: {
      kmsSigners,
      kmsContextId: bytes32Hex(required(environment, "UD_CONTEXT_ID"), "UD_CONTEXT_ID"),
      kmsEpochId: bytes32Hex(environment.UD_EPOCH_ID ?? ZERO_EPOCH, "UD_EPOCH_ID"),
      fheParameter: environment.UD_FHE_PARAMETER ?? "test",
      gatewayEip712Domain: {
        name: "Decryption",
        version: "1",
        chainId: BigInt(required(environment, "UD_GATEWAY_CHAIN_ID")),
        verifyingContract: evmAddress(
          required(environment, "UD_GATEWAY_DECRYPTION_CONTRACT"),
          "UD_GATEWAY_DECRYPTION_CONTRACT",
        ),
      },
    },
    request: {
      handle: bytes(handle, "UD_HANDLE"),
      encryptedValueId: bytes32(environment, "UD_ACL_VALUE_KEY"),
      durationSeconds: BigInt(environment.UD_DURATION_SECONDS ?? "3600"),
    },
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
