// ---------------------------------------------------------------------------
// Solana proof helpers
//
// Builds gateway-backed input proofs for Solana host contracts.
// Talks to the relayer /v2/input-proof endpoint, verifies coprocessor
// signatures, and returns the encoded proof bytes + handle.
// ---------------------------------------------------------------------------

import { ethers } from 'ethers';
// keccak ships without type declarations — cast to a minimal fluent interface
// eslint-disable-next-line @typescript-eslint/no-require-imports
const createHash = require('keccak') as (algorithm: string) => KeccakHasher;
interface KeccakHasher {
  update(data: Buffer | Uint8Array | string): KeccakHasher;
  digest(): Buffer;
}
import TFHE from 'node-tfhe';

import {
  SERIALIZED_SIZE_LIMIT_CIPHERTEXT,
  SERIALIZED_SIZE_LIMIT_PK,
  SERIALIZED_SIZE_LIMIT_CRS,
  RAW_CT_HASH_DOMAIN_SEPARATOR,
  HANDLE_HASH_DOMAIN_SEPARATOR,
  INPUT_PROOF_EXTRA_DATA_VERSION,
  INPUT_ENCRYPTION_TYPES,
  MAX_UINT64,
} from './solana.config';

// ---------------------------------------------------------------------------
// Shared low-level utilities (also imported by solana.relayer-helpers.ts)
// ---------------------------------------------------------------------------

export function hexToBytes(hexValue: string): Uint8Array {
  const normalized = hexValue.startsWith('0x') ? hexValue.slice(2) : hexValue;
  if (normalized.length % 2 !== 0) {
    throw new Error(`hex string must have even length: ${hexValue}`);
  }
  return Uint8Array.from(Buffer.from(normalized, 'hex'));
}

export function bytesToHex(bytes: Uint8Array | Buffer, withPrefix = false): string {
  return `${withPrefix ? '0x' : ''}${Buffer.from(bytes).toString('hex')}`;
}

export function stripHexPrefix(value: unknown): string {
  return String(value).replace(/^0x/i, '');
}

export function ensure0xHex(value: unknown): string {
  const normalized = String(value);
  return normalized.startsWith('0x') ? normalized : `0x${normalized}`;
}

export function cleanUrl(url: string): string {
  const normalized = String(url);
  return normalized.endsWith('/') ? normalized.slice(0, -1) : normalized;
}

export function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

export function normalizeStackHttpUrl(url: string): URL {
  const parsed = new URL(url);
  if (
    parsed.hostname === '0.0.0.0' ||
    parsed.hostname === 'minio' ||
    parsed.hostname === 'gateway-node' ||
    parsed.hostname === 'host-node' ||
    parsed.hostname === 'host-node-solana'
  ) {
    parsed.hostname = '127.0.0.1';
  }
  return parsed;
}

export async function fetchJsonExpectOk(url: string, init?: RequestInit): Promise<unknown> {
  const response = await fetch(url, init);
  const text = await response.text();
  if (!response.ok) {
    throw new Error(`request failed for ${url}: ${response.status} ${text}`);
  }
  return JSON.parse(text);
}

export async function fetchBinaryExpectOk(url: string): Promise<Uint8Array> {
  const response = await fetch(url);
  if (!response.ok) {
    throw new Error(`binary download failed for ${url}: ${response.status}`);
  }
  return new Uint8Array(await response.arrayBuffer());
}

// ---------------------------------------------------------------------------
// Coprocessor signer loading
// ---------------------------------------------------------------------------

export function loadCoprocessorSigners(envValues: Record<string, string>): string[] {
  const count = Number(requiredEnvValue(envValues, 'NUM_COPROCESSORS', 'Solana proof builder'));
  const signers: string[] = [];
  for (let index = 0; index < count; index += 1) {
    signers.push(
      ethers.getAddress(
        requiredEnvValue(
          envValues,
          `COPROCESSOR_SIGNER_ADDRESS_${index}`,
          'Solana proof builder',
        ),
      ),
    );
  }
  return signers;
}

// ---------------------------------------------------------------------------
// Solana contract identity helpers
// ---------------------------------------------------------------------------

export function normalizeIdentityHex(value: string | null | undefined): string | null {
  if (!value) {
    return null;
  }
  const normalized = ensure0xHex(value);
  const bytes = hexToBytes(normalized);
  if (bytes.length !== 32) {
    throw new Error(`expected 32-byte host identity, got ${bytes.length} bytes`);
  }
  return normalized.toLowerCase();
}

export function buildSolanaContractIdentityMap(
  identities: Record<string, unknown>,
): Map<string, string | null> {
  const entries = [
    [
      identities?.test_input_contract_evm_address as string | undefined,
      identities?.test_input_state_pda_hex as string | undefined,
    ],
    [
      identities?.confidential_token_contract_evm_address as string | undefined,
      identities?.confidential_token_state_pda_hex as string | undefined,
    ],
  ].filter(([address, contractIdentity]) => address && contractIdentity) as [string, string][];
  return new Map(
    entries.map(([address, contractIdentity]) => [
      ethers.getAddress(address),
      normalizeIdentityHex(`0x${stripHexPrefix(contractIdentity)}`),
    ]),
  );
}

export function getSolanaContractIdentityByAddress(
  identityMap: Map<string, string | null> | undefined,
  address: string | undefined,
): string | undefined {
  if (!identityMap || !address) {
    return undefined;
  }
  return identityMap.get(ethers.getAddress(address)) ?? undefined;
}

// ---------------------------------------------------------------------------
// Input proof construction
// ---------------------------------------------------------------------------

export interface BuildInputProofParams {
  relayerUrl: string;
  aclAddress: string | null;
  inputVerificationAddress: string;
  gatewayChainId: string | number;
  hostChainId: string | number;
  contractAddress: string;
  userAddress: string;
  contractIdentity?: string | null;
  userIdentity?: string | null;
  aclIdentity?: string | null;
  value: string;
  type: string;
  coprocessorSigners: string[];
  coprocessorThreshold: string | number;
}

export interface InputProofBundle {
  handles: string[];
  selectedHandle: string;
  inputProof: string;
  jobId: string;
}

export async function buildGatewayBackedInputProof(
  params: BuildInputProofParams,
): Promise<InputProofBundle> {
  return buildGatewayBackedInputProofInline({
    ...params,
    gatewayChainId: Number(params.gatewayChainId),
    hostChainId: Number(params.hostChainId),
    value: String(params.value),
    coprocessorThreshold: Number(params.coprocessorThreshold),
  });
}

async function buildGatewayBackedInputProofInline({
  relayerUrl,
  aclAddress,
  inputVerificationAddress,
  gatewayChainId,
  hostChainId,
  contractAddress,
  userAddress,
  contractIdentity = null,
  userIdentity = null,
  aclIdentity = null,
  value,
  type,
  coprocessorSigners,
  coprocessorThreshold,
}: {
  relayerUrl: string;
  aclAddress: string | null;
  inputVerificationAddress: string;
  gatewayChainId: number;
  hostChainId: number;
  contractAddress: string;
  userAddress: string;
  contractIdentity?: string | null;
  userIdentity?: string | null;
  aclIdentity?: string | null;
  value: string;
  type: string;
  coprocessorSigners: string[];
  coprocessorThreshold: number;
}): Promise<InputProofBundle> {
  const normalizedRelayerUrl = cleanUrl(relayerUrl);
  const normalizedContractIdentity = normalizeIdentityHex(contractIdentity);
  const normalizedUserIdentity = normalizeIdentityHex(userIdentity);
  const normalizedAclIdentity = normalizeIdentityHex(aclIdentity);
  const normalizedAclAddress = normalizedAclIdentity
    ? null
    : ethers.getAddress(requiredString(aclAddress, 'aclAddress'));
  const normalizedInputVerificationAddress = ethers.getAddress(inputVerificationAddress);
  const normalizedContractAddress = ethers.getAddress(
    requiredString(contractAddress, 'contractAddress'),
  );
  const normalizedUserAddress = ethers.getAddress(requiredString(userAddress, 'userAddress'));
  const inputProofExtraData =
    normalizedContractIdentity && normalizedUserIdentity
      ? buildInputProofExtraData(normalizedContractIdentity, normalizedUserIdentity)
      : '0x00';

  if (!Number.isFinite(gatewayChainId) || gatewayChainId <= 0) {
    throw new Error(`invalid gateway chain id: ${gatewayChainId}`);
  }
  if (!Number.isFinite(hostChainId) || hostChainId <= 0) {
    throw new Error(`invalid host chain id: ${hostChainId}`);
  }
  if (!Number.isFinite(coprocessorThreshold) || coprocessorThreshold <= 0) {
    throw new Error(`invalid coprocessor threshold: ${coprocessorThreshold}`);
  }
  if (!Array.isArray(coprocessorSigners) || coprocessorSigners.length === 0) {
    throw new Error('at least one coprocessor signer is required');
  }

  const keyMaterial = await getGatewayInputProofKeys(normalizedRelayerUrl);
  const { ciphertext, bits } = encryptGatewayInputValue({
    aclAddress: normalizedAclAddress,
    aclIdentity: normalizedAclIdentity,
    hostChainId,
    tfheCompactPublicKey: keyMaterial.publicKey,
    publicParams: keyMaterial.publicParams,
    contractAddress: normalizedContractAddress,
    userAddress: normalizedUserAddress,
    contractIdentity: normalizedContractIdentity,
    userIdentity: normalizedUserIdentity,
    inputType: type,
    value,
  });

  const payload = {
    ciphertextWithInputVerification: bytesToHex(ciphertext),
    contractChainId: `0x${hostChainId.toString(16)}`,
    extraData: inputProofExtraData,
    contractAddress: normalizedContractAddress,
    userAddress: normalizedUserAddress,
    ...(normalizedContractIdentity ? { contractId: normalizedContractIdentity } : {}),
    ...(normalizedUserIdentity ? { userId: normalizedUserIdentity } : {}),
  };

  const postPayload = await fetchJsonExpectOk(`${normalizedRelayerUrl}/v2/input-proof`, {
    method: 'POST',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify(payload),
  }) as Record<string, unknown>;
  const jobId = (postPayload?.result as Record<string, unknown>)?.jobId as string | undefined;
  if (!jobId) {
    throw new Error(
      `relayer input-proof response did not return a jobId: ${JSON.stringify(postPayload)}`,
    );
  }

  const statusPayload = await waitForGatewayInputProofJob(normalizedRelayerUrl, jobId) as Record<string, unknown>;
  const statusResult = statusPayload?.result as Record<string, unknown> | undefined;
  if (statusPayload?.status !== 'succeeded' || !statusResult?.accepted) {
    throw new Error(`input-proof job did not succeed: ${JSON.stringify(statusPayload)}`);
  }

  const handles = computeGatewayInputHandles(
    ciphertext,
    bits,
    (normalizedAclIdentity ?? normalizedAclAddress) as string,
    hostChainId,
    currentCiphertextVersion(),
  );
  const responseHandles = ((statusResult.handles ?? []) as string[]).map((handle) =>
    hexToBytes(handle),
  );
  assertGatewayHandleListsMatch(handles, responseHandles);

  const signatures = (statusResult.signatures ?? []) as string[];
  verifyGatewayInputProofSignatures({
    handles,
    signatures,
    userAddress: normalizedUserAddress,
    contractAddress: normalizedContractAddress,
    contractIdentity: normalizedContractIdentity,
    userIdentity: normalizedUserIdentity,
    hostChainId,
    extraData: inputProofExtraData,
    gatewayChainId,
    inputVerificationAddress: normalizedInputVerificationAddress,
    coprocessorSigners,
    threshold: coprocessorThreshold,
  });

  let inputProofHex = encodeSingleByte(handles.length);
  inputProofHex += encodeSingleByte(signatures.length);
  for (const handle of handles) {
    inputProofHex += bytesToHex(handle);
  }
  for (const signature of signatures) {
    inputProofHex += stripHexPrefix(signature);
  }
  inputProofHex += stripHexPrefix(inputProofExtraData);

  return {
    handles: handles.map((handle) => bytesToHex(handle, true)),
    selectedHandle: bytesToHex(handles[0], true),
    inputProof: `0x${inputProofHex}`,
    jobId,
  };
}

async function getGatewayInputProofKeys(relayerUrl: string) {
  const data = await fetchJsonExpectOk(`${relayerUrl}/v2/keyurl`) as Record<string, unknown>;
  const response = data?.response as Record<string, unknown> | undefined;
  const fheKeyInfo = response?.fheKeyInfo ?? response?.fhe_key_info;
  const publicKeyRecord = (fheKeyInfo as Array<Record<string, unknown>>)?.[0]?.fhePublicKey ??
    (fheKeyInfo as Array<Record<string, unknown>>)?.[0]?.fhe_public_key;
  const crsRecord = (response?.crs as Record<string, unknown>)?.['2048'];
  const publicKeyUrl = normalizeGatewayInputDownloadUrl(
    (publicKeyRecord as Record<string, unknown>)?.urls?.[0] as string | undefined,
  );
  const publicKeyId = (publicKeyRecord as Record<string, unknown>)?.dataId ??
    (publicKeyRecord as Record<string, unknown>)?.data_id;
  const publicParamsUrl = normalizeGatewayInputDownloadUrl(
    (crsRecord as Record<string, unknown>)?.urls?.[0] as string | undefined,
  );
  const publicParamsId = (crsRecord as Record<string, unknown>)?.dataId ??
    (crsRecord as Record<string, unknown>)?.data_id;

  if (!publicKeyUrl || !publicKeyId || !publicParamsUrl || !publicParamsId) {
    throw new Error(`unexpected /v2/keyurl response: ${JSON.stringify(data)}`);
  }

  const [publicKeyBytes, publicParamsBytes] = await Promise.all([
    fetchBinaryExpectOk(publicKeyUrl),
    fetchBinaryExpectOk(publicParamsUrl),
  ]);

  return {
    publicKey: TFHE.TfheCompactPublicKey.safe_deserialize(
      publicKeyBytes,
      SERIALIZED_SIZE_LIMIT_PK,
    ),
    publicParams: {
      2048: {
        publicParams: TFHE.CompactPkeCrs.safe_deserialize(
          publicParamsBytes,
          SERIALIZED_SIZE_LIMIT_CRS,
        ),
        publicParamsId,
      },
    },
    publicKeyId,
  };
}

function encryptGatewayInputValue({
  aclAddress,
  aclIdentity,
  hostChainId,
  tfheCompactPublicKey,
  publicParams,
  contractAddress,
  userAddress,
  contractIdentity,
  userIdentity,
  inputType,
  value,
}: {
  aclAddress: string | null;
  aclIdentity: string | null;
  hostChainId: number;
  tfheCompactPublicKey: ReturnType<typeof TFHE.TfheCompactPublicKey.safe_deserialize>;
  publicParams: Record<number, { publicParams: ReturnType<typeof TFHE.CompactPkeCrs.safe_deserialize>; publicParamsId: unknown }>;
  contractAddress: string;
  userAddress: string;
  contractIdentity: string | null;
  userIdentity: string | null;
  inputType: string;
  value: string;
}): { ciphertext: Uint8Array; bits: number[] } {
  const builder = TFHE.CompactCiphertextList.builder(tfheCompactPublicKey);
  const bits: number[] = [];
  addGatewayEncryptedValue(builder, bits, inputType, value);

  const totalBits = bits.reduce((sum, bitWidth) => sum + bitWidth, 0);
  if (totalBits > 2048) {
    throw new Error(`too many encrypted bits for one input proof: ${totalBits}`);
  }

  const auxData =
    contractIdentity && userIdentity && aclIdentity
      ? buildGatewayInputAuxDataV1(contractIdentity, userIdentity, aclIdentity, hostChainId)
      : buildGatewayInputAuxData(contractAddress, userAddress, aclAddress as string, hostChainId);
  const encrypted = builder.build_with_proof_packed(
    publicParams[2048].publicParams,
    auxData,
    TFHE.ZkComputeLoad.Verify,
  );

  return {
    ciphertext: encrypted.safe_serialize(SERIALIZED_SIZE_LIMIT_CIPHERTEXT),
    bits,
  };
}

function addGatewayEncryptedValue(
  builder: ReturnType<typeof TFHE.CompactCiphertextList.builder>,
  bits: number[],
  inputType: string,
  rawValue: string,
): void {
  switch (inputType) {
    case 'bool': {
      const value =
        rawValue === 'true' ? true : rawValue === 'false' ? false : BigInt(rawValue) === 1n;
      builder.push_boolean(value);
      bits.push(2);
      return;
    }
    case 'uint8':
      assertGatewayUintRange(rawValue, 8);
      builder.push_u8(Number(rawValue));
      bits.push(8);
      return;
    case 'uint16':
      assertGatewayUintRange(rawValue, 16);
      builder.push_u16(Number(rawValue));
      bits.push(16);
      return;
    case 'uint32':
      assertGatewayUintRange(rawValue, 32);
      builder.push_u32(Number(rawValue));
      bits.push(32);
      return;
    case 'uint64':
      assertGatewayUintRange(rawValue, 64);
      builder.push_u64(BigInt(rawValue));
      bits.push(64);
      return;
    case 'uint128':
      assertGatewayUintRange(rawValue, 128);
      builder.push_u128(BigInt(rawValue));
      bits.push(128);
      return;
    case 'address': {
      const normalized = ethers.getAddress(rawValue);
      builder.push_u160(BigInt(normalized));
      bits.push(160);
      return;
    }
    case 'uint256':
      assertGatewayUintRange(rawValue, 256);
      builder.push_u256(BigInt(rawValue));
      bits.push(256);
      return;
    default:
      throw new Error(`unsupported gateway input type ${inputType}`);
  }
}

function assertGatewayUintRange(rawValue: string, width: number): void {
  const value = BigInt(rawValue);
  if (value < 0n || value >= 1n << BigInt(width)) {
    throw new Error(`value ${rawValue} does not fit in ${width} bits`);
  }
}

function buildGatewayInputAuxData(
  contractAddress: string,
  userAddress: string,
  aclAddress: string,
  hostChainId: number,
): Uint8Array {
  const contractBytes = hexToBytes(contractAddress);
  const userBytes = hexToBytes(userAddress);
  const aclBytes = hexToBytes(aclAddress);
  const chainBytes = Uint8Array.from(
    Buffer.from(hostChainId.toString(16).padStart(64, '0'), 'hex'),
  );
  const auxData = new Uint8Array(
    contractBytes.length + userBytes.length + aclBytes.length + chainBytes.length,
  );
  auxData.set(contractBytes, 0);
  auxData.set(userBytes, 20);
  auxData.set(aclBytes, 40);
  auxData.set(chainBytes, auxData.length - chainBytes.length);
  return auxData;
}

function buildGatewayInputAuxDataV1(
  contractIdentity: string,
  userIdentity: string,
  aclIdentity: string,
  hostChainId: number,
): Uint8Array {
  const contractBytes = hexToBytes(contractIdentity);
  const userBytes = hexToBytes(userIdentity);
  const aclBytes = hexToBytes(aclIdentity);
  const chainBytes = Uint8Array.from(
    Buffer.from(hostChainId.toString(16).padStart(64, '0'), 'hex'),
  );
  const auxData = new Uint8Array(
    contractBytes.length + userBytes.length + aclBytes.length + chainBytes.length,
  );
  auxData.set(contractBytes, 0);
  auxData.set(userBytes, 32);
  auxData.set(aclBytes, 64);
  auxData.set(chainBytes, 96);
  return auxData;
}

export function buildInputProofExtraData(
  contractIdentity: string,
  userIdentity: string,
): string {
  return `0x${Buffer.concat([
    Buffer.from([INPUT_PROOF_EXTRA_DATA_VERSION]),
    Buffer.from(hexToBytes(contractIdentity)),
    Buffer.from(hexToBytes(userIdentity)),
  ]).toString('hex')}`;
}

function currentCiphertextVersion(): number {
  return 0;
}

function computeGatewayInputHandles(
  ciphertextWithZKProof: Uint8Array,
  bitwidths: number[],
  aclAddress: string,
  chainId: number,
  ciphertextVersion: number,
): Uint8Array[] {
  const blobHash = createHash('keccak256')
    .update(Buffer.from(RAW_CT_HASH_DOMAIN_SEPARATOR))
    .update(Buffer.from(ciphertextWithZKProof))
    .digest();
  const aclAddressBytes = Buffer.from(hexToBytes(aclAddress));
  const chainIdHex = chainId.toString(16).padStart(64, '0');
  const chainIdBytes = Buffer.from(chainIdHex, 'hex');

  return bitwidths.map((bitwidth, encryptionIndex) => {
    const encryptionType = INPUT_ENCRYPTION_TYPES[bitwidth];
    const handleHash = createHash('keccak256')
      .update(Buffer.from(HANDLE_HASH_DOMAIN_SEPARATOR))
      .update(blobHash)
      .update(Buffer.from([encryptionIndex]))
      .update(aclAddressBytes)
      .update(chainIdBytes)
      .digest();
    const handle = new Uint8Array(32);
    handle.set(handleHash, 0);
    if (BigInt(chainId) > MAX_UINT64) {
      throw new Error(`chain id ${chainId} exceeds the supported 8-byte handle field`);
    }
    handle[21] = encryptionIndex;
    handle.set(hexToBytes(chainIdHex).slice(24, 32), 22);
    handle[30] = encryptionType;
    handle[31] = ciphertextVersion;
    return handle;
  });
}

function assertGatewayHandleListsMatch(
  expectedHandles: Uint8Array[],
  responseHandles: Uint8Array[],
): void {
  if (expectedHandles.length !== responseHandles.length) {
    throw new Error(
      `relayer returned ${responseHandles.length} handles, expected ${expectedHandles.length}`,
    );
  }
  for (let index = 0; index < expectedHandles.length; index += 1) {
    const expected = bytesToHex(expectedHandles[index], true);
    const actual = bytesToHex(responseHandles[index], true);
    if (expected !== actual) {
      throw new Error(`handle mismatch at index ${index}: expected ${expected}, got ${actual}`);
    }
  }
}

function verifyGatewayInputProofSignatures({
  handles,
  signatures,
  userAddress,
  contractAddress,
  contractIdentity,
  userIdentity,
  hostChainId,
  extraData,
  gatewayChainId,
  inputVerificationAddress,
  coprocessorSigners,
  threshold,
}: {
  handles: Uint8Array[];
  signatures: string[];
  userAddress: string;
  contractAddress: string;
  contractIdentity: string | null;
  userIdentity: string | null;
  hostChainId: number;
  extraData: string;
  gatewayChainId: number;
  inputVerificationAddress: string;
  coprocessorSigners: string[];
  threshold: number;
}): void {
  const domain = {
    name: 'InputVerification',
    version: '1',
    chainId: gatewayChainId,
    verifyingContract: inputVerificationAddress,
  };
  const types = {
    CiphertextVerification: [
      { name: 'ctHandles', type: 'bytes32[]' },
      { name: 'userAddress', type: 'address' },
      { name: 'contractAddress', type: 'address' },
      { name: 'contractChainId', type: 'uint256' },
      { name: 'extraData', type: 'bytes' },
    ],
  };
  const recoveredAddresses = signatures.map((signature) =>
    ethers.verifyTypedData(
      domain,
      types,
      {
        ctHandles: handles,
        userAddress,
        contractAddress,
        contractChainId: hostChainId,
        extraData,
      },
      ensure0xHex(signature),
    ),
  );

  const normalizedExpected = new Set(coprocessorSigners.map((address) => ethers.getAddress(address)));
  const normalizedRecovered = recoveredAddresses.map((address) => ethers.getAddress(address));
  const uniqueRecovered = new Set(normalizedRecovered);
  if (uniqueRecovered.size !== normalizedRecovered.length) {
    throw new Error('duplicate coprocessor signer recovered from input-proof response');
  }
  for (const address of normalizedRecovered) {
    if (!normalizedExpected.has(address)) {
      throw new Error(`unexpected coprocessor signer in input-proof response: ${address}`);
    }
  }
  if (normalizedRecovered.length < threshold) {
    throw new Error(
      `coprocessor signer threshold not reached: got ${normalizedRecovered.length}, need ${threshold}`,
    );
  }
}

function normalizeGatewayInputDownloadUrl(url: string | undefined): string | undefined {
  if (!url) {
    return url;
  }
  const parsed = normalizeStackHttpUrl(url);
  return parsed.toString();
}

async function waitForGatewayInputProofJob(relayerUrl: string, jobId: string): Promise<unknown> {
  const started = Date.now();
  while (Date.now() - started < 120_000) {
    const response = await fetch(`${relayerUrl}/v2/input-proof/${jobId}`);
    const text = await response.text();
    if (!response.ok) {
      throw new Error(`input-proof status failed for ${jobId}: ${response.status} ${text}`);
    }
    const payload = JSON.parse(text);
    if (payload.status === 'queued') {
      await sleep(1000);
      continue;
    }
    return payload;
  }
  throw new Error(`timed out waiting for input-proof job ${jobId}`);
}

function encodeSingleByte(value: number): string {
  const hex = Number(value).toString(16);
  return hex.length % 2 === 0 ? hex : `0${hex}`;
}

// ---------------------------------------------------------------------------
// Internal utilities
// ---------------------------------------------------------------------------

function requiredString(value: unknown, fieldName: string): string {
  if (typeof value !== 'string' || value.length === 0) {
    throw new Error(`missing ${fieldName}`);
  }
  return value;
}

export function requiredEnvValue(values: Record<string, string>, field: string, label: string): string {
  const value = values?.[field];
  if (typeof value !== 'string' || value.length === 0) {
    throw new Error(`${label}: missing ${field}`);
  }
  return value;
}
