// Drift pins for the local EncryptedValue lineage derivation copy (encryptedValueLineage.ts).
// Both goldens are borrowed from the SDK's own pinned suites so the local copy and the SDK copy
// can only drift apart by failing here, in the cheap lane — not 20 minutes into the live gate.

import { describe, expect, test } from "bun:test";

import type { Address } from "@solana/kit";

import { BALANCE_LABEL, TRANSFERRED_AMOUNT_LABEL, deriveValueKey, encryptedValueAddress } from "./encryptedValueLineage";

describe("encryptedValueLineage", () => {
  test("deriveValueKey matches the Rust crate golden vector", () => {
    // Same vector as sdk/js-sdk/src/solana/proof.test.ts ('matches the Rust crate vector'):
    // inputs are 32 bytes of 0x01 / 0x02 / 0x03; expected output captured from
    // `cargo run -p kms-worker --example mmr_vectors` against solana/crates/zama-solana-acl.
    const valueKey = deriveValueKey(
      new Uint8Array(32).fill(1),
      new Uint8Array(32).fill(2),
      new Uint8Array(32).fill(3),
    );
    expect(Buffer.from(valueKey).toString("hex")).toBe(
      "cb421159e2c7709e401334c46b4bcee90093cb616d040fca9c1dc9a14ad77820",
    );
  });

  test("balance lineage PDA matches the SDK golden (label + PDA step, not just the hash)", async () => {
    // Same fixture as sdk/js-sdk/src/solana/vault/derive.test.ts's consensus-critical golden:
    // aclDomain = payoutConfidentialMint = base58(32 x 0x0d), appAccount = the golden
    // batchPayoutTokenAccount, expected = golden batchPayoutBalanceValue. The SDK derives lineage
    // PDAs under its generated ZAMA_HOST_PROGRAM_ADDRESS constant (batcherPdas.ts), not the roots
    // fixture's hostProgram — so that constant is the host program here.
    const hostProgram = "6AtbvED1rfX68aCT1tYgU1aeu4kFksPDxZG9gtB1Fgtu" as Address;
    const payoutConfidentialMint = "swqrv48gsrwpBFbftEwnP2vB4jckpvfGJfXkwaniLCC" as Address;
    const batchPayoutTokenAccount = "8iRxqzbzVoCDyN5ruCrtDs3HEJXL6S5khbmijMta8j6z" as Address;
    const balanceValue = await encryptedValueAddress(
      hostProgram,
      payoutConfidentialMint,
      batchPayoutTokenAccount,
      BALANCE_LABEL,
    );
    expect(balanceValue).toBe("6L34CwYQLjs4e5sHTjCsoNk5UBZwDtTMkKegf7tRdoM7" as Address);
  });

  test("lineage labels are the exact 32-byte underscore-padded strings the token program stores", () => {
    const decode = (bytes: Uint8Array): string => new TextDecoder().decode(bytes);
    expect(decode(BALANCE_LABEL)).toBe("balance_________________________");
    expect(decode(TRANSFERRED_AMOUNT_LABEL)).toBe("transferred_amount______________");
    expect(BALANCE_LABEL.length).toBe(32);
    expect(TRANSFERRED_AMOUNT_LABEL.length).toBe(32);
  });
});
