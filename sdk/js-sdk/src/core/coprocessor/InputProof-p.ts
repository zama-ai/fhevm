import type {
  Bytes,
  Bytes32Hex,
  Bytes65Hex,
  BytesHex,
  ChecksummedAddress,
  Uint,
} from '../types/primitives.js';
import { MAX_UINT8, uintToBytesHexNo0x } from '../base/uint.js';
import {
  assertIsBytes,
  assertIsBytes65HexArray,
  assertIsBytesHex,
  bytes32ToHex,
  bytes65ToHex,
  bytesToHex,
  hexToBytes,
  unsafeBytesEquals,
} from '../base/bytes.js';
import { assert } from '../base/errors/InternalError.js';
import { remove0x } from '../base/string.js';
import {
  InputProofError,
  TooManyHandlesError,
} from '../errors/InputProofError.js';
import { toInputHandle } from '../handle/FhevmHandle.js';
import type { ErrorMetadataParams } from '../base/errors/ErrorBase.js';
import { assertIsChecksummedAddress } from '../base/address.js';
import { InvalidTypeError } from '../base/errors/InvalidTypeError.js';
import type {
  InputProof,
  InputProofBytes,
  UnverifiedInputProof,
  VerifiedInputProof,
} from '../types/inputProof.js';
import type { NonEmptyReadonlyArray } from '../types/utils.js';
import type { InputHandle, InputHandleLike } from '../types/encryptedTypes.js';

////////////////////////////////////////////////////////////////////////////////

const PRIVATE_TOKEN = Symbol('InputProof.token');

////////////////////////////////////////////////////////////////////////////////
// Private class InputProof
////////////////////////////////////////////////////////////////////////////////

class InputProofImpl implements InputProof {
  readonly #inputProofBytesHex: BytesHex;

  // Components of the proof
  readonly #inputHandles: NonEmptyReadonlyArray<InputHandle>;
  readonly #coprocessorSignatures: NonEmptyReadonlyArray<Bytes65Hex>;
  readonly #extraData: BytesHex;
  // Optional data required to verify individual coprocessor signatures
  readonly #signedHandleAccess?: {
    // zkProof's userAddress
    readonly userAddress: ChecksummedAddress;
    // zkProof's contrAddress
    readonly contractAddress: ChecksummedAddress;
  };

  constructor(
    privateToken: symbol,
    parameters: {
      readonly inputProofBytesHex: BytesHex;
      readonly coprocessorSignatures: readonly Bytes65Hex[];
      readonly inputHandles: readonly InputHandle[];
      readonly extraData: BytesHex;
      readonly signedHandleAccess?:
        | {
            readonly userAddress: ChecksummedAddress;
            readonly contractAddress: ChecksummedAddress;
          }
        | undefined;
    },
  ) {
    if (privateToken !== PRIVATE_TOKEN) {
      throw new Error('Unauthorized');
    }

    const {
      inputProofBytesHex,
      coprocessorSignatures,
      inputHandles,
      extraData,
      signedHandleAccess,
    } = parameters;

    // Note: it is not possible to create a ZKProof with zero values.
    // consequently, the handles array cannot be empty
    assert(inputHandles.length > 0);
    assert(coprocessorSignatures.length > 0);

    this.#inputProofBytesHex = inputProofBytesHex;
    this.#coprocessorSignatures =
      coprocessorSignatures as NonEmptyReadonlyArray<Bytes65Hex>;
    this.#inputHandles = inputHandles as NonEmptyReadonlyArray<InputHandle>;
    this.#extraData = extraData;
    if (signedHandleAccess !== undefined) {
      this.#signedHandleAccess = { ...signedHandleAccess };
    }

    Object.freeze(this.#coprocessorSignatures);
    Object.freeze(this.#inputHandles);
    Object.freeze(this.#signedHandleAccess);
    Object.freeze(this);
  }

  public get bytesHex(): BytesHex {
    return this.#inputProofBytesHex;
  }

  public get coprocessorSignatures(): NonEmptyReadonlyArray<Bytes65Hex> {
    return this.#coprocessorSignatures;
  }

  public get inputHandles(): NonEmptyReadonlyArray<InputHandle> {
    return this.#inputHandles;
  }

  public get extraData(): BytesHex {
    return this.#extraData;
  }

  public get verified(): boolean {
    return this.#signedHandleAccess !== undefined;
  }

  public get signedHandleAccess():
    | {
        readonly contractAddress: ChecksummedAddress;
        readonly userAddress: ChecksummedAddress;
      }
    | undefined {
    return this.#signedHandleAccess;
  }
}

////////////////////////////////////////////////////////////////////////////////
// Freeze
////////////////////////////////////////////////////////////////////////////////

Object.freeze(InputProofImpl);
Object.freeze(InputProofImpl.prototype);

////////////////////////////////////////////////////////////////////////////////
// Public API
////////////////////////////////////////////////////////////////////////////////

export function createUnverifiedInputProofFromComponents(args: {
  readonly coprocessorEIP712Signatures: readonly Bytes65Hex[];
  readonly inputHandles: readonly InputHandleLike[];
  readonly extraData: BytesHex;
}): UnverifiedInputProof {
  return createInputProofFromComponents(args) as UnverifiedInputProof;
}

////////////////////////////////////////////////////////////////////////////////

export function createInputProofFromComponents({
  coprocessorEIP712Signatures,
  inputHandles,
  extraData,
  signedHandleAccess,
}: {
  readonly coprocessorEIP712Signatures: readonly Bytes65Hex[];
  readonly inputHandles: readonly InputHandleLike[];
  readonly extraData: BytesHex;
  readonly signedHandleAccess?:
    | {
        readonly userAddress: ChecksummedAddress;
        readonly contractAddress: ChecksummedAddress;
      }
    | undefined;
}): InputProof {
  if (inputHandles.length === 0) {
    throw new InputProofError({
      message: `Input proof must contain at least one external handle`,
    });
  }

  if (signedHandleAccess !== undefined) {
    assertIsChecksummedAddress(signedHandleAccess.userAddress, {});
    assertIsChecksummedAddress(signedHandleAccess.contractAddress, {});
  }

  const externalFhevmHandles: InputHandle[] = inputHandles.map(toInputHandle);

  assertIsBytes65HexArray(coprocessorEIP712Signatures, {});
  assertIsBytesHex(extraData, {});

  const numberOfHandles = inputHandles.length;
  const numberOfSignatures = coprocessorEIP712Signatures.length;

  if (numberOfHandles > MAX_UINT8) {
    throw new TooManyHandlesError({ numberOfHandles });
  }

  assert(numberOfSignatures <= MAX_UINT8);

  const numHandlesHexByte1 = uintToBytesHexNo0x(numberOfHandles as Uint);
  const numSignaturesHexByte1 = uintToBytesHexNo0x(numberOfHandles as Uint);

  assert(numHandlesHexByte1.length === 2); // Byte1
  assert(numSignaturesHexByte1.length === 2); // Byte1

  //
  // Proof format :
  // ==============
  //
  // <len(handles)><len(signatures)><concat(handles)><concat(signatures)>
  //
  // size: Byte1 + Byte1 + len(handles)*Bytes32 + len(signatures)*Bytes65
  //

  let proof: string = '';

  // Add number of handles (uint8 | Byte1)
  proof += uintToBytesHexNo0x(inputHandles.length as Uint);

  // Add number of signatures (uint8 | Byte1)
  proof += uintToBytesHexNo0x(coprocessorEIP712Signatures.length as Uint);

  // Add handles: (uint256 | Byte32) x numHandles
  externalFhevmHandles.map((h) => (proof += h.bytes32HexNo0x));

  // Add signatures: (uint256 | Byte32) x numSignatures
  coprocessorEIP712Signatures.map(
    (signatureBytesHex: BytesHex) => (proof += remove0x(signatureBytesHex)),
  );

  // Append the extra data to the input proof
  proof += remove0x(extraData);

  // Make sure we get the right size
  assert(
    proof.length ===
      (1 + 1 + numberOfHandles * 32 + numberOfSignatures * 65) * 2 +
        (extraData.length - 2),
  );

  const inputProof = new InputProofImpl(PRIVATE_TOKEN, {
    inputProofBytesHex: `0x${proof}` as BytesHex,
    coprocessorSignatures: [...coprocessorEIP712Signatures],
    inputHandles: externalFhevmHandles,
    extraData,
    signedHandleAccess: signedHandleAccess,
  });

  return inputProof;
}

////////////////////////////////////////////////////////////////////////////////

export function createUnverifiedInputProofFromRawBytes(
  inputProofBytes: Bytes,
): UnverifiedInputProof {
  return createInputProofFromRawBytes({
    inputProofBytes,
  }) as UnverifiedInputProof;
}

////////////////////////////////////////////////////////////////////////////////

export function createInputProofFromRawBytes({
  inputProofBytes,
  signedHandleAccess,
}: {
  readonly inputProofBytes: Bytes;
  readonly signedHandleAccess?: {
    readonly userAddress: ChecksummedAddress;
    readonly contractAddress: ChecksummedAddress;
  };
}): InputProof {
  assertIsBytes(inputProofBytes, {});

  if (inputProofBytes.length < 2) {
    throw new InputProofError({
      message: `Invalid proof: too short`,
    });
  }

  const numHandles = inputProofBytes[0];

  if (numHandles === 0 || numHandles === undefined) {
    throw new InputProofError({
      message: `Input proof must contain at least one external handle`,
    });
  }

  const numSignatures = inputProofBytes[1] as unknown as number;

  const HANDLE_SIZE = 32;
  const SIGNATURE_SIZE = 65;
  const HEADER_SIZE = 2;

  const handlesStart = HEADER_SIZE;
  const handlesEnd = handlesStart + numHandles * HANDLE_SIZE;
  const signaturesStart = handlesEnd;
  const signaturesEnd = signaturesStart + numSignatures * SIGNATURE_SIZE;
  const extraDataStart = signaturesEnd;

  if (inputProofBytes.length < signaturesEnd) {
    throw new InputProofError({
      message: `Invalid proof: expected at least ${signaturesEnd} bytes, got ${inputProofBytes.length}`,
    });
  }

  // Extract handles
  const handles: Bytes32Hex[] = [];
  for (let i = 0; i < numHandles; i++) {
    const start = handlesStart + i * HANDLE_SIZE;
    const end = start + HANDLE_SIZE;
    const handleBytes = inputProofBytes.slice(start, end);
    const handleBytes32Hex = bytes32ToHex(handleBytes);
    handles.push(handleBytes32Hex);
  }

  // Extract signatures
  const signatures: Bytes65Hex[] = [];
  for (let i = 0; i < numSignatures; i++) {
    const start = signaturesStart + i * SIGNATURE_SIZE;
    const end = start + SIGNATURE_SIZE;
    const signatureBytes = inputProofBytes.slice(start, end);
    const signatureBytes65Hex = bytes65ToHex(signatureBytes);
    signatures.push(signatureBytes65Hex);
  }

  // Extract extra data
  const extraDataBytes = inputProofBytes.slice(extraDataStart);
  const extraData = bytesToHex(extraDataBytes);

  const inputProof = createInputProofFromComponents({
    coprocessorEIP712Signatures: signatures,
    inputHandles: handles,
    extraData,
    signedHandleAccess: signedHandleAccess,
  });

  /// Debug TO BE REMOVED
  assert(bytesToHex(inputProofBytes) === inputProof.bytesHex);
  //////////

  return inputProof;
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Validates that the provided handles and inputProof bytes match this InputProof.
 * Use this as a sanity check to ensure handles correspond to the proof data.
 */
export function inputProofBytesEquals(
  bytesA: InputProofBytes,
  bytesB: InputProofBytes,
): boolean {
  if (bytesA.handles.length !== bytesB.handles.length) {
    return false;
  }
  for (let i = 0; i < bytesA.handles.length; ++i) {
    const a = bytesA.handles[i];
    const b = bytesB.handles[i];
    if (!unsafeBytesEquals(a, b)) {
      return false;
    }
  }
  return unsafeBytesEquals(bytesA.inputProof, bytesB.inputProof);
}

////////////////////////////////////////////////////////////////////////////////

export function isInputProof(value: unknown): value is InputProof {
  return value instanceof InputProofImpl;
}

////////////////////////////////////////////////////////////////////////////////

export function assertIsInputProof(
  value: unknown,
  options: { subject?: string } & ErrorMetadataParams,
): asserts value is InputProof {
  if (!isInputProof(value)) {
    throw new InvalidTypeError(
      {
        subject: options.subject,
        type: typeof value,
        expectedType: 'InputProof',
      },
      options,
    );
  }
}

////////////////////////////////////////////////////////////////////////////////

export function isVerifiedInputProof(
  value: unknown,
): value is VerifiedInputProof & {
  readonly signedHandleAccess: {
    readonly userAddress: ChecksummedAddress;
    readonly contractAddress: ChecksummedAddress;
  };
} {
  return isInputProof(value) && value.signedHandleAccess !== undefined;
}

////////////////////////////////////////////////////////////////////////////////

export function assertIsVerifiedInputProof(
  value: unknown,
  options: { subject?: string } & ErrorMetadataParams,
): asserts value is VerifiedInputProof {
  if (!isVerifiedInputProof(value)) {
    throw new InvalidTypeError(
      {
        subject: options.subject,
        type: typeof value,
        expectedType: 'VerifiedInputProof',
      },
      options,
    );
  }
}

////////////////////////////////////////////////////////////////////////////////

export function toInputProofBytes(inputProof: InputProof): InputProofBytes {
  if (!(inputProof instanceof InputProofImpl)) {
    throw new InputProofError({ message: 'Invalid inputProof object' });
  }
  return {
    handles: inputProof.inputHandles.map(
      (h) => h.bytes32 as Uint8Array,
    ) as unknown as NonEmptyReadonlyArray<Uint8Array>,
    inputProof: hexToBytes(inputProof.bytesHex),
  };
}
