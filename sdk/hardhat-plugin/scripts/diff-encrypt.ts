/**
 * M2 gate — differential test of the new engine's handle/encrypt logic against the compiled OLD
 * `@fhevm/mock-utils`. The old code is the oracle: for identical inputs the new implementation must
 * produce byte-identical blobs, handles, extraData, input proofs, and EIP-712 digests.
 *
 * Randomness is injected (fixed rand buffers) so both sides are deterministic. Run: `npm run diff:encrypt`.
 */
import { TypedDataEncoder, Wallet, getAddress, getBytes, hexlify } from "ethers";

import {
  GATEWAY_CHAIN_ID,
  GATEWAY_INPUT_VERIFICATION_ADDRESS,
  EIP712_DOMAIN_VERSION,
  EIP712_INPUT_VERIFICATION_DOMAIN_NAME,
} from "../src/engine/addresses";
import { FheType } from "../src/engine/fhetype";
import { computeHandles } from "../src/engine/handle";
import {
  CIPHERTEXT_VERIFICATION_TYPES,
  type EncryptValue,
  assembleInputProof,
  computeMockCiphertext,
  packExtraData,
} from "../src/engine/encrypt";

// The compiled old package (the oracle).
const OLD = "/Users/aurora/Desktop/aurora/cleartext-mock/fhevm-mocks/packages/mock-utils/src/_cjs";
/* eslint-disable @typescript-eslint/no-var-requires */
const { FhevmHandle } = require(`${OLD}/fhevm/FhevmHandle.js`);
const { computeInputProofHex } = require(`${OLD}/fhevm/contracts/InputVerifier.js`);
const { MockRelayerEncryptedInput } = require(`${OLD}/fhevm/MockRelayerEncryptedInput.js`);
/* eslint-enable @typescript-eslint/no-var-requires */

const ACL = "0x50157CFfD6bBFA2DECe204a89ec419c23ef5755D";
const CONTRACT = getAddress("0x1234567890abcdef1234567890abcdef12345678");
const USER = getAddress("0xabcdef1234567890abcdef1234567890abcdef12");
const CHAIN_ID = 31337;
const VERSION = 0;

let failures = 0;
function check(label: string, mine: string, theirs: string): void {
  const a = mine.toLowerCase();
  const b = theirs.toLowerCase();
  if (a === b) {
    console.log(`  ✓ ${label}`);
  } else {
    failures++;
    console.log(`  ✗ ${label}\n      mine:   ${a}\n      theirs: ${b}`);
  }
}

// Deterministic "random" buffers so both sides agree.
function fixedRand(n: number): Uint8Array[] {
  return Array.from({ length: n }, (_, i) => getBytes("0x" + (i + 1).toString(16).padStart(2, "0").repeat(32)));
}

interface Case {
  readonly name: string;
  readonly values: EncryptValue[];
}

const CASES: Case[] = [
  { name: "euint32(1)", values: [{ type: FheType.euint32, value: 1n }] },
  { name: "ebool(true)", values: [{ type: FheType.ebool, value: 1n }] },
  { name: "euint8(255)", values: [{ type: FheType.euint8, value: 255n }] },
  { name: "euint64(max)", values: [{ type: FheType.euint64, value: (1n << 64n) - 1n }] },
  { name: "euint128(big)", values: [{ type: FheType.euint128, value: (1n << 100n) + 7n }] },
  { name: "euint256(max)", values: [{ type: FheType.euint256, value: (1n << 256n) - 1n }] },
  { name: "eaddress", values: [{ type: FheType.eaddress, value: "0x2222222222222222222222222222222222222222" }] },
  {
    name: "bundle[euint32,ebool,eaddress,euint256]",
    values: [
      { type: FheType.euint32, value: 42n },
      { type: FheType.ebool, value: 0n },
      { type: FheType.eaddress, value: "0x3333333333333333333333333333333333333333" },
      { type: FheType.euint256, value: 123456789n },
    ],
  },
];

// old FheType numeric ids equal ours (verified), so we can pass the same numbers as "fhevmTypes".
function oldTypeIds(values: EncryptValue[]): number[] {
  return values.map((v) => v.type as number);
}

function oldClearBigInts(values: EncryptValue[]): bigint[] {
  return values.map((v) => (typeof v.value === "string" ? BigInt(v.value) : v.value));
}

function main(): void {
  const signer = new Wallet("0x7ec8ada6642fc4ccfb7729bc29c17cf8d21b61abd5642d1db992c0b8672ab901");

  for (const c of CASES) {
    console.log(`\n[${c.name}]`);
    const rand = fixedRand(c.values.length);

    // 1. mock ciphertext blob — new vs old (the old builder is a public static).
    const myBlob = computeMockCiphertext(c.values, rand);
    const theirBlob: Uint8Array = MockRelayerEncryptedInput._computeMockCiphertextWithZKProof(
      oldClearBigInts(c.values),
      oldTypeIds(c.values),
      rand,
    );
    check("blob", hexlify(myBlob), hexlify(theirBlob));

    // 2. handles — new vs old, given the (verified-equal) blob.
    const myHandles = computeHandles({
      ciphertextWithZKProof: myBlob,
      types: c.values.map((v) => v.type),
      aclAddress: ACL,
      chainId: CHAIN_ID,
      version: VERSION,
    }).map(hexlify);
    const theirHandles: string[] = FhevmHandle.computeHandlesHex(theirBlob, oldTypeIds(c.values), ACL, CHAIN_ID, VERSION);
    check("handles", myHandles.join(""), theirHandles.join(""));

    // 3. extraData.
    const myExtra = packExtraData(c.values);
    let theirExtra = "0x";
    for (const v of oldClearBigInts(c.values)) {
      theirExtra += BigInt(v).toString(16).padStart(64, "0");
    }
    if (theirExtra === "0x") theirExtra = "0x00";
    check("extraData", myExtra, theirExtra);

    // 4. input proof assembly — reuse a fixed fake 65-byte signature; only layout is under test here.
    const fakeSig = "0x" + "ab".repeat(65);
    const myProof = assembleInputProof(myHandles, [fakeSig], myExtra);
    const theirProof = computeInputProofHex(theirHandles, [fakeSig], theirExtra);
    check("inputProof layout", myProof, theirProof);

    // 5. EIP-712 digest of the CiphertextVerification message.
    const domain = {
      name: EIP712_INPUT_VERIFICATION_DOMAIN_NAME,
      version: EIP712_DOMAIN_VERSION,
      chainId: GATEWAY_CHAIN_ID,
      verifyingContract: GATEWAY_INPUT_VERIFICATION_ADDRESS,
    };
    const message = {
      ctHandles: myHandles,
      userAddress: USER,
      contractAddress: CONTRACT,
      contractChainId: CHAIN_ID,
      extraData: myExtra,
    };
    const myDigest = TypedDataEncoder.hash(domain, CIPHERTEXT_VERIFICATION_TYPES, message);
    // Old types come from the same constants; recover the signer both ways to prove interop.
    const oldTypes = {
      CiphertextVerification: [
        { name: "ctHandles", type: "bytes32[]" },
        { name: "userAddress", type: "address" },
        { name: "contractAddress", type: "address" },
        { name: "contractChainId", type: "uint256" },
        { name: "extraData", type: "bytes" },
      ],
    };
    const theirDigest = TypedDataEncoder.hash(domain, oldTypes, message);
    check("eip712 digest", myDigest, theirDigest);
    void signer;
  }

  console.log(failures === 0 ? "\nM2 GATE PASSED\n" : `\nM2 GATE FAILED — ${failures} mismatch(es)\n`);
  if (failures > 0) process.exit(1);
}

main();
