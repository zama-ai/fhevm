import type { EthCallResult } from '../../modules/ethereum/types.js';
import type { FhevmRuntime } from '../../types/coreFhevmRuntime.js';
import type { KmsUserDecryptEip712V2 } from '../../types/kms.js';
import type { Bytes32Hex, Bytes65Hex, BytesHex, ChecksummedAddress } from '../../types/primitives.js';
import {
  Erc1271EmptySigOnEoaError,
  Erc1271EoaMismatchNoCodeError,
  Erc1271RejectedError,
  Erc1271WrongMagicError,
} from '../../errors/Erc1271Error.js';
import { getTrustedClient } from '../../runtime/CoreFhevm-p.js';

////////////////////////////////////////////////////////////////////////////////

/**
 * ERC-1271 magic return value of `isValidSignature(bytes32,bytes)`:
 * `bytes4(keccak256("isValidSignature(bytes32,bytes)"))`. It doubles as the
 * function selector, so it also prefixes the STATICCALL calldata.
 */
export const ERC1271_MAGIC_VALUE = '0x1626ba7e' as const;

/**
 * Gas cap for the `isValidSignature` STATICCALL. Matches the relayer /
 * KMS-connector default (`erc1271_gas_limit = 100000`) so all three layers
 * bound the wallet's verification the same way.
 */
export const ERC1271_GAS_LIMIT = 100_000n;

////////////////////////////////////////////////////////////////////////////////

/** Byte length of a `0x`-prefixed hex string. */
function byteLengthOfHex(hex: string): number {
  return (hex.length - 2) / 2;
}

////////////////////////////////////////////////////////////////////////////////

export type VerifyErc1271UserDecryptContext = {
  readonly runtime: FhevmRuntime;
  readonly client: NonNullable<object>;
};

export type VerifyErc1271UserDecryptParameters = {
  /** The claimed identity the signature must authenticate — the contract wallet. */
  readonly userAddress: ChecksummedAddress;
  /** The unified EIP-712 typed data the user signed (domain + struct + message). */
  readonly eip712: KmsUserDecryptEip712V2;
  /** The opaque signature blob (65-byte EOA, multisig concat, or empty `0x`). */
  readonly signature: BytesHex;
};

////////////////////////////////////////////////////////////////////////////////
// verifyErc1271UserDecrypt
////////////////////////////////////////////////////////////////////////////////

/**
 * Precautionary client-side port of the shared crate's `verify_signature`
 * (`shared/user-decryption-signature/src/lib.rs`), validating a user-decryption
 * signature against `userAddress` per RFC-012:
 *
 * 1. If the signature is 65 bytes and `ecrecover(digest) === userAddress`,
 *    accept locally (EOA fast path, no RPC).
 * 2. Otherwise STATICCALL `IERC1271(userAddress).isValidSignature(digest, sig)`
 *    and accept iff it returns the magic value `0x1626ba7e`.
 *
 * This check is **precautionary, not authoritative** (the KMS connector runs the
 * same algorithm and gates key release):
 * - A **definitive** local failure (wrong magic / revert / malformed returndata /
 *   empty signature or ecrecover mismatch on an EOA) throws — fail-fast, matching
 *   the relayer's sync 400.
 * - An **inconclusive** outcome (no read provider, or an RPC transport error)
 *   logs a warning and returns, forwarding the request to the KMS.
 *
 * @throws {@link Erc1271VerificationError} on a definitive local rejection.
 */
export async function verifyErc1271UserDecrypt(
  context: VerifyErc1271UserDecryptContext,
  parameters: VerifyErc1271UserDecryptParameters,
): Promise<void> {
  const { userAddress, eip712, signature } = parameters;
  const ethereum = context.runtime.ethereum;

  const { domain, primaryType, message } = eip712;
  const fields = eip712.types[primaryType];
  const types = { [primaryType]: [...fields] };

  const digest: Bytes32Hex = ethereum.hashTypedData({ domain, types, primaryType, message });

  const sigByteLength = byteLengthOfHex(signature);

  // 1. EOA fast path — local, no RPC. Only meaningful for a 65-byte signature.
  //    An unparsable blob (or a recovered address that isn't userAddress) simply
  //    falls through to the ERC-1271 STATICCALL, never rejecting outright.
  if (sigByteLength === 65) {
    try {
      const recovered = await ethereum.recoverTypedDataAddress({
        domain,
        types,
        primaryType,
        message,
        signature: signature as Bytes65Hex,
      });
      if (recovered.toLowerCase() === userAddress.toLowerCase()) {
        return;
      }
    } catch {
      // Unparsable 65-byte blob (e.g. a Safe eth_sign / approveHash shape) —
      // fall through to ERC-1271.
    }
  }

  // 2. ERC-1271 STATICCALL. The magic value doubles as the function selector.
  const encodedArgs = ethereum.encode({ types: ['bytes32', 'bytes'], values: [digest, signature] });
  const calldata = `${ERC1271_MAGIC_VALUE}${encodedArgs.slice(2)}` as BytesHex;

  let result: EthCallResult;
  try {
    const trustedClient = getTrustedClient(context);
    result = await ethereum.call(trustedClient, {
      to: userAddress,
      data: calldata,
      gas: ERC1271_GAS_LIMIT,
    });
  } catch (err) {
    // Inconclusive: no read provider or a transport-level error. The KMS is the
    // authority — degrade gracefully and forward rather than hard-failing.
    const reason = err instanceof Error ? err.message : String(err);
    context.runtime.config.logger?.warn?.(
      `ERC-1271 user-decryption signature check could not complete for userAddress ${userAddress}; ` +
        `forwarding to the KMS (authoritative). Reason: ${reason}`,
    );
    return;
  }

  if (!result.success) {
    // A clean EVM revert is a definitive rejection.
    throw new Erc1271RejectedError({ userAddress, detail: `isValidSignature call reverted: ${result.reason}` });
  }

  const returndata = result.data;
  const returnByteLength = byteLengthOfHex(returndata);

  if (returnByteLength === 0) {
    // STATICCALL to a no-code address succeeds with empty returndata — the
    // dominant signal that userAddress is an EOA.
    if (sigByteLength === 0) {
      throw new Erc1271EmptySigOnEoaError({ userAddress });
    }
    throw new Erc1271EoaMismatchNoCodeError({ userAddress });
  }

  // Solidity ABI-encodes `bytes4` as a full 32-byte word; a non-compliant
  // fallback returning fewer bytes is rejected before matching the magic value.
  if (returnByteLength < 32) {
    throw new Erc1271RejectedError({ userAddress, detail: `returndata length ${returnByteLength} < 32` });
  }

  const magicValue = returndata.slice(0, 10).toLowerCase();
  if (magicValue === ERC1271_MAGIC_VALUE) {
    return;
  }

  throw new Erc1271WrongMagicError({ userAddress, magicValue });
}
