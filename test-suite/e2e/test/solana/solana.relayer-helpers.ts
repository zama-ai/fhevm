// ---------------------------------------------------------------------------
// Solana relayer helpers
//
// Handles user-decrypt and public-decrypt relayer calls for Solana e2e tests.
// Uses EIP-712 (required by the relayer/KMS protocol) alongside native Solana
// Ed25519 signatures for native-auth flows.
// ---------------------------------------------------------------------------

import fs from 'node:fs';

import { ed25519 } from '@noble/curves/ed25519';
import { ethers } from 'ethers';
import TKMS from 'node-tkms';

import { RELAYER_URL, DECRYPTION_EXTRA_DATA_VERSION } from './solana.config';
import {
  bytesToHex,
  ensure0xHex,
  fetchJsonExpectOk,
  hexToBytes,
  normalizeStackHttpUrl,
  requiredEnvValue,
  sleep,
  stripHexPrefix,
} from './solana.proof-helpers';

// ---------------------------------------------------------------------------
// Minimal wallet interface — avoids exposing ethers types to test files
// ---------------------------------------------------------------------------

export interface TestWallet {
  address: string;
  signTypedData(
    domain: Record<string, unknown>,
    types: Record<string, Array<{ name: string; type: string }>>,
    value: Record<string, unknown>,
  ): Promise<string>;
}

export function createTestWallet(privateKey: string): TestWallet {
  return new ethers.Wallet(normalizeHexPrivateKey(privateKey));
}

export function createRandomTestWallet(): TestWallet {
  return ethers.Wallet.createRandom();
}

function normalizeHexPrivateKey(value: string): string {
  return value.startsWith('0x') ? value : `0x${value}`;
}

// ---------------------------------------------------------------------------
// KMS signer loading
// ---------------------------------------------------------------------------

export function loadKmsSigners(envValues: Record<string, string>): string[] {
  const count = Number(requiredEnvValue(envValues, 'NUM_KMS_NODES', 'Solana user decrypt'));
  const signers: string[] = [];
  for (let index = 0; index < count; index += 1) {
    signers.push(
      ethers.getAddress(
        requiredEnvValue(
          envValues,
          `KMS_SIGNER_ADDRESS_${index}`,
          'Solana user decrypt',
        ),
      ),
    );
  }
  return signers;
}

export function loadKmsContextId(envValues: Record<string, string>): string {
  return requiredEnvValue(envValues, 'KMS_CONTEXT_ID', 'Solana user decrypt');
}

// ---------------------------------------------------------------------------
// Solana Ed25519 keypair loading
// ---------------------------------------------------------------------------

export function loadSolanaKeypairBytes(keypairPath: string): Uint8Array {
  const raw = fs.readFileSync(keypairPath, 'utf8');
  const parsed = JSON.parse(raw);
  if (!Array.isArray(parsed)) {
    throw new Error(`invalid Solana keypair file ${keypairPath}: expected JSON byte array`);
  }
  return Uint8Array.from(parsed);
}

export function loadSolanaEd25519PrivateKey(keypairPath: string): Uint8Array {
  const keypairBytes = loadSolanaKeypairBytes(keypairPath);
  if (keypairBytes.length < 32) {
    throw new Error(`invalid Solana keypair file ${keypairPath}: expected at least 32 bytes`);
  }
  return keypairBytes.slice(0, 32);
}

export function loadSolanaEd25519PublicKey(keypairPath: string): Uint8Array {
  const keypairBytes = loadSolanaKeypairBytes(keypairPath);
  if (keypairBytes.length >= 64) {
    return keypairBytes.slice(32, 64);
  }
  return ed25519.getPublicKey(loadSolanaEd25519PrivateKey(keypairPath));
}

// ---------------------------------------------------------------------------
// Native Solana auth signer construction
//
// The auth signer is a verifier_address + public_key blob. The verifier is
// deployed once during `gateway-deploy` and its address is written to the
// test-suite env as SOLANA_ED25519_VERIFIER_ADDRESS.
// ---------------------------------------------------------------------------

export function buildNativeSolanaAuthSigner(
  testSuiteEnv: Record<string, string>,
  keypairPath: string,
): string {
  const verifierAddress = requiredEnvValue(
    testSuiteEnv,
    'SOLANA_ED25519_VERIFIER_ADDRESS',
    'solana-native-auth',
  );
  const publicKey = loadSolanaEd25519PublicKey(keypairPath);
  return bytesToHex(
    Uint8Array.from([
      ...hexToBytes(verifierAddress),
      ...publicKey,
    ]),
    true,
  );
}

// ---------------------------------------------------------------------------
// EIP-712 helpers for Solana — signs typed data with an Ed25519 key
// ---------------------------------------------------------------------------

export function signEd25519TypedData(
  eip712: EIP712TypedData,
  privateKey: Uint8Array,
): string {
  const primaryType = requiredString(eip712.primaryType, 'primaryType');
  const digest = ethers.TypedDataEncoder.hash(
    eip712.domain,
    {
      [primaryType]: requiredValue(
        eip712.types?.[primaryType],
        `${primaryType} typed-data fields`,
      ),
    },
    eip712.message,
  );
  return bytesToHex(ed25519.sign(hexToBytes(digest), privateKey), true);
}

// ---------------------------------------------------------------------------
// Decryption extra-data encoding
// ---------------------------------------------------------------------------

export function buildDecryptionExtraData(
  contextId: string,
  identities: string[],
  authSigner: string | null = null,
): string {
  if (!Array.isArray(identities) || identities.length === 0) {
    throw new Error('decryption extraData requires at least one identity');
  }
  const normalizedContextId = BigInt(contextId);
  const parts: Buffer[] = [
    Buffer.from([DECRYPTION_EXTRA_DATA_VERSION]),
    Buffer.from(
      normalizedContextId.toString(16).padStart(64, '0'),
      'hex',
    ),
    Buffer.from([identities.length]),
  ];
  for (const identity of identities) {
    parts.push(Buffer.from(hexToBytes(identity)));
  }
  if (authSigner) {
    const authSignerBytes = hexToBytes(authSigner);
    if (authSignerBytes.length < 20 || authSignerBytes.length > 255) {
      throw new Error(`invalid authSigner length: ${authSignerBytes.length}`);
    }
    parts.push(Buffer.from([authSignerBytes.length]));
    parts.push(Buffer.from(authSignerBytes));
  }
  return `0x${Buffer.concat(parts).toString('hex')}`;
}

// ---------------------------------------------------------------------------
// ML-KEM keypair for relayer encryption
// ---------------------------------------------------------------------------

export function generateRelayerKeypair(): { publicKey: string; privateKey: string } {
  const privateKey = TKMS.ml_kem_pke_keygen();
  const publicKey = TKMS.ml_kem_pke_get_pk(privateKey);
  return {
    publicKey: bytesToHex(TKMS.ml_kem_pke_pk_to_u8vec(publicKey)),
    privateKey: bytesToHex(TKMS.ml_kem_pke_sk_to_u8vec(privateKey)),
  };
}

// ---------------------------------------------------------------------------
// EIP-712 types
// ---------------------------------------------------------------------------

export interface EIP712TypedData {
  types: Record<string, Array<{ name: string; type: string }>>;
  primaryType: string;
  domain: {
    name: string;
    version: string;
    chainId: number;
    verifyingContract: string;
  };
  message: Record<string, unknown>;
}

export function buildUserDecryptEip712({
  verifyingContract,
  contractsChainId,
  publicKey,
  contractAddresses,
  startTimestamp,
  durationDays,
  extraData,
}: {
  verifyingContract: string;
  contractsChainId: number;
  publicKey: string;
  contractAddresses: string[];
  startTimestamp: number;
  durationDays: number;
  extraData: string;
}): EIP712TypedData {
  if (!ethers.isAddress(verifyingContract)) {
    throw new Error(`invalid verifying contract address: ${verifyingContract}`);
  }
  if (
    !Array.isArray(contractAddresses) ||
    contractAddresses.some((address) => !ethers.isAddress(address))
  ) {
    throw new Error(`invalid contract address list for user decrypt: ${contractAddresses}`);
  }

  return {
    types: {
      EIP712Domain: [
        { name: 'name', type: 'string' },
        { name: 'version', type: 'string' },
        { name: 'chainId', type: 'uint256' },
        { name: 'verifyingContract', type: 'address' },
      ],
      UserDecryptRequestVerification: [
        { name: 'publicKey', type: 'bytes' },
        { name: 'contractAddresses', type: 'address[]' },
        { name: 'startTimestamp', type: 'uint256' },
        { name: 'durationDays', type: 'uint256' },
        { name: 'extraData', type: 'bytes' },
      ],
    },
    primaryType: 'UserDecryptRequestVerification',
    domain: {
      name: 'Decryption',
      version: '1',
      chainId: contractsChainId,
      verifyingContract,
    },
    message: {
      publicKey: ensure0xHex(publicKey),
      contractAddresses,
      startTimestamp: String(startTimestamp),
      durationDays: String(durationDays),
      extraData,
    },
  };
}

// ---------------------------------------------------------------------------
// User decrypt request
// ---------------------------------------------------------------------------

export interface UserDecryptRequestParams {
  handle?: string;
  handles?: string[];
  contractAddress?: string;
  contractAddresses?: string[];
  userIdentity?: string;
  nativeContractIdentities?: string[] | null;
  nativeSignerKeypairPath?: string;
  handleContractPairs?: Array<{ handle: string; contractAddress: string }>;
  userWallet: TestWallet;
  addresses: Record<string, string>;
  testSuiteEnv: Record<string, string>;
  label: string;
  startTimestamp?: number;
  durationDays?: number;
}

export interface UserDecryptResult {
  accepted: boolean;
  success?: boolean;
  jobId?: string;
  httpStatus?: number;
  responseText?: string;
  parseError?: string;
  errorMessage?: string | null;
  clearValues?: Record<string, unknown>;
  decryptedValue?: unknown;
  userAddress?: string;
  payload?: unknown;
}

export async function executeUserDecryptRequest({
  handle,
  handles,
  contractAddress,
  contractAddresses,
  userIdentity,
  nativeContractIdentities,
  nativeSignerKeypairPath,
  handleContractPairs,
  userWallet,
  addresses,
  testSuiteEnv,
  label,
  startTimestamp,
  durationDays,
}: UserDecryptRequestParams): Promise<UserDecryptResult> {
  const { publicKey, privateKey } = generateRelayerKeypair();
  const requestHandles = Array.isArray(handles) && handles.length > 0 ? handles : [handle];
  const requestContractAddresses: string[] =
    Array.isArray(contractAddresses) && contractAddresses.length > 0
      ? contractAddresses
      : contractAddress !== undefined ? [contractAddress] : [];
  const requestHandleContractPairs =
    Array.isArray(handleContractPairs) && handleContractPairs.length > 0
      ? handleContractPairs
      : requestHandles.map((requestHandle) => ({
          handle: requestHandle,
          contractAddress,
        }));
  const requestNativeContractIdentities =
    Array.isArray(nativeContractIdentities) && nativeContractIdentities.length > 0
      ? nativeContractIdentities
      : [];
  const requestStartTimestamp =
    startTimestamp ?? (await latestGatewayBlockTimestamp(testSuiteEnv, label));
  const requestDurationDays = durationDays ?? 1;
  const userAddress = userWallet.address;
  const useNativeSolanaAuth =
    typeof nativeSignerKeypairPath === 'string' &&
    nativeSignerKeypairPath.length > 0 &&
    userIdentity &&
    requestNativeContractIdentities.length === requestContractAddresses.length;
  const extraData = useNativeSolanaAuth
    ? buildDecryptionExtraData(
        loadKmsContextId({ ...addresses, ...testSuiteEnv }),
        [userIdentity as string, ...requestNativeContractIdentities],
        buildNativeSolanaAuthSigner(testSuiteEnv, nativeSignerKeypairPath as string),
      )
    : userIdentity && requestNativeContractIdentities.length === requestContractAddresses.length
      ? buildDecryptionExtraData(
          loadKmsContextId({ ...addresses, ...testSuiteEnv }),
          [userIdentity, ...requestNativeContractIdentities],
        )
      : '0x00';
  const eip712 = buildUserDecryptEip712({
    verifyingContract: requiredEnvValue(testSuiteEnv, 'DECRYPTION_ADDRESS', label),
    contractsChainId: Number(addresses.SOLANA_HOST_CHAIN_ID),
    publicKey,
    contractAddresses: requestContractAddresses,
    startTimestamp: requestStartTimestamp,
    durationDays: requestDurationDays,
    extraData,
  });
  const signature = useNativeSolanaAuth
    ? signEd25519TypedData(eip712, loadSolanaEd25519PrivateKey(nativeSignerKeypairPath as string))
    : await userWallet.signTypedData(
        eip712.domain as Record<string, unknown>,
        Object.fromEntries(
          Object.entries(eip712.types).filter(([typeName]) => typeName !== 'EIP712Domain'),
        ),
        eip712.message,
      );

  const body = {
    handleContractPairs: requestHandleContractPairs,
    requestValidity: {
      startTimestamp: String(requestStartTimestamp),
      durationDays: String(requestDurationDays),
    },
    contractsChainId: addresses.SOLANA_HOST_CHAIN_ID,
    contractAddresses: requestContractAddresses,
    userAddress,
    signature: signature.replace(/^0x/, ''),
    publicKey: publicKey.replace(/^0x/, ''),
    extraData,
    ...(userIdentity ? { userId: userIdentity } : {}),
    ...(requestNativeContractIdentities.length > 0
      ? { contractIds: requestNativeContractIdentities }
      : {}),
  };

  const response = await fetch(`${RELAYER_URL}/v2/user-decrypt`, {
    method: 'POST',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify(body),
  });
  const responseText = await response.text();

  if (!response.ok) {
    return {
      accepted: false,
      httpStatus: response.status,
      responseText,
      errorMessage: extractErrorMessage(responseText),
    };
  }

  const postPayload = JSON.parse(responseText);
  const jobId = postPayload?.result?.jobId;
  if (!jobId) {
    return {
      accepted: false,
      httpStatus: response.status,
      responseText,
      parseError: 'missing jobId',
      errorMessage: extractErrorMessage(responseText),
    };
  }

  const statusResult = await waitForUserDecryptJob(jobId);
  if (statusResult.httpStatus !== 200 || (statusResult.payload as Record<string, unknown>)?.status !== 'succeeded') {
    return {
      accepted: true,
      success: false,
      jobId,
      httpStatus: statusResult.httpStatus,
      payload: statusResult.payload,
      errorMessage: extractPayloadErrorMessage(statusResult.payload),
    };
  }

  const clearValues = decryptUserDecryptResult({
    responsePayload: statusResult.payload,
    handles: requestHandles as string[],
    userAddress,
    publicKey,
    privateKey,
    gatewayChainId: requiredEnvValue(testSuiteEnv, 'CHAIN_ID_GATEWAY', label),
    verifyingContract: requiredEnvValue(testSuiteEnv, 'DECRYPTION_ADDRESS', label),
    kmsSigners: loadKmsSigners(addresses),
    extraData,
  });

  return {
    accepted: true,
    success: true,
    jobId,
    clearValues,
    decryptedValue: clearValues[requestHandles[0] as string],
    userAddress,
  };
}

export function formatUserDecryptOutcome(result: UserDecryptResult): string {
  if (!result.accepted) {
    return `http=${result.httpStatus} body=${result.responseText ?? ''}${
      result.parseError ? ` parseError=${result.parseError}` : ''
    }`;
  }
  return `http=${result.httpStatus} payload=${JSON.stringify(result.payload)}`;
}

function includesErrorMessage(
  result: UserDecryptResult | PublicDecryptResult,
  snippet: string,
): boolean {
  const message = String(
    result?.errorMessage ??
      result?.responseText ??
      JSON.stringify(result?.payload ?? {}),
  ).toLowerCase();
  return message.includes(String(snippet).toLowerCase());
}

export function isUserDecryptAclRejection(result: UserDecryptResult): boolean {
  return (
    includesErrorMessage(result, 'not authorized to user decrypt handle') ||
    includesErrorMessage(result, 'not allowed on host acl') ||
    includesErrorMessage(result, 'acl check failed')
  );
}

export function isUserEqualsContractRejection(result: UserDecryptResult): boolean {
  return (
    includesErrorMessage(
      result,
      'should not be equal to contract address when requesting user decryption',
    ) ||
    includesErrorMessage(result, 'useraddressincontractaddresses') ||
    includesErrorMessage(result, '0xdc4d78b1')
  );
}

export function isExpiredUserDecryptRejection(result: UserDecryptResult): boolean {
  return (
    includesErrorMessage(result, 'user decrypt request has expired') ||
    includesErrorMessage(result, 'userdecryptionrequestexpired') ||
    includesErrorMessage(result, '0x30348040')
  );
}

async function waitForUserDecryptJob(
  jobId: string,
): Promise<{ httpStatus: number; payload: unknown }> {
  const started = Date.now();
  while (Date.now() - started < 120_000) {
    const response = await fetch(`${RELAYER_URL}/v2/user-decrypt/${jobId}`);
    const text = await response.text();
    let payload = null;
    if (text) {
      try {
        payload = JSON.parse(text);
      } catch (error) {
        throw new Error(
          `user decrypt status check returned non-JSON payload for ${jobId}: ${error}\n${text}`,
        );
      }
    }
    if (response.status === 202 && (payload as Record<string, unknown>)?.status === 'queued') {
      await sleep(1000);
      continue;
    }
    return { httpStatus: response.status, payload };
  }
  throw new Error(`timed out waiting for user decrypt job ${jobId}`);
}

function decryptUserDecryptResult({
  responsePayload,
  handles,
  userAddress,
  publicKey,
  privateKey,
  gatewayChainId,
  verifyingContract,
  kmsSigners,
  extraData,
}: {
  responsePayload: unknown;
  handles: string[];
  userAddress: string;
  publicKey: string;
  privateKey: string;
  gatewayChainId: string;
  verifyingContract: string;
  kmsSigners: string[];
  extraData: string;
}): Record<string, unknown> {
  const payload = responsePayload as Record<string, unknown>;
  const aggregatedResponse = (payload?.result as Record<string, unknown>)?.result;
  if (!Array.isArray(aggregatedResponse) || aggregatedResponse.length === 0) {
    throw new Error(
      `user decrypt response payload does not contain re-encrypted shares: ${JSON.stringify(
        responsePayload,
      )}`,
    );
  }
  const normalizedAggregatedResponse = (aggregatedResponse as Array<Record<string, unknown>>).map((share) => ({
    payload:
      typeof share?.payload === 'string'
        ? share.payload.replace(/^0x/i, '')
        : share?.payload,
    signature:
      typeof share?.signature === 'string'
        ? share.signature.replace(/^0x/i, '')
        : share?.signature,
    extra_data:
      typeof share?.extraData === 'string'
        ? share.extraData.replace(/^0x/i, '')
        : typeof share?.extra_data === 'string'
          ? share.extra_data.replace(/^0x/i, '')
          : undefined,
  }));

  verifyUserDecryptResponseSignatures({
    aggregatedResponse,
    handles,
    publicKey,
    gatewayChainId,
    verifyingContract,
    kmsSigners,
    extraData,
  });

  const publicKeyBytes = TKMS.u8vec_to_ml_kem_pke_pk(hexToBytes(publicKey));
  const privateKeyBytes = TKMS.u8vec_to_ml_kem_pke_sk(hexToBytes(privateKey));
  const indexedKmsSigners = kmsSigners.map((signer, index) =>
    TKMS.new_server_id_addr(index + 1, signer),
  );
  const client = TKMS.new_client(indexedKmsSigners, userAddress, 'default');
  const decrypted = TKMS.process_user_decryption_resp_from_js(
    client,
    null,
    null,
    normalizedAggregatedResponse,
    publicKeyBytes,
    privateKeyBytes,
    false,
  );
  const clearValues: Record<string, unknown> = {};
  for (let index = 0; index < handles.length; index += 1) {
    clearValues[handles[index]] = decodeUserDecryptBytes(
      handles[index],
      (decrypted[index] as Record<string, unknown>)?.bytes ?? [],
    );
  }
  return clearValues;
}

function verifyUserDecryptResponseSignatures({
  aggregatedResponse,
  handles,
  publicKey,
  gatewayChainId,
  verifyingContract,
  kmsSigners,
  extraData,
}: {
  aggregatedResponse: Array<Record<string, unknown>>;
  handles: string[];
  publicKey: string;
  gatewayChainId: string;
  verifyingContract: string;
  kmsSigners: string[];
  extraData: string;
}): void {
  const domain = {
    name: 'Decryption',
    version: '1',
    chainId: Number(gatewayChainId),
    verifyingContract: ethers.getAddress(verifyingContract),
  };
  const types = {
    UserDecryptResponseVerification: [
      { name: 'publicKey', type: 'bytes' },
      { name: 'ctHandles', type: 'bytes32[]' },
      { name: 'userDecryptedShare', type: 'bytes' },
      { name: 'extraData', type: 'bytes' },
    ],
  };
  const normalizedExpected = new Set(kmsSigners.map((signer) => ethers.getAddress(signer)));
  const normalizedRecovered = aggregatedResponse.map((share) => {
    const sharePayload = requiredString(share?.payload, 'user decrypt share payload');
    const shareSignature = ensure0xHex(requiredString(share?.signature, 'user decrypt share signature'));
    const shareExtraData =
      typeof share?.extraData === 'string'
        ? share.extraData
        : typeof share?.extra_data === 'string'
          ? share.extra_data
          : extraData;
    return ethers.getAddress(
      ethers.verifyTypedData(
        domain,
        types,
        {
          publicKey: ensure0xHex(requiredString(publicKey, 'user decrypt public key')),
          ctHandles: handles.map((h) => ensure0xHex(requiredString(h, 'user decrypt handle'))),
          userDecryptedShare: ensure0xHex(sharePayload),
          extraData: ensure0xHex((shareExtraData as string) ?? '0x'),
        },
        shareSignature,
      ),
    );
  });

  const uniqueRecovered = new Set(normalizedRecovered);
  if (uniqueRecovered.size !== normalizedRecovered.length) {
    throw new Error('duplicate KMS signer recovered from user decrypt response');
  }
  for (const signer of normalizedRecovered) {
    if (!normalizedExpected.has(signer)) {
      throw new Error(`unexpected KMS signer in user decrypt response: ${signer}`);
    }
  }
  if (normalizedRecovered.length === 0) {
    throw new Error('user decrypt response did not contain any signed shares');
  }
}

function decodeUserDecryptBytes(handle: string, bytes: unknown): unknown {
  const typeDiscriminant = extractHandleTypeDiscriminant(handle);
  const normalizedBytes =
    bytes instanceof Uint8Array ? bytes : Uint8Array.from((bytes as number[]) ?? []);

  switch (typeDiscriminant) {
    case 0:
      return normalizedBytes.some((value) => value !== 0);
    case 2:
    case 3:
    case 4:
    case 5:
    case 6:
    case 8:
      return bytesToDecimalString(normalizedBytes);
    case 7: {
      const addressBytes =
        normalizedBytes.length >= 20
          ? normalizedBytes.slice(normalizedBytes.length - 20)
          : normalizedBytes;
      return `0x${Buffer.from(addressBytes).toString('hex')}`.toLowerCase();
    }
    default:
      throw new Error(
        `unsupported user decrypt handle type ${typeDiscriminant} for handle ${handle}`,
      );
  }
}

function bytesToDecimalString(bytes: Uint8Array | Buffer): string {
  if (!bytes || bytes.length === 0) {
    return '0';
  }
  return BigInt(`0x${Buffer.from(bytes).toString('hex')}`).toString(10);
}

// ---------------------------------------------------------------------------
// Public decrypt request
// ---------------------------------------------------------------------------

export interface PublicDecryptResult {
  accepted: boolean;
  success: boolean;
  httpStatus?: number;
  responseText?: string;
  errorMessage?: string | null;
  parseError?: string;
  jobId?: string;
  clearValues: Record<string, unknown>;
  payload?: unknown;
}

export async function executePublicDecryptRequest({
  handles,
  label,
}: {
  handles: string[];
  label: string;
}): Promise<PublicDecryptResult> {
  const response = await fetch(`${RELAYER_URL}/v2/public-decrypt`, {
    method: 'POST',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify({
      ciphertextHandles: handles,
      extraData: '0x00',
    }),
  });
  const responseText = await response.text();
  if (!response.ok) {
    return {
      accepted: false,
      success: false,
      httpStatus: response.status,
      responseText,
      errorMessage: extractErrorMessage(responseText),
      clearValues: {},
    };
  }

  const postPayload = JSON.parse(responseText);
  const jobId = postPayload?.result?.jobId;
  if (!jobId) {
    return {
      accepted: false,
      success: false,
      httpStatus: response.status,
      responseText,
      parseError: 'missing jobId',
      errorMessage: extractErrorMessage(responseText),
      clearValues: {},
    };
  }

  const statusResult = await waitForPublicDecryptJob(jobId);
  if (
    statusResult.httpStatus !== 200 ||
    (statusResult.payload as Record<string, unknown>)?.status !== 'succeeded'
  ) {
    return {
      accepted: true,
      success: false,
      jobId,
      httpStatus: statusResult.httpStatus,
      payload: statusResult.payload,
      responseText: statusResult.responseText,
      errorMessage:
        extractPayloadErrorMessage(statusResult.payload) ??
        extractErrorMessage(statusResult.responseText ?? ''),
      clearValues: {},
    };
  }

  return {
    accepted: true,
    success: true,
    jobId,
    httpStatus: statusResult.httpStatus,
    payload: statusResult.payload,
    clearValues: decodePublicDecryptClearValues(
      handles,
      (((statusResult.payload as Record<string, unknown>)?.result as Record<string, unknown>)
        ?.decryptedValue as string | undefined),
    ),
  };
}

export function formatPublicDecryptOutcome(result: PublicDecryptResult): string {
  if (!result.accepted) {
    return `http=${result.httpStatus} body=${result.responseText ?? ''}${
      result.parseError ? ` parseError=${result.parseError}` : ''
    }`;
  }
  return `http=${result.httpStatus} payload=${JSON.stringify(result.payload)}`;
}

export function includesPublicDecryptErrorMessage(
  result: PublicDecryptResult,
  snippet: string,
): boolean {
  const message = String(
    result?.errorMessage ??
      result?.responseText ??
      JSON.stringify(result?.payload ?? {}),
  ).toLowerCase();
  return message.includes(String(snippet).toLowerCase());
}

async function waitForPublicDecryptJob(
  jobId: string,
): Promise<{ httpStatus: number; payload: unknown; responseText?: string }> {
  const started = Date.now();
  while (Date.now() - started < 120_000) {
    const response = await fetch(`${RELAYER_URL}/v2/public-decrypt/${jobId}`);
    const text = await response.text();
    let payload = null;
    if (text) {
      try {
        payload = JSON.parse(text);
      } catch (error) {
        throw new Error(
          `public decrypt status check returned non-JSON payload for ${jobId}: ${error}\n${text}`,
        );
      }
    }
    if (
      response.status === 202 &&
      (payload as Record<string, unknown>)?.status === 'queued'
    ) {
      await sleep(1000);
      continue;
    }
    return { httpStatus: response.status, payload, responseText: text };
  }
  throw new Error(`timed out waiting for public decrypt job ${jobId}`);
}

function decodePublicDecryptClearValues(
  handles: string[],
  decryptedValueHex: string | undefined,
): Record<string, unknown> {
  if (!Array.isArray(handles) || handles.length === 0) {
    throw new Error('public decrypt decode requires at least one handle');
  }
  if (typeof decryptedValueHex !== 'string') {
    throw new Error('public decrypt response missing decryptedValue');
  }

  const normalized = decryptedValueHex.startsWith('0x')
    ? decryptedValueHex.slice(2)
    : decryptedValueHex;
  if (normalized.length % 64 !== 0) {
    throw new Error(
      `public decrypt decryptedValue length must be a multiple of 32 bytes, got ${normalized.length / 2} bytes`,
    );
  }

  const words: string[] = [];
  for (let offset = 0; offset < normalized.length; offset += 64) {
    words.push(normalized.slice(offset, offset + 64));
  }
  if (words.length < handles.length) {
    throw new Error(
      `public decrypt response returned ${words.length} words for ${handles.length} handles`,
    );
  }

  const clearValues: Record<string, unknown> = {};
  for (let index = 0; index < handles.length; index += 1) {
    clearValues[handles[index]] = decodePublicDecryptWord(handles[index], words[index]);
  }
  return clearValues;
}

function decodePublicDecryptWord(handle: string, wordHex: string): unknown {
  const typeDiscriminant = extractHandleTypeDiscriminant(handle);
  const bytes = hexToBytes(wordHex);

  switch (typeDiscriminant) {
    case 0:
      return bytes[31] === 1;
    case 2:
    case 3:
    case 4:
    case 5:
    case 6:
    case 8:
      return BigInt(`0x${wordHex}`).toString(10);
    case 7:
      return `0x${Buffer.from(bytes.slice(12, 32)).toString('hex')}`.toLowerCase();
    default:
      throw new Error(
        `unsupported public decrypt handle type ${typeDiscriminant} for handle ${handle}`,
      );
  }
}

function extractHandleTypeDiscriminant(handle: string): number {
  const normalized = handle.startsWith('0x') ? handle.slice(2) : handle;
  if (normalized.length < 4) {
    throw new Error(`handle too short to decode type: ${handle}`);
  }
  return Number.parseInt(normalized.slice(-4, -2), 16);
}

// ---------------------------------------------------------------------------
// Gateway block timestamp (for EIP-712 request validity window)
// ---------------------------------------------------------------------------

export async function latestGatewayBlockTimestamp(
  testSuiteEnv: Record<string, string>,
  label: string,
): Promise<number> {
  const gatewayRpcUrl = normalizeStackHttpUrl(
    requiredEnvValue(testSuiteEnv, 'GATEWAY_RPC_URL', label),
  ).toString();
  const payload = await fetchJsonExpectOk(gatewayRpcUrl, {
    method: 'POST',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify({
      jsonrpc: '2.0',
      id: 1,
      method: 'eth_getBlockByNumber',
      params: ['latest', false],
    }),
  }) as Record<string, unknown>;
  const blockTimestampHex = (payload?.result as Record<string, unknown>)?.timestamp;
  if (typeof blockTimestampHex !== 'string') {
    throw new Error(
      `${label}: gateway RPC eth_getBlockByNumber returned no timestamp: ${JSON.stringify(payload)}`,
    );
  }
  return Number(BigInt(blockTimestampHex));
}

// ---------------------------------------------------------------------------
// Internal utilities
// ---------------------------------------------------------------------------

function extractErrorMessage(responseText: string): string | null {
  if (typeof responseText !== 'string' || responseText.length === 0) {
    return null;
  }
  try {
    const payload = JSON.parse(responseText);
    return (
      payload?.error?.message ??
      payload?.result?.error?.message ??
      payload?.message ??
      responseText
    );
  } catch {
    return responseText;
  }
}

function extractPayloadErrorMessage(payload: unknown): string | null {
  if (!payload || typeof payload !== 'object') {
    return null;
  }
  const p = payload as Record<string, unknown>;
  const error = p['error'] as Record<string, unknown> | undefined;
  const result = p['result'] as Record<string, unknown> | undefined;
  const resultError = result?.['error'] as Record<string, unknown> | undefined;
  return (
    (error?.['message'] as string | undefined) ??
    (resultError?.['message'] as string | undefined) ??
    (p['message'] as string | undefined) ??
    null
  );
}

function requiredString(value: unknown, fieldName: string): string {
  if (typeof value !== 'string' || value.length === 0) {
    throw new Error(`missing ${fieldName}`);
  }
  return value;
}

function requiredValue<T>(value: T | undefined | null, fieldName: string): T {
  if (value === undefined || value === null) {
    throw new Error(`missing ${fieldName}`);
  }
  return value;
}

