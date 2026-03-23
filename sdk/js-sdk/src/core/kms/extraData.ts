import type { BytesHex } from "../types/primitives.js";
import type { UintBigInt } from "../types/primitives.js";

/**
 * Mirrors the contract's `_extractKmsContextId` version-dispatch logic:
 * - empty bytes (`'0x'`) or first byte `0x00` → legacy (current context / init-time signers)
 * - first byte `0x01` → v1 context-bearing extraData
 * - other versions → error
 *
 * @internal Not exported from the public SDK API.
 */

/**
 * Returns `true` when the contract would use `$.currentKmsContextId` (legacy path):
 * - `extraData` is empty bytes (`'0x'`)
 * - First byte is `0x00` (version 0), regardless of trailing bytes
 *
 * Matches the contract: `extraData.length == 0 || uint8(extraData[0]) == 0x00`.
 */
export function isLegacyExtraData(extraData: BytesHex): boolean {
  // '0x' is empty bytes
  if (extraData === ("0x" as BytesHex)) return true;
  // First byte is at characters 2-3 (after '0x' prefix)
  const firstByte = extraData.slice(2, 4).toLowerCase();
  return firstByte === "00";
}

/**
 * Parses versioned extraData to extract the context ID.
 * Called only when `isLegacyExtraData` returns `false`.
 *
 * - Version `0x01`: requires at least 33 bytes (1 version + 32 contextId).
 *   Reads 32-byte contextId starting at byte 1 as a `uint256` (`bigint`).
 *   Trailing bytes after byte 33 are ignored (forward-compat, matching contract).
 * - Other versions: throws `"Unsupported extraData version <N>"`
 *   (matching contract's `UnsupportedExtraDataVersion`)
 *
 * @param extraData - A non-legacy `BytesHex` value
 * @returns The parsed version and contextId
 */
export function parseExtraData(extraData: BytesHex): {
  version: number;
  contextId: UintBigInt;
} {
  // First byte = version (characters 2-3 after '0x')
  const version = parseInt(extraData.slice(2, 4), 16);

  if (version === 0x01) {
    // 1 version byte + 32 contextId bytes = 33 bytes = 66 hex chars + 2 for '0x' = 68
    if (extraData.length < 68) {
      throw new Error(
        `extraData too short for v1: expected at least 33 bytes, got ${(extraData.length - 2) / 2}`,
      );
    }
    // 32-byte contextId starts at byte 1 (hex chars 4..67)
    const contextIdHex = extraData.slice(4, 68);
    const contextId = BigInt("0x" + contextIdHex) as UintBigInt;
    return { version, contextId };
  }

  throw new Error(`Unsupported extraData version ${version}`);
}

/**
 * Constructs v1 extraData for outgoing requests.
 * Inverse of `parseExtraData`.
 *
 * Encodes `0x01` (version byte) + 32-byte big-endian `contextId` (`uint256`).
 *
 * @param contextId - The KMS context ID as a `bigint` (must be non-negative)
 * @returns A `BytesHex` value — always 33 bytes (66 hex chars + `0x` prefix)
 */
const MAX_UINT256 = 2n ** 256n - 1n;

export function buildRequestExtraData(contextId: UintBigInt): BytesHex {
  if (contextId < 0n || contextId > MAX_UINT256) {
    throw new Error(
      `contextId must be a non-negative uint256, got ${contextId}`,
    );
  }
  return `0x01${contextId.toString(16).padStart(64, "0")}` as BytesHex;
}
