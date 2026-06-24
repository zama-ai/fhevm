import type { WithEncrypt } from '../types/coreFhevmRuntime.js';
import type { FheEncryptionKeyWasm, FheEncryptionKeyBytes } from '../types/fheEncryptionKey.js';
import { bytesToHexLarge } from '../base/bytes.js';
import { createFheEncryptionKeyWasm } from './FheEncryptionKeyWasm-p.js';

////////////////////////////////////////////////////////////////////////////////

const DIAGNOSTIC_EDGE_BYTE_LENGTH = 32;

type BytesDiagnostic = {
  readonly id: string;
  readonly length: number;
  readonly sha256: string;
  readonly firstBytesHex: string;
  readonly lastBytesHex: string;
};

type FheEncryptionKeyBytesDiagnostic = {
  readonly metadata: FheEncryptionKeyBytes['metadata'];
  readonly publicKey: BytesDiagnostic;
  readonly crs: BytesDiagnostic & { readonly capacity: number };
};

type GlobalCryptoProvider = Omit<typeof globalThis, 'crypto'> & { readonly crypto?: Crypto | undefined };

export async function deserializeFheEncryptionKey(
  context: { readonly runtime: WithEncrypt },
  parameters: FheEncryptionKeyBytes,
): Promise<FheEncryptionKeyWasm> {
  const logger = context.runtime.config.logger;
  const diagnostic = logger === undefined ? undefined : await createFheEncryptionKeyBytesDiagnostic(parameters);

  logger?.debug(`[fhevm-sdk] fetched FHE encryption key bytes ${JSON.stringify(diagnostic)}`);

  let publicKeyNative: Awaited<ReturnType<typeof context.runtime.encrypt.deserializeFheEncryptionPublicKey>>;
  try {
    publicKeyNative = await context.runtime.encrypt.deserializeFheEncryptionPublicKey({
      publicKeyBytes: parameters.publicKeyBytes,
    });
  } catch (cause: unknown) {
    logger?.error(`[fhevm-sdk] failed to deserialize FHE public key bytes ${JSON.stringify(diagnostic)}`, cause);
    throw cause;
  }

  const crsNative = await context.runtime.encrypt.deserializeFheEncryptionCrs({
    crsBytes: parameters.crsBytes,
  });

  return createFheEncryptionKeyWasm(new WeakRef(context.runtime), {
    publicKey: publicKeyNative,
    crs: crsNative,
    metadata: parameters.metadata,
  });
}

async function createFheEncryptionKeyBytesDiagnostic(
  parameters: FheEncryptionKeyBytes,
): Promise<FheEncryptionKeyBytesDiagnostic> {
  const [publicKey, crs] = await Promise.all([
    createBytesDiagnostic(parameters.publicKeyBytes.id, parameters.publicKeyBytes.bytes),
    createBytesDiagnostic(parameters.crsBytes.id, parameters.crsBytes.bytes),
  ]);

  return {
    metadata: parameters.metadata,
    publicKey,
    crs: { ...crs, capacity: parameters.crsBytes.capacity },
  };
}

async function createBytesDiagnostic(id: string, bytes: Uint8Array): Promise<BytesDiagnostic> {
  return {
    id,
    length: bytes.length,
    sha256: await sha256Hex(bytes),
    firstBytesHex: edgeBytesHex(bytes, 0),
    lastBytesHex: edgeBytesHex(bytes, Math.max(0, bytes.length - DIAGNOSTIC_EDGE_BYTE_LENGTH)),
  };
}

function edgeBytesHex(bytes: Uint8Array, start: number): string {
  return bytesToHexLarge(bytes.subarray(start, start + DIAGNOSTIC_EDGE_BYTE_LENGTH));
}

async function sha256Hex(bytes: Uint8Array): Promise<string> {
  const globalObject = globalThis as GlobalCryptoProvider;
  const subtle = globalObject.crypto?.subtle;
  if (subtle === undefined) {
    return 'unavailable';
  }

  try {
    const digest = await subtle.digest('SHA-256', copyBytesToArrayBuffer(bytes));
    return bytesToHexLarge(new Uint8Array(digest));
  } catch (cause: unknown) {
    return `unavailable: ${cause instanceof Error ? cause.message : String(cause)}`;
  }
}

function copyBytesToArrayBuffer(bytes: Uint8Array): ArrayBuffer {
  const copy = new Uint8Array(bytes.length);
  copy.set(bytes);
  return copy.buffer;
}
