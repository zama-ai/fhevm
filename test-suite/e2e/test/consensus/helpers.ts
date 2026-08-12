/**
 * Reusable evidence collectors for ciphertext byte-consensus tests.
 *
 * The worker persists an operation's output bytes in `ciphertexts` and its
 * provenance in `computations` (transaction id, host chain, block number).
 * The transaction is the materialization boundary, so the oracle is organised
 * per transaction: every produced output must be a canonical, completed
 * computation, whole transactions must be completely executed, and the
 * persisted bytes and digests must be byte-identical between coprocessors
 * running the same software and backend/hardware class.
 */
import { execFile as oldExecFile } from 'child_process';
import { ethers } from 'ethers';
import { Pool } from 'pg';
import { promisify } from 'util';

const execFile = promisify(oldExecFile);

const CIPHERTEXT_COMMITS_ABI = [
  'event AddCiphertextMaterial(bytes32 indexed ctHandle, uint256 keyId, bytes32 ciphertextDigest, bytes32 snsCiphertextDigest, address coprocessorTxSender)',
  'event AddCiphertextMaterialConsensus(bytes32 indexed ctHandle, uint256 keyId, bytes32 ciphertextDigest, bytes32 snsCiphertextDigest, address[] coprocessorTxSenders)',
];

const GATEWAY_CONFIG_ABI = [
  'function getCoprocessorTxSenders() view returns (address[])',
  'function getCoprocessor(address) view returns (tuple(address txSenderAddress,address signerAddress,string s3BucketUrl))',
];

const DEFAULT_KMS_ATTESTATION_PROBE_CONTAINER = 'kms-connector-kms-worker';
const DEFAULT_KMS_ATTESTATION_PROBE_IMAGE = 'ghcr.io/zama-ai/fhevm/test-suite/e2e:fhevm-local';
const ATTESTATION_HEADER = 'x-amz-meta-ct-attestation';
// RFC-023 fixes the KMS lookup context to U256::ONE.  It is part of the
// signature payload even though the S3 header deliberately omits it.
const COPROCESSOR_CONTEXT_ID = 1n;
const ATTESTATION_FORMATS = {
  uncompressed_on_cpu: 10,
  compressed_on_cpu: 11,
  uncompressed_on_gpu: 20,
  compressed_on_gpu: 21,
} as const;

export const DEFAULT_CIPHERTEXT_VERSION = 0;

export interface ConsensusEvent {
  ctHandle: string;
  keyId: bigint;
  ciphertextDigest: string;
  snsCiphertextDigest: string;
  senders: string[];
  blockNumber: number;
}

/** An on-chain Coprocessor bucket that the KMS worker will query for RFC-023 material. */
export interface CoprocessorAttestationBucket {
  txSender: string;
  signer: string;
  bucketUrl: string;
}

/** Canonical terminal-output material which every RFC-023 attestation must bind. */
export interface AttestationOutputEvidence {
  handle: string;
  keyId: string;
  ciphertextDigest: string;
  snsCiphertextDigest: string;
  /** Exact `ciphertext_digest.ciphertext128_format` value from canonical DB state. */
  ciphertext128Format: number;
}

/** JSON wire form of the `x-amz-meta-ct-attestation` response header. */
export interface CiphertextAttestationMetadata {
  version: 1;
  keyId: string;
  ciphertextDigest: string;
  snsCiphertextDigest: string;
  format: keyof typeof ATTESTATION_FORMATS;
  signer: string;
  signature: string;
}

export interface CanonicalOutputRow {
  handle: Buffer;
  ciphertext: Buffer;
  /**
   * The optional local SNS representation. The transaction sender removes it
   * only after its AddCiphertextMaterial transaction succeeds, so its absence
   * does not by itself mean that the SNS digest is invalid.
   */
  snsCiphertext: Buffer | null;
  ciphertextType: number;
  ciphertextVersion: number;
  fheOperation: number;
  /** The producing L1 transaction — the materialization boundary. */
  transactionId: Buffer;
  hostChainId: number;
  blockNumber: number;
  keyId: Buffer;
  ciphertextDigest: Buffer | null;
  snsCiphertextDigest: Buffer | null;
  /** `ciphertext_digest.ciphertext128_format`, bound by the RFC-023 format field. */
  ciphertext128Format: number;
}

/**
 * Completion evidence for one producing transaction: the worker acquires and
 * re-executes whole transactions, so at quiescence every computation row of a
 * fixture transaction must be completed and none may be an error.
 */
export interface TransactionCompletionRow {
  transactionId: Buffer;
  hostChainId: number;
  blockNumber: number;
  totalCount: number;
  completedCount: number;
  errorCount: number;
}

export interface ConsensusDatabaseReport {
  databaseUrl: string;
  outputs: CanonicalOutputRow[];
  transactions: TransactionCompletionRow[];
}

export interface CanonicalOutputOptions {
  /** Current workers write version zero. Keep this explicit so a future
   * format bump cannot silently compare a legacy ciphertext version. */
  ciphertextVersion?: number;
  /**
   * Exact number of producing transactions per handle (hex, lowercase) when
   * a fixture deliberately aliases an output across same-block transactions.
   * Handles not listed expect exactly one producer.
   */
  expectedProducers?: Record<string, number>;
}

type CanonicalOutputSqlRow = {
  handle: Buffer;
  ciphertext: Buffer;
  sns_ciphertext: Buffer | null;
  ciphertext_type: string | number;
  ciphertext_version: string | number;
  fhe_operation: string | number;
  transaction_id: Buffer;
  host_chain_id: string | number;
  block_number: string | number;
  key_id_gw: Buffer | null;
  ciphertext_digest: Buffer | null;
  sns_ciphertext_digest: Buffer | null;
  ciphertext128_format: string | number | null;
};

type TransactionCompletionSqlRow = {
  transaction_id: Buffer;
  host_chain_id: string | number;
  block_number: string | number;
  total_count: string | number;
  completed_count: string | number;
  error_count: string | number;
};

/**
 * A normal asynchronous state transition (work/digest not ready yet). The
 * polling helper retries only this class. A byte/provenance mismatch is a
 * completed-but-bad observation and must fail immediately rather than being
 * relabelled as a timeout after several minutes.
 */
class ConsensusStateIncomplete extends Error {}

export interface TransactionScope {
  transactionId: Buffer;
  hostChainId: number;
  blockNumber: number;
}

function bytes32ToBuffer(value: string): Buffer {
  if (!/^0x[0-9a-fA-F]{64}$/.test(value)) {
    throw new Error(`expected bytes32 hex value, got ${value}`);
  }
  return Buffer.from(value.slice(2), 'hex');
}

/**
 * The KMS connector's RFC-023 object layout.  Keep this test-side helper
 * deliberately literal: the gate must probe the same object, not merely a
 * bucket health endpoint that could be reachable while the output is absent.
 */
export function rfc023CiphertextUrl(bucketUrl: string, handle: string): string {
  if (!/^0x[0-9a-fA-F]{64}$/.test(handle)) {
    throw new Error(`expected bytes32 ciphertext handle, got ${handle}`);
  }

  let url: URL;
  try {
    url = new URL(bucketUrl);
  } catch {
    throw new Error(`invalid Coprocessor bucket URL ${bucketUrl}`);
  }
  if (
    (url.protocol !== 'http:' && url.protocol !== 'https:') ||
    url.username ||
    url.password ||
    url.search ||
    url.hash
  ) {
    throw new Error(`unsupported Coprocessor bucket URL ${bucketUrl}`);
  }

  // The connector deliberately appends directly to the registered string.
  // Preserve it byte-for-byte (including an accidental trailing slash) so the
  // E2E gate cannot validate a normalized URL different from KMS's real HEAD.
  // Layout per #3401: 64-bit ciphertext material lives under the ct128/
  // prefix, chunked; the first chunk existing is the readiness signal.
  return `${bucketUrl}/ct128/${handle.slice(2).toLowerCase()}/1`;
}

function requiredHex(value: unknown, bytes: number, field: string): string {
  if (typeof value !== 'string' || !ethers.isHexString(value, bytes)) {
    throw new Error(`${field} must be a ${bytes}-byte 0x-prefixed hex value`);
  }
  return value.toLowerCase();
}

/** serde serializes U256 as a minimally padded hexadecimal quantity. Normalize it for RFC-023's 32-byte payload. */
function requiredU256(value: unknown, field: string): string {
  if (typeof value !== 'string' || !/^0x[0-9a-fA-F]+$/.test(value)) {
    throw new Error(`${field} must be a 0x-prefixed unsigned hexadecimal quantity`);
  }
  try {
    return ethers.toBeHex(BigInt(value), 32).toLowerCase();
  } catch {
    throw new Error(`${field} does not fit in uint256`);
  }
}

function requiredAddress(value: unknown, field: string): string {
  if (typeof value !== 'string' || !ethers.isAddress(value)) {
    throw new Error(`${field} must be an EVM address`);
  }
  return ethers.getAddress(value);
}

/**
 * JSON.parse deliberately applies last-member-wins semantics. RFC-023 is
 * deserialized by Rust serde, which rejects a duplicate struct member, so
 * validate the JSON then scan the root wire object before JSON.parse can hide
 * the duplicate from the field validator.
 *
 * RFC-023's header is a flat struct. JSON.parse checks the complete grammar;
 * this small scanner only identifies the decoded names of its root members,
 * including names written with JSON escapes.
 */
function parseJsonObjectRejectingDuplicateMembers(value: string): Record<string, unknown> {
  let parsed: unknown;
  try {
    parsed = JSON.parse(value);
  } catch (error) {
    throw new Error(`attestation metadata is not valid JSON: ${(error as Error).message}`);
  }
  if (!parsed || typeof parsed !== 'object' || Array.isArray(parsed)) {
    throw new Error('attestation metadata must be a JSON object');
  }

  let cursor = 0;
  const skipWhitespace = (): void => {
    while (/[\t\n\r ]/.test(value[cursor] ?? '')) cursor += 1;
  };
  const parseString = (): string => {
    const start = cursor;
    cursor += 1; // The preceding JSON.parse guarantees this is an opening quote.
    let escaped = false;
    while (cursor < value.length) {
      const char = value[cursor++];
      if (escaped) {
        escaped = false;
      } else if (char === '\\') {
        escaped = true;
      } else if (char === '"') {
        return JSON.parse(value.slice(start, cursor)) as string;
      }
    }
    throw new Error('attestation metadata is not valid JSON: unterminated member name');
  };
  const skipMemberValue = (): void => {
    let nested = 0;
    let inString = false;
    let escaped = false;
    while (cursor < value.length) {
      const char = value[cursor];
      if (inString) {
        cursor += 1;
        if (escaped) escaped = false;
        else if (char === '\\') escaped = true;
        else if (char === '"') inString = false;
        continue;
      }
      if (char === '"') inString = true;
      else if (char === '{' || char === '[') nested += 1;
      else if (char === '}' || char === ']') {
        if (nested === 0) return;
        nested -= 1;
      } else if (char === ',' && nested === 0) return;
      cursor += 1;
    }
  };

  skipWhitespace();
  cursor += 1; // Root is known to be a JSON object after JSON.parse above.
  const members = new Set<string>();
  while (true) {
    skipWhitespace();
    if (value[cursor] === '}') return parsed as Record<string, unknown>;
    const member = parseString();
    if (members.has(member)) {
      throw new Error(`attestation metadata has duplicate object member ${JSON.stringify(member)}`);
    }
    members.add(member);
    skipWhitespace();
    cursor += 1; // Root JSON validity guarantees the colon.
    skipMemberValue();
    if (value[cursor] === '}') return parsed as Record<string, unknown>;
    cursor += 1; // Root JSON validity guarantees the comma.
  }
}

/** Parses and validates the RFC-023 JSON wire shape before any decrypt request is emitted. */
export function parseCiphertextAttestationMetadata(value: string): CiphertextAttestationMetadata {
  const metadata = parseJsonObjectRejectingDuplicateMembers(value);
  if (metadata.version !== 1) {
    throw new Error(`unsupported attestation version ${String(metadata.version)}`);
  }
  if (
    typeof metadata.format !== 'string' ||
    !Object.prototype.hasOwnProperty.call(ATTESTATION_FORMATS, metadata.format)
  ) {
    throw new Error(`unsupported attestation format ${String(metadata.format)}`);
  }
  return {
    version: 1,
    keyId: requiredU256(metadata.key_id, 'attestation key_id'),
    ciphertextDigest: requiredHex(metadata.ciphertext_digest, 32, 'attestation ciphertext_digest'),
    snsCiphertextDigest: requiredHex(metadata.sns_ciphertext_digest, 32, 'attestation sns_ciphertext_digest'),
    format: metadata.format as keyof typeof ATTESTATION_FORMATS,
    signer: requiredAddress(metadata.signer, 'attestation signer'),
    signature: requiredHex(metadata.signature, 65, 'attestation signature'),
  };
}

/** Extracts exactly one attestation metadata value from the wget response headers. */
export function attestationMetadataFromWgetHeaders(headers: string): CiphertextAttestationMetadata {
  const prefix = `${ATTESTATION_HEADER}:`;
  const values = headers
    .split(/\r?\n/)
    .map((line) => line.trimStart())
    .filter((line) => line.toLowerCase().startsWith(prefix))
    .map((line) => line.slice(prefix.length).trim());
  if (values.length !== 1) {
    throw new Error(`expected exactly one ${ATTESTATION_HEADER} response header, got ${values.length}`);
  }
  if (!values[0]) throw new Error(`${ATTESTATION_HEADER} response header is empty`);
  return parseCiphertextAttestationMetadata(values[0]);
}

/**
 * RFC-023's raw-signature prehash.  `handle` and context are absent from the
 * S3 JSON header, so recovering this signature is the only way the readiness
 * gate can prove the metadata belongs to the terminal object it just HEADed.
 */
export function rfc023AttestationPrehash(
  metadata: Pick<
    CiphertextAttestationMetadata,
    'version' | 'keyId' | 'ciphertextDigest' | 'snsCiphertextDigest' | 'format'
  >,
  handle: string,
  contextId = COPROCESSOR_CONTEXT_ID,
): string {
  const format = ATTESTATION_FORMATS[metadata.format];
  if (format === undefined) throw new Error(`unsupported attestation format ${metadata.format}`);
  const canonicalBytes = ethers.concat([
    ethers.toUtf8Bytes('FHEVMCTA'),
    ethers.toBeHex(metadata.version, 1),
    requiredHex(handle, 32, 'attestation handle'),
    requiredHex(metadata.keyId, 32, 'attestation key_id'),
    ethers.toBeHex(contextId, 32),
    requiredHex(metadata.ciphertextDigest, 32, 'attestation ciphertext_digest'),
    requiredHex(metadata.snsCiphertextDigest, 32, 'attestation sns_ciphertext_digest'),
    ethers.toBeHex(format, 1),
  ]);
  return ethers.keccak256(canonicalBytes);
}

/** Builds immutable digest/key evidence from the already-validated canonical database report. */
export function attestationEvidenceFromCanonicalOutput(output: CanonicalOutputRow): AttestationOutputEvidence {
  if (output.handle.length !== 32)
    throw new Error(`canonical output handle must be 32 bytes, got ${output.handle.length}`);
  if (output.keyId.length !== 32)
    throw new Error(`canonical output key id must be 32 bytes, got ${output.keyId.length}`);
  if (!Object.values(ATTESTATION_FORMATS).some((format) => format === output.ciphertext128Format)) {
    throw new Error(
      `canonical output ciphertext_digest.ciphertext128_format ${output.ciphertext128Format} is not an RFC-023 format`,
    );
  }
  return {
    handle: bufferHex(output.handle),
    keyId: bufferHex(output.keyId),
    ciphertextDigest: bufferHex(requireDigest(output.ciphertextDigest, 'canonical output')),
    snsCiphertextDigest: bufferHex(requireDigest(output.snsCiphertextDigest, 'canonical SNS output')),
    ciphertext128Format: output.ciphertext128Format,
  };
}

/**
 * Binds a fetched metadata header to both the canonical terminal output and
 * the on-chain Coprocessor registration.  Signature recovery proves the
 * otherwise wire-omitted `(handle, context_id)` pair; the field checks prove
 * that the object carries the same key and material digests as Gateway quorum.
 */
export function assertAttestationMatchesOutputEvidence(
  metadata: CiphertextAttestationMetadata,
  evidence: AttestationOutputEvidence,
  bucket: CoprocessorAttestationBucket,
): void {
  const expectedKeyId = requiredHex(evidence.keyId, 32, 'expected key_id');
  const expectedCiphertextDigest = requiredHex(evidence.ciphertextDigest, 32, 'expected ciphertext_digest');
  const expectedSnsCiphertextDigest = requiredHex(evidence.snsCiphertextDigest, 32, 'expected sns_ciphertext_digest');
  if (metadata.keyId !== expectedKeyId) {
    throw new Error(`attestation key_id ${metadata.keyId} does not match canonical output ${expectedKeyId}`);
  }
  if (metadata.ciphertextDigest !== expectedCiphertextDigest) {
    throw new Error('attestation ciphertext_digest does not match canonical output');
  }
  if (metadata.snsCiphertextDigest !== expectedSnsCiphertextDigest) {
    throw new Error('attestation sns_ciphertext_digest does not match canonical output');
  }
  if (ATTESTATION_FORMATS[metadata.format] !== evidence.ciphertext128Format) {
    throw new Error(
      `attestation format ${metadata.format} does not match canonical ciphertext_digest.ciphertext128_format ${evidence.ciphertext128Format}`,
    );
  }

  const expectedSigner = requiredAddress(bucket.signer, 'registered Coprocessor signer');
  const prehash = rfc023AttestationPrehash(metadata, evidence.handle);
  let recovered: string;
  try {
    recovered = ethers.getAddress(ethers.recoverAddress(prehash, metadata.signature));
  } catch (error) {
    throw new Error(`attestation signature cannot recover the terminal handle/context: ${(error as Error).message}`);
  }
  if (recovered !== metadata.signer) {
    throw new Error(`attestation signature recovered ${recovered}, not advertised signer ${metadata.signer}`);
  }
  if (metadata.signer !== expectedSigner) {
    throw new Error(
      `attestation signer ${metadata.signer} does not match registered Coprocessor ${bucket.txSender} signer ${expectedSigner}`,
    );
  }
}

/**
 * Resolve precisely the bucket set the KMS connector discovers from
 * GatewayConfig.  The connector's own S3_CONFIG is its KMS vault, not the
 * attested-ciphertext source, so using that configuration here would test the
 * wrong storage path.
 */
export async function getRegisteredCoprocessorBuckets(
  gatewayRpcUrl: string,
  gatewayConfigAddress: string,
): Promise<CoprocessorAttestationBucket[]> {
  const provider = new ethers.JsonRpcProvider(gatewayRpcUrl);
  const contract = new ethers.Contract(gatewayConfigAddress, GATEWAY_CONFIG_ABI, provider);
  try {
    const txSenders = (await contract.getCoprocessorTxSenders()) as string[];
    if (txSenders.length === 0) throw new Error('GatewayConfig has no registered Coprocessor transaction senders');

    return await Promise.all(
      txSenders.map(async (txSender) => {
        const coprocessor = (await contract.getCoprocessor(txSender)) as {
          txSenderAddress?: unknown;
          signerAddress?: unknown;
          s3BucketUrl?: unknown;
        };
        const registeredTxSender = requiredAddress(coprocessor.txSenderAddress, 'GatewayConfig Coprocessor tx sender');
        if (registeredTxSender !== ethers.getAddress(txSender)) {
          throw new Error(`GatewayConfig Coprocessor record returned mismatched tx sender ${registeredTxSender}`);
        }
        if (typeof coprocessor.s3BucketUrl !== 'string' || coprocessor.s3BucketUrl.length === 0) {
          throw new Error(`GatewayConfig Coprocessor ${txSender} has no S3 bucket URL`);
        }
        // Validate the URL once while resolving the registry.  The generated
        // RFC-023 path is included in the failure log below.
        rfc023CiphertextUrl(coprocessor.s3BucketUrl, ethers.ZeroHash);
        return {
          txSender: registeredTxSender,
          signer: requiredAddress(coprocessor.signerAddress, `GatewayConfig Coprocessor ${txSender} signer`),
          bucketUrl: coprocessor.s3BucketUrl,
        };
      }),
    );
  } finally {
    provider.destroy();
  }
}

/** Arguments for a no-shell HEAD probe in the KMS worker's network namespace. */
export function kmsNamespaceAttestationHeadArgs(
  workerContainer: string,
  probeImage: string,
  attestationUrl: string,
): string[] {
  return [
    'run',
    '--rm',
    '--network',
    `container:${validatedContainerName(workerContainer)}`,
    '--entrypoint',
    '/usr/bin/wget',
    probeImage,
    '--server-response',
    '--spider',
    '--timeout=5',
    attestationUrl,
  ];
}

async function probeKmsNamespaceAttestation(
  bucket: CoprocessorAttestationBucket,
  evidence: AttestationOutputEvidence,
  workerContainer: string,
  probeImage: string,
): Promise<void> {
  const attestationUrl = rfc023CiphertextUrl(bucket.bucketUrl, evidence.handle);
  try {
    const { stdout, stderr } = await execFile(
      'docker',
      kmsNamespaceAttestationHeadArgs(workerContainer, probeImage, attestationUrl),
      { timeout: 10_000, maxBuffer: 1024 * 1024 },
    );
    const headers = `${stdout}\n${stderr}`;
    const metadata = attestationMetadataFromWgetHeaders(headers);
    assertAttestationMatchesOutputEvidence(metadata, evidence, bucket);
  } catch (error) {
    const detail = error instanceof Error ? error.message : String(error);
    throw new Error(
      `KMS-namespace attestation HEAD failed for tx sender ${bucket.txSender}, bucket ${bucket.bucketUrl}, URL ${attestationUrl}: ${detail}`,
    );
  }
}

/**
 * Wait for one fully published output to be observable through every
 * GatewayConfig bucket from the exact network namespace used by the KMS
 * connector.  This is deliberately an E2E-only guard: it prevents a fixture
 * from burning terminal UserDecryptionRequest retries while MinIO/namespace
 * routing has not yet converged, and does not alter connector retry semantics.
 */
export async function waitForKmsNamespaceAttestationReadiness(options: {
  gatewayRpcUrl: string;
  gatewayConfigAddress: string;
  evidence: AttestationOutputEvidence;
  expectedCoprocessorCount: number;
  timeoutMs?: number;
  pollIntervalMs?: number;
  workerContainer?: string;
  probeImage?: string;
}): Promise<CoprocessorAttestationBucket[]> {
  const buckets = await getRegisteredCoprocessorBuckets(options.gatewayRpcUrl, options.gatewayConfigAddress);
  if (buckets.length !== options.expectedCoprocessorCount) {
    throw new Error(
      `GatewayConfig has ${buckets.length} Coprocessor buckets; expected ${options.expectedCoprocessorCount} for this consensus gate`,
    );
  }

  const workerContainer =
    options.workerContainer ?? process.env.KMS_ATTESTATION_PROBE_CONTAINER ?? DEFAULT_KMS_ATTESTATION_PROBE_CONTAINER;
  const probeImage =
    options.probeImage ?? process.env.KMS_ATTESTATION_PROBE_IMAGE ?? DEFAULT_KMS_ATTESTATION_PROBE_IMAGE;
  const deadline = Date.now() + (options.timeoutMs ?? 120_000);
  let lastError: unknown;
  while (Date.now() < deadline) {
    try {
      // A serial gate keeps a broken URL diagnostic unambiguous and performs
      // exactly one RFC-023 HEAD for each configured bucket per attempt.
      for (const bucket of buckets) {
        await probeKmsNamespaceAttestation(bucket, options.evidence, workerContainer, probeImage);
      }
      return buckets;
    } catch (error) {
      lastError = error;
      await sleep(options.pollIntervalMs ?? 1_000);
    }
  }
  throw new Error(
    `timed out waiting for KMS-namespace attestation readiness: ${(lastError as Error)?.message ?? lastError}`,
  );
}

function bufferHex(value: Buffer): string {
  return `0x${value.toString('hex')}`;
}

function safeNumber(value: string | number, field: string): number {
  const parsed = Number(value);
  if (!Number.isSafeInteger(parsed)) {
    throw new Error(`${field} value ${value} is outside JavaScript's safe integer range`);
  }
  return parsed;
}

function valueEquals(left: Buffer | null, right: Buffer | null): boolean {
  return left === right || (left !== null && right !== null && left.equals(right));
}

function requireDigest(value: Buffer | null, label: string): Buffer {
  if (!value) throw new ConsensusStateIncomplete(`${label} is not publishable: digest missing`);
  if (value.length !== 32) throw new Error(`${label} digest must be exactly 32 bytes, got ${value.length}`);
  return value;
}

function keccakDigest(value: Buffer): Buffer {
  return Buffer.from(ethers.getBytes(ethers.keccak256(value)));
}

function outputLabel(row: CanonicalOutputRow): string {
  return `${bufferHex(row.handle)} produced by ${bufferHex(row.transactionId)} at block ${row.blockNumber}`;
}

/**
 * Obtain each raw output ciphertext together with its canonical computation
 * provenance: the producing transaction, host chain and block, the operation
 * that produced it, and its completion state.  Requiring a completed,
 * non-error `computations` producer proves the bytes are the worker's
 * committed output, not a stray row.
 */
export async function queryCanonicalOutputs(
  databaseUrl: string,
  handles: string[],
  options: CanonicalOutputOptions = {},
): Promise<CanonicalOutputRow[]> {
  if (handles.length === 0) return [];

  const pool = new Pool({ connectionString: databaseUrl, max: 1 });
  try {
    const result = await pool.query<CanonicalOutputSqlRow>(
      `SELECT ciphertext.handle,
              ciphertext.ciphertext,
              sns_ciphertext.ciphertext AS sns_ciphertext,
              ciphertext.ciphertext_type,
              ciphertext.ciphertext_version,
              computation.fhe_operation,
              computation.transaction_id,
              computation.host_chain_id,
              computation.block_number,
              digest.key_id_gw,
              digest.ciphertext AS ciphertext_digest,
              digest.ciphertext128 AS sns_ciphertext_digest,
              digest.ciphertext128_format
         FROM ciphertexts AS ciphertext
         JOIN computations AS computation
           ON computation.output_handle = ciphertext.handle
         LEFT JOIN ciphertext_digest AS digest
           ON digest.handle = computation.output_handle
          AND digest.host_chain_id = computation.host_chain_id
         LEFT JOIN ciphertexts128 AS sns_ciphertext
           ON sns_ciphertext.handle = ciphertext.handle
        WHERE ciphertext.handle = ANY($1::BYTEA[])
          AND ciphertext.ciphertext_version = $2
          AND computation.is_completed
          AND NOT computation.is_error
        ORDER BY computation.host_chain_id,
                 computation.block_number,
                 ciphertext.handle,
                 computation.transaction_id`,
      [handles.map(bytes32ToBuffer), options.ciphertextVersion ?? DEFAULT_CIPHERTEXT_VERSION],
    );

    return result.rows.map((row) => ({
      handle: row.handle,
      ciphertext: row.ciphertext,
      snsCiphertext: row.sns_ciphertext,
      ciphertextType: safeNumber(row.ciphertext_type, 'ciphertexts.ciphertext_type'),
      ciphertextVersion: safeNumber(row.ciphertext_version, 'ciphertexts.ciphertext_version'),
      fheOperation: safeNumber(row.fhe_operation, 'computations.fhe_operation'),
      transactionId: row.transaction_id,
      hostChainId: safeNumber(row.host_chain_id, 'computations.host_chain_id'),
      blockNumber: safeNumber(row.block_number, 'computations.block_number'),
      keyId: (() => {
        if (!row.key_id_gw) {
          throw new ConsensusStateIncomplete(
            `output ${bufferHex(row.handle)} is not publishable: gateway key id missing`,
          );
        }
        return row.key_id_gw;
      })(),
      ciphertextDigest: row.ciphertext_digest,
      snsCiphertextDigest: row.sns_ciphertext_digest,
      ciphertext128Format: (() => {
        if (row.ciphertext128_format === null) {
          throw new ConsensusStateIncomplete(
            `output ${bufferHex(row.handle)} is not publishable: ciphertext_digest.ciphertext128_format missing`,
          );
        }
        return safeNumber(row.ciphertext128_format, 'ciphertext_digest.ciphertext128_format');
      })(),
    }));
  } finally {
    await pool.end();
  }
}

/**
 * Validates the worker/SNS publication boundary inside each coprocessor DB.
 *
 * The TFHE output bytes are retained in `ciphertexts`, so every output must
 * prove `Keccak256(raw ciphertext) == ciphertext_digest.ciphertext`. SNS
 * ciphertext bytes are deliberately short-lived: transaction-sender removes
 * them after a successful L1 submission. When they are still present we prove
 * their Keccak digest too; after that retention point, the durable SNS digest
 * is still required and is bound to the L1 consensus event below.
 */
export function assertCanonicalOutputDigestBindings(perDatabase: CanonicalOutputRow[][]): void {
  for (let databaseIndex = 0; databaseIndex < perDatabase.length; databaseIndex += 1) {
    for (const row of perDatabase[databaseIndex]) {
      if (row.ciphertext.length === 0) {
        throw new Error(`database ${databaseIndex} output ${outputLabel(row)} has empty raw ciphertext`);
      }
      if (row.keyId.length !== 32) {
        throw new Error(
          `database ${databaseIndex} output ${outputLabel(row)} has invalid gateway key id length ${row.keyId.length}`,
        );
      }
      if (!Object.values(ATTESTATION_FORMATS).some((format) => format === row.ciphertext128Format)) {
        throw new Error(
          `database ${databaseIndex} output ${outputLabel(row)} has unsupported ` +
            `ciphertext_digest.ciphertext128_format ${row.ciphertext128Format}`,
        );
      }

      const ciphertextDigest = requireDigest(
        row.ciphertextDigest,
        `database ${databaseIndex} output ${outputLabel(row)}`,
      );
      const expectedCiphertextDigest = keccakDigest(row.ciphertext);
      if (!ciphertextDigest.equals(expectedCiphertextDigest)) {
        throw new Error(
          `database ${databaseIndex} ciphertext digest mismatch for ${outputLabel(row)}: ` +
            'Keccak256(raw ciphertext) does not match ciphertext_digest.ciphertext',
        );
      }

      const snsCiphertextDigest = requireDigest(
        row.snsCiphertextDigest,
        `database ${databaseIndex} SNS output ${outputLabel(row)}`,
      );
      if (row.snsCiphertext && row.snsCiphertext.length > 0) {
        const expectedSnsDigest = keccakDigest(row.snsCiphertext);
        if (!snsCiphertextDigest.equals(expectedSnsDigest)) {
          throw new Error(
            `database ${databaseIndex} SNS digest mismatch for ${outputLabel(row)}: ` +
              'Keccak256(raw SNS ciphertext) does not match ciphertext_digest.ciphertext128',
          );
        }
      }
    }
  }
}

/** The distinct producing transactions of a validated output set. */
export function transactionScopesFromOutputs(outputs: CanonicalOutputRow[], databaseIndex: number): TransactionScope[] {
  const scopes = new Map<string, TransactionScope>();
  for (const output of outputs) {
    const id = bufferHex(output.transactionId);
    const scope = scopes.get(id);
    if (!scope) {
      scopes.set(id, {
        transactionId: output.transactionId,
        hostChainId: output.hostChainId,
        blockNumber: output.blockNumber,
      });
      continue;
    }
    if (scope.hostChainId !== output.hostChainId || scope.blockNumber !== output.blockNumber) {
      throw new Error(`database ${databaseIndex} assigns transaction ${id} to multiple canonical provenance scopes`);
    }
  }
  return [...scopes.values()];
}

/**
 * Completion evidence per producing transaction.  The worker acquires and
 * (re-)executes whole transactions — the materialization boundary — so at
 * quiescence every computation row of each fixture transaction must be
 * completed and none may be an error.
 */
export async function queryTransactionCompletion(
  databaseUrl: string,
  scopes: TransactionScope[],
): Promise<TransactionCompletionRow[]> {
  if (scopes.length === 0) return [];

  const pool = new Pool({ connectionString: databaseUrl, max: 1 });
  try {
    const result = await pool.query<TransactionCompletionSqlRow>(
      `WITH requested AS (
           SELECT *
             FROM UNNEST($1::BYTEA[], $2::BIGINT[], $3::BIGINT[])
                  AS request(transaction_id, host_chain_id, block_number)
       )
       SELECT requested.transaction_id,
              requested.host_chain_id,
              requested.block_number,
              COUNT(computation.output_handle) AS total_count,
              COUNT(computation.output_handle) FILTER (WHERE computation.is_completed) AS completed_count,
              COUNT(computation.output_handle) FILTER (WHERE computation.is_error) AS error_count
         FROM requested
         LEFT JOIN computations AS computation
           ON computation.transaction_id = requested.transaction_id
          AND computation.host_chain_id = requested.host_chain_id
          AND computation.block_number = requested.block_number
        GROUP BY requested.transaction_id,
                 requested.host_chain_id,
                 requested.block_number
        ORDER BY requested.transaction_id`,
      [
        scopes.map((scope) => scope.transactionId),
        scopes.map((scope) => scope.hostChainId),
        scopes.map((scope) => scope.blockNumber),
      ],
    );

    return result.rows.map((row) => ({
      transactionId: row.transaction_id,
      hostChainId: safeNumber(row.host_chain_id, 'computations.host_chain_id'),
      blockNumber: safeNumber(row.block_number, 'computations.block_number'),
      totalCount: safeNumber(row.total_count, 'transaction computation count'),
      completedCount: safeNumber(row.completed_count, 'transaction completed count'),
      errorCount: safeNumber(row.error_count, 'transaction error count'),
    }));
  } finally {
    await pool.end();
  }
}

/**
 * Fails closed when any expected output is absent, duplicated, still missing
 * a digest, or differs in bytes or provenance between homogeneous
 * coprocessors.  The comparison is deliberately byte exact: callers must use
 * this only within one software revision and one backend/hardware class.
 *
 * Handles are content-derived, so the same output handle can legitimately be
 * produced by several same-block transactions (an alias).  A caller expecting
 * that declares the exact producer count in `expectedProducers`; every alias
 * instance must then agree on the persisted bytes and block provenance, and
 * every database must agree on the exact producing-transaction set.
 */
export function assertEquivalentCanonicalOutputs(
  perDatabase: CanonicalOutputRow[][],
  expectedHandles: string[],
  expectedProducers: Record<string, number> = {},
): void {
  if (perDatabase.length < 2) {
    throw new Error('canonical output comparison requires at least two coprocessor databases');
  }
  assertCanonicalOutputDigestBindings(perDatabase);

  const expected = new Set(expectedHandles.map((handle) => bufferHex(bytes32ToBuffer(handle))));
  const producerCount = (handle: string): number => {
    const declared = expectedProducers[handle];
    if (declared === undefined) return 1;
    if (!Number.isInteger(declared) || declared < 1) {
      throw new Error(`invalid expected producer count ${declared} for ${handle}`);
    }
    return declared;
  };

  const normalize = (rows: CanonicalOutputRow[], databaseIndex: number) => {
    const byHandle = new Map<string, CanonicalOutputRow[]>();
    for (const row of rows) {
      const handle = bufferHex(row.handle);
      if (!expected.has(handle)) {
        throw new Error(`database ${databaseIndex} returned unexpected canonical output ${outputLabel(row)}`);
      }
      if (!row.ciphertextDigest || !row.snsCiphertextDigest) {
        throw new ConsensusStateIncomplete(
          `database ${databaseIndex} output ${outputLabel(row)} is not publishable: digest missing`,
        );
      }
      const group = byHandle.get(handle);
      if (group) group.push(row);
      else byHandle.set(handle, [row]);
    }
    for (const handle of expected) {
      const group = byHandle.get(handle);
      if (!group) {
        throw new ConsensusStateIncomplete(`database ${databaseIndex} is missing canonical output ${handle}`);
      }
      const expectedCount = producerCount(handle);
      if (group.length < expectedCount) {
        throw new ConsensusStateIncomplete(
          `database ${databaseIndex} has ${group.length} of ${expectedCount} producers for ${handle}`,
        );
      }
      if (group.length > expectedCount) {
        throw new Error(
          `database ${databaseIndex} returned ${group.length} canonical producers for ${handle}; expected ${expectedCount}`,
        );
      }
      group.sort((left, right) => left.transactionId.compare(right.transactionId));
      // Alias instances must be byte- and provenance-identical apart from
      // their producing transaction: the whole point of canonical
      // materialization is that WHICH transaction produced the handle can
      // never change the bytes.
      for (const row of group) {
        if (
          !row.ciphertext.equals(group[0].ciphertext) ||
          row.ciphertextType !== group[0].ciphertextType ||
          row.fheOperation !== group[0].fheOperation ||
          row.blockNumber !== group[0].blockNumber ||
          row.hostChainId !== group[0].hostChainId
        ) {
          throw new Error(`database ${databaseIndex} alias producers disagree for ${handle}`);
        }
      }
    }
    return byHandle;
  };

  const baseline = normalize(perDatabase[0], 0);
  for (let databaseIndex = 1; databaseIndex < perDatabase.length; databaseIndex += 1) {
    const candidate = normalize(perDatabase[databaseIndex], databaseIndex);
    for (const [handle, leftGroup] of baseline) {
      const rightGroup = candidate.get(handle)!;
      for (let producerIndex = 0; producerIndex < leftGroup.length; producerIndex += 1) {
        const left = leftGroup[producerIndex];
        const right = rightGroup[producerIndex];
        const equal =
          left.ciphertext.equals(right.ciphertext) &&
          left.ciphertextType === right.ciphertextType &&
          left.ciphertextVersion === right.ciphertextVersion &&
          left.fheOperation === right.fheOperation &&
          left.transactionId.equals(right.transactionId) &&
          left.hostChainId === right.hostChainId &&
          left.blockNumber === right.blockNumber &&
          left.keyId.equals(right.keyId) &&
          valueEquals(left.ciphertextDigest, right.ciphertextDigest) &&
          valueEquals(left.snsCiphertextDigest, right.snsCiphertextDigest) &&
          left.ciphertext128Format === right.ciphertext128Format;
        if (!equal) {
          throw new Error(
            `canonical output mismatch for ${handle} between databases 0 and ${databaseIndex}; ` +
              'same-SW/same-backend consensus requires exact ciphertext and provenance equality',
          );
        }
      }
    }
  }
}

/**
 * Whole producing transactions must be completely executed before the fixture
 * is called quiescent, and every coprocessor must agree on the exact per-
 * transaction row counts.  A partially completed transaction is retried by
 * the polling helper; an errored row is a hard failure.
 */
export function assertEquivalentCompletedTransactions(
  perDatabase: TransactionCompletionRow[][],
  scopes: TransactionScope[],
): void {
  if (perDatabase.length < 2) {
    throw new Error('transaction completion comparison requires at least two coprocessor databases');
  }
  const normalize = (rows: TransactionCompletionRow[], databaseIndex: number) => {
    const byId = new Map<string, TransactionCompletionRow>();
    for (const row of rows) {
      const id = bufferHex(row.transactionId);
      if (byId.has(id)) throw new Error(`database ${databaseIndex} returned duplicate transaction completion ${id}`);
      if (row.totalCount <= 0) {
        throw new Error(`database ${databaseIndex} transaction ${id} has no computation rows`);
      }
      if (row.errorCount > 0) {
        throw new Error(`database ${databaseIndex} transaction ${id} has ${row.errorCount} errored computations`);
      }
      if (row.completedCount !== row.totalCount) {
        throw new ConsensusStateIncomplete(
          `database ${databaseIndex} transaction ${id} has completed ${row.completedCount} of ${row.totalCount} computations`,
        );
      }
      byId.set(id, row);
    }
    for (const scope of scopes) {
      const id = bufferHex(scope.transactionId);
      if (!byId.has(id)) {
        throw new ConsensusStateIncomplete(`database ${databaseIndex} is missing transaction completion for ${id}`);
      }
    }
    return byId;
  };

  const baseline = normalize(perDatabase[0], 0);
  for (let databaseIndex = 1; databaseIndex < perDatabase.length; databaseIndex += 1) {
    const candidate = normalize(perDatabase[databaseIndex], databaseIndex);
    if (candidate.size !== baseline.size) {
      throw new Error(`database ${databaseIndex} has a different producing-transaction set from database 0`);
    }
    for (const [id, left] of baseline) {
      const right = candidate.get(id);
      if (!right) throw new Error(`database ${databaseIndex} is missing transaction completion ${id}`);
      const equal =
        left.hostChainId === right.hostChainId &&
        left.blockNumber === right.blockNumber &&
        left.totalCount === right.totalCount &&
        left.completedCount === right.completedCount &&
        left.errorCount === right.errorCount;
      if (!equal) {
        throw new Error(`transaction completion mismatch for ${id} between databases 0 and ${databaseIndex}`);
      }
    }
  }
}

/** Collects and validates a fixture once its output handles are known. */
export async function collectConsensusDatabaseReports(
  databaseUrls: string[],
  expectedHandles: string[],
  options: CanonicalOutputOptions = {},
): Promise<ConsensusDatabaseReport[]> {
  const outputs = await Promise.all(
    databaseUrls.map((databaseUrl) => queryCanonicalOutputs(databaseUrl, expectedHandles, options)),
  );
  assertEquivalentCanonicalOutputs(outputs, expectedHandles, options.expectedProducers ?? {});

  const scopes = transactionScopesFromOutputs(outputs[0], 0);
  const transactions = await Promise.all(
    databaseUrls.map((databaseUrl) => queryTransactionCompletion(databaseUrl, scopes)),
  );
  assertEquivalentCompletedTransactions(transactions, scopes);

  return databaseUrls.map((databaseUrl, index) => ({
    databaseUrl,
    outputs: outputs[index],
    transactions: transactions[index],
  }));
}

/** Polls the database oracle without turning a genuine divergence into a pass. */
export async function waitForConsensusDatabaseReports(
  databaseUrls: string[],
  expectedHandles: string[],
  options: CanonicalOutputOptions & { timeoutMs?: number; pollIntervalMs?: number } = {},
): Promise<ConsensusDatabaseReport[]> {
  const deadline = Date.now() + (options.timeoutMs ?? 120_000);
  let lastError: unknown;
  while (Date.now() < deadline) {
    try {
      return await collectConsensusDatabaseReports(databaseUrls, expectedHandles, options);
    } catch (error) {
      if (!(error instanceof ConsensusStateIncomplete)) {
        throw error;
      }
      lastError = error;
      await sleep(options.pollIntervalMs ?? 1_000);
    }
  }
  throw new Error(`timed out waiting for canonical byte consensus: ${(lastError as Error)?.message ?? lastError}`);
}

/**
 * Closes the publication chain for every fixture output:
 *
 *   raw TFHE ciphertext -> local Keccak digest -> Gateway consensus event
 *
 * The SNS digest follows the same Gateway binding. Its raw bytes are checked
 * when still retained locally by `assertCanonicalOutputDigestBindings`; once
 * transaction-sender has successfully submitted a handle it is expected to
 * delete that transient local copy, so the durable digest is the remaining
 * auditable SNS boundary.
 */
export function assertConsensusEventBindings(
  reports: readonly ConsensusDatabaseReport[],
  events: readonly ConsensusEvent[],
  options: { expectComplete?: boolean } = {},
): void {
  const { expectComplete = true } = options;
  if (reports.length === 0) throw new Error('on-chain consensus binding requires at least one database report');

  const outputs = new Map<string, CanonicalOutputRow>();
  for (const output of reports[0].outputs) {
    const handle = bufferHex(output.handle).toLowerCase();
    const existing = outputs.get(handle);
    if (existing) {
      // Alias producers of one handle share the persisted material; the
      // database oracle has already proven their byte equality. Any digest
      // disagreement here would be a validation gap, not an alias.
      if (
        !valueEquals(existing.ciphertextDigest, output.ciphertextDigest) ||
        !valueEquals(existing.snsCiphertextDigest, output.snsCiphertextDigest) ||
        !existing.keyId.equals(output.keyId)
      ) {
        throw new Error(`baseline report contains conflicting duplicate output ${handle}`);
      }
      continue;
    }
    outputs.set(handle, output);
  }
  const seen = new Set<string>();
  for (const event of events) {
    const handle = bufferHex(bytes32ToBuffer(event.ctHandle)).toLowerCase();
    if (!outputs.has(handle)) {
      throw new Error(`Gateway consensus event is for unexpected ciphertext ${handle}`);
    }
    if (seen.has(handle)) {
      throw new Error(`Gateway emitted duplicate AddCiphertextMaterialConsensus events for ${handle}`);
    }
    seen.add(handle);

    const output = outputs.get(handle)!;
    const ciphertextDigest = requireDigest(output.ciphertextDigest, `baseline output ${handle}`);
    const snsCiphertextDigest = requireDigest(output.snsCiphertextDigest, `baseline SNS output ${handle}`);
    const eventCiphertextDigest = bytes32ToBuffer(event.ciphertextDigest);
    const eventSnsCiphertextDigest = bytes32ToBuffer(event.snsCiphertextDigest);
    const expectedKeyId = BigInt(bufferHex(output.keyId));
    if (
      event.keyId !== expectedKeyId ||
      !eventCiphertextDigest.equals(ciphertextDigest) ||
      !eventSnsCiphertextDigest.equals(snsCiphertextDigest)
    ) {
      throw new Error(`Gateway AddCiphertextMaterialConsensus does not bind the canonical key/digests for ${handle}`);
    }
  }
  if (expectComplete && seen.size !== outputs.size) {
    throw new Error('Gateway is missing AddCiphertextMaterialConsensus events for one or more canonical outputs');
  }
}

/** Wait for the on-chain event carrying a quorum result for one terminal output. */
export async function waitForConsensus(
  gatewayRpcUrl: string,
  ciphertextCommitsAddress: string,
  ctHandle: string,
  timeoutMs = 120_000,
): Promise<ConsensusEvent | null> {
  const provider = new ethers.JsonRpcProvider(gatewayRpcUrl);
  const contract = new ethers.Contract(ciphertextCommitsAddress, CIPHERTEXT_COMMITS_ABI, provider);
  try {
    const deadline = Date.now() + timeoutMs;
    while (Date.now() < deadline) {
      const events = await contract.queryFilter(contract.filters.AddCiphertextMaterialConsensus(ctHandle));
      if (events.length > 1) {
        throw new Error(`Gateway emitted duplicate AddCiphertextMaterialConsensus events for ${ctHandle}`);
      }
      if (events.length === 1) {
        const event = events[0] as ethers.EventLog;
        return {
          ctHandle: event.args[0] as string,
          keyId: event.args[1] as bigint,
          ciphertextDigest: event.args[2] as string,
          snsCiphertextDigest: event.args[3] as string,
          senders: event.args[4] as string[],
          blockNumber: event.blockNumber,
        };
      }
      await sleep(1_000);
    }
    return null;
  } finally {
    provider.destroy();
  }
}

/**
 * The generated E2E topology uses one database per coprocessor.  Explicit
 * URLs are preferred for external runs; the fallback is the in-network
 * compose address used by the test-suite container.
 */
export function getCoprocessorDbUrls(count: number): string[] {
  if (!Number.isInteger(count) || count < 1) throw new Error(`invalid coprocessor count ${count}`);
  const host = process.env.CONSENSUS_POSTGRES_HOST ?? 'db:5432';
  return Array.from({ length: count }, (_, index) => {
    const configured = process.env[`DATABASE_URL_${index}`];
    if (configured) return configured;
    const name = index === 0 ? 'coprocessor' : `coprocessor_${index}`;
    return `postgresql://postgres:postgres@${host}/${name}`;
  });
}

/** Explicitly waits for all isolated databases; no test should infer readiness from Docker start. */
export async function waitForDatabaseReadiness(databaseUrls: string[], timeoutMs = 120_000): Promise<void> {
  const deadline = Date.now() + timeoutMs;
  let lastError: unknown;
  while (Date.now() < deadline) {
    try {
      await Promise.all(
        databaseUrls.map(async (databaseUrl) => {
          const pool = new Pool({ connectionString: databaseUrl, max: 1 });
          try {
            await pool.query('SELECT 1');
          } finally {
            await pool.end();
          }
        }),
      );
      return;
    } catch (error) {
      lastError = error;
      await sleep(1_000);
    }
  }
  throw new Error(`timed out waiting for coprocessor databases: ${(lastError as Error)?.message ?? lastError}`);
}

function validatedContainerName(value: string): string {
  if (!/^[a-zA-Z0-9][a-zA-Z0-9_.-]*$/.test(value)) {
    throw new Error(`unsafe Docker container name: ${value}`);
  }
  return value;
}

async function docker(action: 'start' | 'stop' | 'restart' | 'pause' | 'unpause', containers: string[]): Promise<void> {
  if (containers.length === 0) return;
  await execFile('docker', [action, ...containers.map(validatedContainerName)]);
}

export async function dockerStart(...containers: string[]): Promise<void> {
  await docker('start', containers);
}

export async function dockerStop(...containers: string[]): Promise<void> {
  await docker('stop', containers);
}

export async function dockerRestart(...containers: string[]): Promise<void> {
  await docker('restart', containers);
}

export async function dockerPause(...containers: string[]): Promise<void> {
  await docker('pause', containers);
}

export async function dockerUnpause(...containers: string[]): Promise<void> {
  await docker('unpause', containers);
}

export function containerName(instanceIndex: number, service: string): string {
  if (!Number.isInteger(instanceIndex) || instanceIndex < 0) {
    throw new Error(`invalid coprocessor index ${instanceIndex}`);
  }
  if (!/^[a-z0-9-]+$/.test(service)) throw new Error(`invalid coprocessor service ${service}`);
  return `${instanceIndex === 0 ? 'coprocessor' : `coprocessor${instanceIndex}`}-${service}`;
}

export function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}
