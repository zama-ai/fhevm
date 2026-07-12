import { type Signer, type TypedDataField, getAddress, verifyTypedData } from "ethers";

import {
  EIP712_DECRYPTION_DOMAIN_NAME,
  EIP712_DOMAIN_VERSION,
  GATEWAY_CHAIN_ID,
  GATEWAY_DECRYPTION_ADDRESS,
} from "./addresses";
import { FheType } from "./fhetype";
import { FhevmNode } from "./node";
import { readCleartext } from "./rpc";

/**
 * Decrypt is where ACL enforcement lives, so two invariants are load-bearing:
 *
 *  1. The plaintext read goes through the ACL-ENFORCING on-chain methods
 *     (`plaintextForUserDecryption` / `plaintextForPublicDecryption`), which revert if the caller/user
 *     is not authorized. We let that revert PROPAGATE. The raw `plaintexts(handle)` getter is never used
 *     here — swallowing its revert (as the old mock did) would turn "not permitted" into a silent 0.
 *  2. The user's EIP-712 signature is verified in TS for parity with production (a wrong signer fails),
 *     even though the on-chain read authorizes by `userAddress` + ACL, not by the signature.
 */

const USER_DECRYPT_TYPES: Record<string, TypedDataField[]> = {
  UserDecryptRequestVerification: [
    { name: "publicKey", type: "bytes" },
    { name: "contractAddresses", type: "address[]" },
    { name: "startTimestamp", type: "uint256" },
    { name: "durationDays", type: "uint256" },
    { name: "extraData", type: "bytes" },
  ],
};

/** The cross-chain Decryption domain: gateway chain id + gateway Decryption contract. */
function decryptionDomain() {
  return {
    name: EIP712_DECRYPTION_DOMAIN_NAME,
    version: EIP712_DOMAIN_VERSION,
    chainId: GATEWAY_CHAIN_ID,
    verifyingContract: GATEWAY_DECRYPTION_ADDRESS,
  };
}

export interface HandleContractPair {
  readonly handle: string;
  readonly contractAddress: string;
}

export interface UserDecryptKeypair {
  readonly publicKey: string;
  readonly privateKey: string;
}

const DEFAULT_START_TIMESTAMP = 0;
const DEFAULT_DURATION_DAYS = 365;
const USER_DECRYPT_EXTRA_DATA = "0x00";

/**
 * Formats a raw uint256 cleartext according to the requested FHE type: ebool -> boolean,
 * eaddress -> checksummed address, everything else -> bigint.
 */
export function formatDecrypted(raw: bigint, type: FheType): boolean | bigint | string {
  if (type === FheType.ebool) {
    return raw === 1n;
  }
  if (type === FheType.eaddress) {
    return getAddress("0x" + raw.toString(16).padStart(40, "0"));
  }
  return raw;
}

/**
 * User-decrypts one or more handles. The user signs a `UserDecryptRequestVerification` (verified here
 * for parity), then the on-chain `plaintextForUserDecryption` authorizes by ACL and returns the values.
 * A user without `persistAllowed` on a handle makes that call revert — which propagates.
 */
export async function userDecrypt(
  node: FhevmNode,
  params: {
    pairs: HandleContractPair[];
    user: Signer;
    keypair: UserDecryptKeypair;
    startTimestamp?: number;
    durationDays?: number;
  },
): Promise<bigint[]> {
  const userAddress = getAddress(await params.user.getAddress());
  const contractAddresses = [...new Set(params.pairs.map((p) => getAddress(p.contractAddress)))];
  const startTimestamp = params.startTimestamp ?? DEFAULT_START_TIMESTAMP;
  const durationDays = params.durationDays ?? DEFAULT_DURATION_DAYS;

  const message = {
    publicKey: params.keypair.publicKey.startsWith("0x") ? params.keypair.publicKey : `0x${params.keypair.publicKey}`,
    contractAddresses,
    startTimestamp,
    durationDays,
    extraData: USER_DECRYPT_EXTRA_DATA,
  };
  const domain = decryptionDomain();
  const signature = await params.user.signTypedData(domain, USER_DECRYPT_TYPES, message);

  // Parity check: the signature must recover to the requesting user. On-chain authorization is by
  // userAddress + ACL, not by this signature, but a wrong signer should still fail (as in production).
  const recovered = verifyTypedData(domain, USER_DECRYPT_TYPES, message, signature);
  if (getAddress(recovered) !== userAddress) {
    throw new Error(`User-decrypt signature does not match the requesting user (${recovered} != ${userAddress}).`);
  }

  // ACL-enforcing on-chain read. Reverts (and we let it) if the user or the contract is not authorized.
  const pairsTuple = params.pairs.map((p) => [p.handle, getAddress(p.contractAddress)]);
  const result = await readCleartext(node, "ACL", "plaintextForUserDecryption", [pairsTuple, userAddress]);
  return (result[0] as bigint[]).map((v) => BigInt(v));
}

/**
 * Public-decrypts handles that have been marked publicly decryptable. `plaintextForPublicDecryption`
 * enforces `isAllowedForDecryption` on-chain and reverts otherwise.
 */
export async function publicDecrypt(node: FhevmNode, handles: string[]): Promise<bigint[]> {
  const result = await readCleartext(node, "ACL", "plaintextForPublicDecryption", [handles]);
  return (result[0] as bigint[]).map((v) => BigInt(v));
}

/**
 * A ceremonial transport keypair. In production this is an ML-KEM keypair the KMS re-encrypts under; in
 * the mock the value is read cleartext on-chain, so only its presence and byte-length in the EIP-712
 * message matter — the private half is never used. Padded to ML-KEM sizes, as the old mock did.
 */
export function generateKeypair(): UserDecryptKeypair {
  return {
    publicKey: "0x" + "de".repeat(32),
    privateKey: "0x" + "ad".repeat(32),
  };
}
