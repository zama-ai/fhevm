import { ethers as EthersT } from 'ethers';

import { HardhatFhevmError } from '../error';
import { FhevmType, type FhevmTypeName, getFhevmTypeName } from './fheType';

/**
 * Decoding of a bytes32 FHE handle.
 *
 * The layout is fixed by the protocol (see `FHEVMExecutor`), and the plugin only needs to read it —
 * handles are produced on-chain and by `@fhevm/sdk`, never here:
 *
 * ```
 *   bytes  0-20  keccak-derived hash
 *   byte     21  input index, or 0xff when the handle is the result of a computation
 *   bytes 22-29  chain id
 *   byte     30  FHE type
 *   byte     31  handle version
 * ```
 *
 * Ported from `@fhevm/mock-utils`' `FhevmHandle`, reduced to what the plugin still uses: the type
 * (for `fhevm.typeof` and for HCU pricing) and the computed/input flag.
 */
export type FhevmHandleInfo = {
  readonly handleBytes32Hex: string;
  readonly chainId: number;
  readonly fhevmType: FhevmType;
  readonly typeName: FhevmTypeName;
  /** `true` when the handle came out of an FHE operation rather than a user input. */
  readonly computed: boolean;
  readonly version: number;
};

/** The handle a Solidity FHE variable holds before anything is ever assigned to it. */
const UNINITIALIZED_HANDLE = `0x${'0'.repeat(64)}`;

/**
 * Rejects the zero handle with the diagnosis that actually applies.
 *
 * A contract returns `bytes32(0)` for an FHE value that was never written — an uninitialized
 * variable, which is much the most common way a decryption call fails. Left to itself the SDK
 * decodes the zero word structurally and reports `Handle (0x000…) has chainId 0, expected <id>`,
 * which is true but reads like a network misconfiguration and sends the reader hunting in the wrong
 * place. Checking here, before the handle reaches the SDK, keeps the error pointed at the cause.
 */
export function assertHandleIsInitialized(handleBytes32: string): void {
  if (typeof handleBytes32 === 'string' && handleBytes32.toLowerCase() === UNINITIALIZED_HANDLE) {
    throw new HardhatFhevmError(`Handle is not initialized`);
  }
}

export function parseFhevmHandle(handleBytes32: string): FhevmHandleInfo {
  // `isHexString` rejects non-strings itself, so it still covers an untyped caller on its own.
  if (!EthersT.isHexString(handleBytes32, 32)) {
    throw new HardhatFhevmError(`Invalid FHE handle '${handleBytes32}': expected a 32-byte 0x-prefixed hex string.`);
  }

  const byte = (i: number): number => Number.parseInt(handleBytes32.slice(2 + i * 2, 4 + i * 2), 16);

  const index = byte(21);
  const chainId = Number(BigInt('0x' + handleBytes32.slice(46, 62)));
  const typeByte = byte(30);
  const version = byte(31);

  if (!(typeByte in FhevmType)) {
    throw new HardhatFhevmError(
      `Invalid FHE handle '${handleBytes32}': byte 30 is not a known FHE type (got 0x${typeByte.toString(16)}).`,
    );
  }

  return {
    handleBytes32Hex: handleBytes32,
    chainId,
    fhevmType: typeByte,
    typeName: getFhevmTypeName(typeByte),
    computed: index === 255,
    version,
  };
}
