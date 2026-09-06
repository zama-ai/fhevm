// Unit cover for the on-chain public-leaf proof builder: the proof it hands the consume steps must
// verify against the peaks it was cross-checked with, and a live account that disagrees with the
// scenario's history must fail HERE, naming the leaf count, not later inside the on-chain verifier.

import { describe, expect, test } from "bun:test";
import { address, getAddressEncoder } from "@solana/kit";

import {
  reconstructSolanaEncryptedValueAccount,
  verifyPublicDecryptProof,
  type SolanaEncryptedValueAccountEvent,
} from "@sdk-src/solana/proof.js";

import { publicLeafProof } from "./fhe-vertical";

const ENCRYPTED_VALUE = address("SysvarC1ock11111111111111111111111111111111");
const HANDLE = new Uint8Array(32).fill(0x92);
const OWNER = new Uint8Array(32).fill(0x11);
const encryptedValueBytes = new Uint8Array(getAddressEncoder().encode(ENCRYPTED_VALUE));

// What a burn writes, then an explicit re-seal: one allow, the public leaf, the public leaf again.
const history: readonly SolanaEncryptedValueAccountEvent[] = [
  { kind: "allowed", handle: HANDLE, key: OWNER },
  { kind: "markedPublic", handle: HANDLE },
  { kind: "markedPublic", handle: HANDLE },
];
const live = reconstructSolanaEncryptedValueAccount(encryptedValueBytes, history);

describe("publicLeafProof", () => {
  test("builds a proof of the requested public leaf that verifies against the live peaks", () => {
    const proof = publicLeafProof(ENCRYPTED_VALUE, live, history, 1n);
    expect(proof.leafIndex).toBe(1n);
    expect(verifyPublicDecryptProof(encryptedValueBytes, live.peaks, live.leafCount, HANDLE, proof)).toBe(true);
  });

  test("rejects a live account whose leaves disagree with the expected history", () => {
    const shorter = reconstructSolanaEncryptedValueAccount(encryptedValueBytes, history.slice(0, 2));
    expect(() => publicLeafProof(ENCRYPTED_VALUE, shorter, history, 1n)).toThrow(/holds 2 leaves that do not match the 3-leaf history/);
  });

  test("refuses to prove a leaf that is not public", () => {
    expect(() => publicLeafProof(ENCRYPTED_VALUE, live, history, 0n)).toThrow(/not a public leaf/);
  });
});
