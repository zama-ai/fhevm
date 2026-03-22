import type { ErrorMetadataParams } from "../base/errors/ErrorBase.js";
import type { BytesHex } from "../types/primitives.js";
import type { FheTypeId, FheType } from "../types/fheType.js";
import type {
  DecryptedFheValue,
  DecryptedFheValueMap,
} from "../types/decryptedFheValue.js";
import type { FhevmHandle, FhevmHandleOfType } from "../types/fhevmHandle.js";
import type {
  DecryptedFhevmHandle,
  DecryptedFhevmHandleOfType,
  DecryptedFhevmHandleOfTypeBase,
} from "../types/decryptedFhevmHandle.js";
import { InvalidTypeError } from "../base/errors/InvalidTypeError.js";
import { asFheDecryptedValue } from "./FheType.js";
import { assertNever } from "../base/errors/utils.js";
import type { Fhevm } from "../types/coreFhevmClient.js";

////////////////////////////////////////////////////////////////////////////////

const PRIVATE_TOKEN = Symbol("DecryptedFhevmHandle.token");

////////////////////////////////////////////////////////////////////////////////

/**
 * Module-scoped symbol used as the method key for origin verification.
 * Never exported — invisible to IDE autocomplete and external code.
 * @internal
 */
const VERIFY_ORIGIN_FUNC = Symbol("DecryptedFhevmHandle.verifyOrigin");

/**
 * Internal implementation. Not exported — external code cannot instantiate.
 *
 * Security relies on:
 * - Class not being exported (no `new` from outside)
 * - `Object.freeze` on every instance (immutability)
 * - Private fields (`#handle`, `#value`, `#originToken`) inaccessible from outside
 * - `Object.freeze` on prototype (no prototype pollution)
 * - Symbol-keyed `[VERIFY_ORIGIN]` method invisible to IDE and external code
 */
class DecryptedFhevmHandleImpl<T extends FheType>
  implements DecryptedFhevmHandleOfTypeBase<T>
{
  readonly #handle: FhevmHandleOfType<T>;
  readonly #value: DecryptedFheValueMap[T];
  readonly #originToken: symbol;

  constructor(
    privateToken: symbol,
    parameters: {
      readonly handle: FhevmHandleOfType<T>;
      readonly value: DecryptedFheValueMap[T];
      readonly originToken: symbol;
    },
  ) {
    if (privateToken !== PRIVATE_TOKEN) {
      throw new Error("Unauthorized");
    }

    this.#handle = parameters.handle;
    this.#value = parameters.value;
    this.#originToken = parameters.originToken;
  }

  public get fheType(): T {
    return this.#handle.fheType;
  }

  public get handle(): FhevmHandleOfType<T> {
    return this.#handle;
  }

  public get value(): DecryptedFheValueMap[T] {
    return this.#value;
  }

  /**
   * Checks that this instance was created with the given origin token.
   * Symbol-keyed — invisible to IDE autocomplete and inaccessible without
   * the module-scoped {@link VERIFY_ORIGIN_FUNC} symbol.
   */
  public [VERIFY_ORIGIN_FUNC](token: symbol): boolean {
    return this.#originToken === token;
  }

  /**
   * Safe string representation that does not expose the value.
   */
  public toString(): string {
    return `DecryptedFhevmHandle<${this.#handle.fheType}>`;
  }

  /**
   * Safe JSON serialization that does not expose the value.
   */
  public toJson(): { handle: string; fheType: FheType } {
    return {
      handle: this.#handle.bytes32Hex,
      fheType: this.#handle.fheType,
    };
  }
}

Object.freeze(DecryptedFhevmHandleImpl.prototype);

// ============================================================================
// Public API — Guards & Assertions
// ============================================================================

/**
 * Returns `true` if `value` was created via {@link createDecryptedFhevmHandle}
 * and its origin matches the given `originToken`.
 *
 * Uses `instanceof` against the non-exported `DecryptedFhevmHandleImpl` class
 * (unforgeable in same-realm contexts), then verifies the origin token.
 *
 * @param value - The value to check
 * @param originToken - Origin symbol held privately by the decrypt flow
 */
export function isDecryptedFhevmHandle(
  value: unknown,
  originToken: symbol,
): value is DecryptedFhevmHandle {
  if (!(value instanceof DecryptedFhevmHandleImpl)) return false;
  return value[VERIFY_ORIGIN_FUNC](originToken);
}

/**
 * Asserts that `value` was created via {@link createDecryptedFhevmHandle}
 * and its origin matches the given `originToken`.
 *
 * @throws {InvalidTypeError} If the value is not a `DecryptedFhevmHandle`
 * instance, or if it fails origin verification.
 */
export function assertIsDecryptedFhevmHandle(
  value: unknown,
  options: { subject?: string; originToken: symbol } & ErrorMetadataParams,
): asserts value is DecryptedFhevmHandle {
  if (!isDecryptedFhevmHandle(value, options.originToken)) {
    throw new InvalidTypeError(
      {
        subject: options.subject,
        type: typeof value,
        expectedType: "DecryptedFhevmHandle",
      },
      options,
    );
  }
}

/**
 * Returns `true` if every element was created via
 * {@link createDecryptedFhevmHandle} and its origin matches the given
 * `originToken`.
 */
export function isDecryptedFhevmHandleArray(
  values: readonly unknown[],
  originToken: symbol,
): values is readonly DecryptedFhevmHandle[] {
  return values.every((v) => isDecryptedFhevmHandle(v, originToken));
}

/**
 * Asserts that `values` is an array where every element was created via
 * {@link createDecryptedFhevmHandle} and its origin matches the given
 * `originToken`.
 *
 * @throws {InvalidTypeError} If the value is not an array, or if any element
 * is not a `DecryptedFhevmHandle` instance (error includes the index).
 */
export function assertIsDecryptedFhevmHandleArray(
  values: unknown,
  options: { subject?: string; originToken: symbol } & ErrorMetadataParams,
): asserts values is readonly DecryptedFhevmHandle[] {
  if (!Array.isArray(values)) {
    throw new InvalidTypeError(
      {
        subject: options.subject,
        type: typeof values,
        expectedType: "DecryptedFhevmHandle[]",
      },
      options,
    );
  }
  for (let i = 0; i < values.length; ++i) {
    if (!isDecryptedFhevmHandle(values[i], options.originToken)) {
      throw new InvalidTypeError(
        {
          subject: options.subject,
          index: i,
          type: typeof values[i],
          expectedType: "DecryptedFhevmHandle",
        },
        options,
      );
    }
  }
}

// ============================================================================
// Public API — Factory
// ============================================================================

/**
 * Creates a validated, immutable {@link DecryptedFhevmHandleOfTypeBase}.
 *
 * The `originToken` parameter acts as access control: only code that holds
 * a private `Symbol` (e.g. `publicDecrypt`, `userDecrypt`) can produce
 * instances that pass {@link isDecryptedFhevmHandle} with origin verification.
 *
 * @param handle - A validated {@link FhevmHandle}
 * @param value - The decrypted plaintext value (validated against `handle.fheType`)
 * @param originToken - Private symbol owned by the calling decrypt flow
 * @returns A frozen `DecryptedFhevmHandle` instance
 * @throws {InvalidTypeError} If the value doesn't match the handle's FHE type
 */
export function createDecryptedFhevmHandle<T extends FheType>(
  handle: FhevmHandleOfType<T>,
  value: DecryptedFheValueMap[T],
  originToken: symbol,
): DecryptedFhevmHandleOfType<T> {
  const v = new DecryptedFhevmHandleImpl<T>(PRIVATE_TOKEN, {
    handle,
    value: asFheDecryptedValue(handle.fheType, value),
    originToken,
  });
  Object.freeze(v);
  return v;
}

/**
 * Creates an array of {@link DecryptedFhevmHandleOfTypeBase}s from parallel arrays of
 * handles and clear values.
 *
 * @param orderedHandles - Validated FHEVM handles
 * @param orderedClearValues - Corresponding decrypted values (same length & order)
 * @param originToken - Private symbol owned by the calling decrypt flow
 * @returns A frozen array of frozen `DecryptedFhevmHandle` instances
 */
export function createDecryptedFhevmHandleArray(
  orderedHandles: readonly FhevmHandle[],
  orderedClearValues: readonly DecryptedFheValue[],
  originToken: symbol,
): readonly DecryptedFhevmHandle[] {
  if (orderedHandles.length !== orderedClearValues.length) {
    throw new InvalidTypeError(
      {
        subject: "orderedClearValues",
        type: `Array(${orderedClearValues.length})`,
        expectedType: `Array(${orderedHandles.length}) — must match orderedHandles length`,
      },
      {},
    );
  }

  const result = orderedHandles.map((handle, i) =>
    createDecryptedFhevmHandle(
      handle,
      orderedClearValues[i] as unknown as DecryptedFheValue,
      originToken,
    ),
  );

  return Object.freeze(result);
}

function _decryptedFheValueToBigInt(value: DecryptedFheValue): bigint {
  if (typeof value === "boolean") {
    return value ? BigInt("0x01") : BigInt("0x00");
  }
  return BigInt(value);
}

export function abiEncodeDecryptedFhevmHandles(
  fhevm: Fhevm,
  args: {
    readonly orderedHandles: readonly DecryptedFhevmHandle[];
  },
): {
  abiTypes: Array<"uint256">;
  abiValues: Array<string | bigint>;
  abiEncodedClearValues: BytesHex;
} {
  const orderedHandles = args.orderedHandles;
  const abiTypes: Array<"uint256"> = [];
  const abiValues: Array<string | bigint> = [];

  for (const orderedHandle of orderedHandles) {
    const handleType: FheTypeId = orderedHandle.handle.fheTypeId;

    const clearTextValueBigInt = _decryptedFheValueToBigInt(
      orderedHandle.value,
    );

    //abiTypes.push(fhevmTypeInfo.solidityTypeName);
    abiTypes.push("uint256");

    switch (handleType) {
      // eaddress
      case 7: {
        // string
        abiValues.push(
          `0x${clearTextValueBigInt.toString(16).padStart(40, "0")}`,
        );
        break;
      }
      // ebool
      case 0: {
        // bigint (0 or 1)
        if (
          clearTextValueBigInt !== BigInt(0) &&
          clearTextValueBigInt !== BigInt(1)
        ) {
          throw new Error(
            `Invalid ebool clear text value ${clearTextValueBigInt}. Expecting 0 or 1.`,
          );
        }
        abiValues.push(clearTextValueBigInt);
        break;
      }
      case 2: //euint8
      case 3: //euint16
      case 4: //euint32
      case 5: //euint64
      case 6: //euint128
      case 8: {
        //euint256
        // bigint
        abiValues.push(clearTextValueBigInt);
        break;
      }
      default: {
        assertNever(
          handleType,
          `Unsupported Fhevm primitive type id: ${handleType}`,
        );
      }
    }
  }

  // ABI encode the decryptedResult as done in the KMS, since all decrypted values
  // are native static types, thay have same abi-encoding as uint256:
  const abiEncodedClearValues: BytesHex = fhevm.runtime.ethereum.encode({
    types: abiTypes,
    values: abiValues,
  });

  return {
    abiTypes,
    abiValues,
    abiEncodedClearValues,
  };
}
