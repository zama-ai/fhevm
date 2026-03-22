import type { DecryptedFheValueMap } from "./decryptedFheValue.js";
import type { FheType } from "./fheType.js";
import type { FhevmHandleOfType } from "./fhevmHandle.js";

////////////////////////////////////////////////////////////////////////////////
//
// DecryptedFhevmHandle
//
////////////////////////////////////////////////////////////////////////////////

export interface DecryptedFhevmHandleOfTypeBase<T extends FheType> {
  readonly fheType: T;
  readonly handle: FhevmHandleOfType<T>;
  readonly value: DecryptedFheValueMap[T];
}

/**
 * A decrypted FHEVM handle: pairs an encrypted {@link FhevmHandle} with its
 * plaintext clear value.
 *
 * When `T` is a specific {@link FheType} literal (e.g. `'ebool'`), this
 * resolves to a single concrete type. When `T` is the full `FheType`
 * union (the default), it produces a **discriminated union** of all variants,
 * enabling narrowing via `fheType`:
 *
 * ```typescript
 * declare const d: DecryptedFhevmHandle;
 * if (d.fheType === 'ebool') {
 *   d.value; // boolean ✅
 * }
 * ```
 *
 * Instances are created internally via {@link createDecryptedFhevmHandle}
 * and can be checked with {@link isDecryptedFhevmHandle}.
 * Direct construction is not possible.
 *
 * @template T - The FHE type name ('ebool', 'euint8', 'eaddress', etc.)
 */
export type DecryptedFhevmHandleOfType<T extends FheType = FheType> = {
  [K in T]: DecryptedFhevmHandleOfTypeBase<K>;
}[T];

export type DecryptedEbool = DecryptedFhevmHandleOfType<"ebool">;
export type DecryptedEuint8 = DecryptedFhevmHandleOfType<"euint8">;
export type DecryptedEuint16 = DecryptedFhevmHandleOfType<"euint16">;
export type DecryptedEuint32 = DecryptedFhevmHandleOfType<"euint32">;
export type DecryptedEuint64 = DecryptedFhevmHandleOfType<"euint64">;
export type DecryptedEuint128 = DecryptedFhevmHandleOfType<"euint128">;
export type DecryptedEuint256 = DecryptedFhevmHandleOfType<"euint256">;
export type DecryptedEaddress = DecryptedFhevmHandleOfType<"eaddress">;

export type DecryptedFhevmHandle = DecryptedFhevmHandleOfType;
