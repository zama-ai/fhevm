// Scenario: the specimen decrypt vertical — one wallet drives the `encrypted-counter` specimen
// program (`solana/programs/encrypted-counter`) and decrypts what it wrote, through the SDK's
// one-path authorization: the request names the handle and its `EncryptedValue` account, and the
// KMS Connector reads the account, proves the wallet's allow leaf against the coprocessors, and
// releases the shares. The client fetches no proof and names no ACL key.
//
// Why a specimen program and not a raw `fhe_execute`: the host only accepts a value whose authority
// is a PDA of the program that declares it (RFC 035), so a wallet cannot own a value it wrote
// itself. The counter is the smallest program that does: `initialize` creates the owner's count at
// 0, `increment(amount)` adds to it, and both allow the owner on the handle they install.
//
// Assertion map:
//   [compute]  the write confirmed and installed a nonzero current handle -> `initializeCounter` /
//              `incrementCounter` return the handle they read back from the account.
//   [compute]  SNS commit -> `stack.waitForSnsCommit(handle)`.
//   [user-decrypt]  pure-SDK cleartext == expected -> `userDecryptExpect` (ML-KEM keygen, ed25519
//              permit, in-SDK de-signcryption) with UD_EXPECTED pinned.
//   [historical-user-decrypt]  a later write replaces the current handle, and the OLD handle still
//              decrypts to its old value: the Connector proves the old allow leaf from the account's
//              history, no client-side proof involved. The scenario asserts the update really
//              rotated the current handle before decrypting both.

import { describe, expect, test } from "bun:test";

import { userDecryptExpect } from "../../src/solana/fhe-vertical";
import { incrementCounter, initializeCounter } from "../../src/solana/specimens";
import { verticalSetup } from "../harness/solana/vertical";

// Each phase does its own write + SNS commit wait (up to ~3min) + KMS round-trips.
const SCENARIO_TIMEOUT_MS = 15 * 60_000;

const hex = (bytes: Uint8Array): string => `0x${Buffer.from(bytes).toString("hex")}`;

describe("solana specimen decrypt vertical", () => {
  test(
    "counter: initialize -> increment(42) -> pure-SDK user-decrypt == 42",
    async () => {
      const { stack, context, wallet, config, secretKey } = await verticalSetup();

      await initializeCounter(context, wallet.signer);
      const { value, handle } = await incrementCounter(context, wallet.signer, 42n);
      expect(handle.some((byte) => byte !== 0)).toBe(true);
      await stack.waitForSnsCommit(hex(handle));

      const decrypted = await userDecryptExpect(config, {
        encryptedValue: value.encryptedValue,
        handle,
        secretKey,
        expected: 42n,
      });
      expect(decrypted).toBe(42n);
    },
    SCENARIO_TIMEOUT_MS,
  );

  test(
    "historical decrypt: increment again, then user-decrypt the OLD handle and the current one",
    async () => {
      const { stack, context, wallet, config, secretKey } = await verticalSetup();

      await initializeCounter(context, wallet.signer);
      const original = await incrementCounter(context, wallet.signer, 42n);
      // The old handle must be SNS-committed before the update buries it.
      await stack.waitForSnsCommit(hex(original.handle));

      const updated = await incrementCounter(context, wallet.signer, 7n);
      expect(hex(updated.handle)).not.toBe(hex(original.handle));
      expect(updated.value.encryptedValue).toBe(original.value.encryptedValue);
      await stack.waitForSnsCommit(hex(updated.handle));

      const encryptedValue = original.value.encryptedValue;
      expect(await userDecryptExpect(config, { encryptedValue, handle: original.handle, secretKey, expected: 42n })).toBe(42n);
      expect(await userDecryptExpect(config, { encryptedValue, handle: updated.handle, secretKey, expected: 49n })).toBe(49n);
    },
    SCENARIO_TIMEOUT_MS,
  );
});
