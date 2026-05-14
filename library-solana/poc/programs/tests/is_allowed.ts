import * as anchor from "@coral-xyz/anchor";
import { Keypair } from "@solana/web3.js";
import { assert, expect } from "chai";
import {
  allowKey,
  ensureConfigInitialized,
  initPermissionList,
  makeHandle,
  program,
} from "./utils";

describe("acl :: is_allowed", () => {
  before(ensureConfigInitialized);

  // `is_allowed` returns `Result<()>` — success = subject is in the allow
  // list, error = it isn't (or the handle is wrong). Anchor's `.view()`
  // requires a return value, so we use `.rpc()`: success resolves, failure
  // rejects with an `AnchorError` we can match against.

  it("succeeds when the subject is in allowed_accounts", async () => {
    const handle = makeHandle(20);
    const initialKey = Keypair.generate().publicKey;
    const outputIndex = new anchor.BN(0);
    const permissionList = await initPermissionList(
      handle,
      initialKey,
      outputIndex
    );
    const subject = Keypair.generate().publicKey;
    await allowKey(handle, permissionList, subject, initialKey, outputIndex);

    await program.methods
      .isAllowed(handle, subject, initialKey, outputIndex)
      .accountsPartial({ permissionList })
      .rpc();
  });

  it("reverts with HandleAuthorizationFailed when the subject is not in allowed_accounts", async () => {
    const handle = makeHandle(21);
    const initialKey = Keypair.generate().publicKey;
    const outputIndex = new anchor.BN(0);
    const permissionList = await initPermissionList(
      handle,
      initialKey,
      outputIndex
    );
    // Allow some other key, not the one we'll query for.
    await allowKey(
      handle,
      permissionList,
      Keypair.generate().publicKey,
      initialKey,
      outputIndex
    );
    const outsider = Keypair.generate().publicKey;

    try {
      await program.methods
        .isAllowed(handle, outsider, initialKey, outputIndex)
        .accountsPartial({ permissionList })
        .rpc();
      assert.fail("expected HandleAuthorizationFailed");
    } catch (err: any) {
      const code = err?.error?.errorCode?.code ?? "";
      const msg = code + " " + (err?.message ?? "");
      expect(msg).to.match(/HandleAuthorizationFailed/);
    }
  });

  it("reverts on a freshly initialized permission_list with no subjects", async () => {
    const handle = makeHandle(22);
    const initialKey = Keypair.generate().publicKey;
    const outputIndex = new anchor.BN(0);
    const permissionList = await initPermissionList(
      handle,
      initialKey,
      outputIndex
    );
    const someone = Keypair.generate().publicKey;

    try {
      await program.methods
        .isAllowed(handle, someone, initialKey, outputIndex)
        .accountsPartial({ permissionList })
        .rpc();
      assert.fail("expected HandleAuthorizationFailed");
    } catch (err: any) {
      const code = err?.error?.errorCode?.code ?? "";
      const msg = code + " " + (err?.message ?? "");
      expect(msg).to.match(/HandleAuthorizationFailed/);
    }
  });

  it("reverts with HandleMismatch when the handle argument doesn't match the account", async () => {
    const handle = makeHandle(23);
    const initialKey = Keypair.generate().publicKey;
    const outputIndex = new anchor.BN(0);
    const permissionList = await initPermissionList(
      handle,
      initialKey,
      outputIndex
    );
    const wrongHandle = handle.map((b) => b ^ 0xff);
    const someone = Keypair.generate().publicKey;

    try {
      await program.methods
        .isAllowed(wrongHandle, someone, initialKey, outputIndex)
        .accountsPartial({ permissionList })
        .rpc();
      assert.fail("expected HandleMismatch");
    } catch (err: any) {
      const code = err?.error?.errorCode?.code ?? "";
      const msg = code + " " + (err?.message ?? "");
      expect(msg).to.match(/HandleMismatch/);
    }
  });
});
