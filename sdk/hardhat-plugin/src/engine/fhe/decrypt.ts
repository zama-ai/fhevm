import { AbiCoder, type Signer, type TypedDataField, concat, getAddress, getBytes, toBeHex } from "ethers";

import {
  EIP712_DECRYPTION_DOMAIN_NAME,
  EIP712_DOMAIN_VERSION,
  GATEWAY_DECRYPTION_ADDRESS,
} from "../stack/config";
import { FheType } from "./fhetype";
import { FhevmNode } from "../node";
import { readCleartext } from "../stack/rpc";

/**
 * Decryption.
 *
 * This moved wholesale. It used to be two view getters on ACL (`plaintextForUserDecryption` /
 * `plaintextForPublicDecryption`) returning raw `uint256[]`. Those are gone; decryption now lives on
 * `CleartextKMSVerifier` and mimics the production wire format:
 *
 *  - The user's EIP-712 signature is verified ON-CHAIN, not just here. It must be exact.
 *  - Results come back XOR-masked with the first 32 bytes of the caller's `publicKey`, standing in for the
 *    KMS re-encrypting under the user's transport key. We unmask with the same 32 bytes.
 *  - `extraData` (the KMS context id) is built ON-CHAIN and is covered by the signature, so we must
 *    reconstruct it byte-for-byte before signing — hence the `getCurrentKmsContextId` read below.
 *
 * ACL enforcement still lives on-chain and still reverts (`_requireAllPairsAuthorized`); we let it propagate.
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

/**
 * `CleartextKMSVerifier._buildCurrentExtradata()`: a 33-byte blob, `0x01` followed by the current KMS
 * context id as a uint256. The contract rebuilds this itself and hashes it into the digest it verifies our
 * signature against, so it has to be reproduced exactly — a mismatch surfaces as an opaque
 * `InvalidUserDecryptSignature`.
 */
async function buildCurrentExtraData(node: FhevmNode): Promise<string> {
  const [contextId] = await readCleartext(node, "KMSVerifier", "getCurrentKmsContextId");
  return concat(["0x01", toBeHex(BigInt(contextId as bigint), 32)]);
}

/**
 * The user-decrypt EIP-712 domain.
 *
 * The chain id here is the HOST chain id, NOT the gateway one. `CleartextKMSVerifier` is initialized with
 * the gateway chain id and uses it for public decryption, but user decryption deliberately overrides it to
 * `block.chainid` (`_domainHashWithHostChainId`). The verifying contract stays the gateway-side address.
 * Using the gateway chain id here recovers a different signer and fails on-chain.
 */
function userDecryptionDomain(hostChainId: number) {
  return {
    name: EIP712_DECRYPTION_DOMAIN_NAME,
    version: EIP712_DOMAIN_VERSION,
    chainId: hostChainId,
    verifyingContract: GATEWAY_DECRYPTION_ADDRESS,
  };
}

/** Reverses the mock's "encrypt-for-user": each value is XORed with the first 32 bytes of the public key. */
function unmask(values: bigint[], publicKey: string): bigint[] {
  const keyBytes = getBytes(publicKey);
  if (keyBytes.length < 32) {
    throw new Error(`publicKey must be at least 32 bytes (got ${keyBytes.length}).`);
  }
  const mask = BigInt(toBeHex(BigInt("0x" + Buffer.from(keyBytes.subarray(0, 32)).toString("hex")), 32));
  return values.map((v) => v ^ mask);
}

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
 * User-decrypts one or more handles via `CleartextKMSVerifier.userDecrypt`.
 *
 * `contractAddresses` must cover every contract in `pairs` — the contract cross-checks them
 * (`_requireAllPairsAuthorized`) and reverts otherwise.
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
  const publicKey = params.keypair.publicKey.startsWith("0x")
    ? params.keypair.publicKey
    : `0x${params.keypair.publicKey}`;

  const extraData = await buildCurrentExtraData(node);
  const message = { publicKey, contractAddresses, startTimestamp, durationDays, extraData };
  const signature = await params.user.signTypedData(userDecryptionDomain(node.chainId), USER_DECRYPT_TYPES, message);

  // ACL-enforcing on-chain read. Reverts (and we let it) if the user or a contract is not authorized.
  const pairsTuple = params.pairs.map((p) => [p.handle, getAddress(p.contractAddress)]);
  const [payload] = await readCleartext(node, "KMSVerifier", "userDecrypt", [
    pairsTuple,
    userAddress,
    publicKey,
    contractAddresses,
    startTimestamp,
    durationDays,
    signature,
  ]);

  // payload = abi.encode(uint256[] maskedCleartexts, bytes extraData)
  const [masked] = AbiCoder.defaultAbiCoder().decode(["uint256[]", "bytes"], payload as string);
  return unmask((masked as bigint[]).map((v) => BigInt(v)), publicKey);
}

/**
 * Public-decrypts handles that have been marked publicly decryptable.
 * `CleartextKMSVerifier.publicDecrypt` enforces `isAllowedForDecryption` on-chain and reverts otherwise.
 *
 * Its first return value is NOT standard ABI encoding: `_encodeTypedCleartexts` hand-packs one right-aligned
 * 32-byte word per handle. So we read it as fixed-width words rather than decoding a `uint256[]`.
 */
export async function publicDecrypt(node: FhevmNode, handles: string[]): Promise<bigint[]> {
  const [encoded] = await readCleartext(node, "KMSVerifier", "publicDecrypt", [handles]);
  const bytes = getBytes(encoded as string);
  if (bytes.length !== handles.length * 32) {
    throw new Error(`publicDecrypt returned ${bytes.length} bytes, expected ${handles.length * 32}.`);
  }

  const values: bigint[] = [];
  for (let i = 0; i < handles.length; i++) {
    const word = bytes.subarray(i * 32, (i + 1) * 32);
    values.push(BigInt("0x" + Buffer.from(word).toString("hex")));
  }
  return values;
}

/**
 * A ceremonial transport keypair. In production this is an ML-KEM keypair the KMS re-encrypts under; here
 * only the public half matters, and only its first 32 bytes, which the contract uses as the XOR mask. The
 * private half is never used.
 */
export function generateKeypair(): UserDecryptKeypair {
  return {
    publicKey: "0x" + "de".repeat(32),
    privateKey: "0x" + "ad".repeat(32),
  };
}
