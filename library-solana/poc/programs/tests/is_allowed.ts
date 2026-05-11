import {
  Keypair,
  PublicKey,
  SystemProgram,
  Transaction,
} from "@solana/web3.js";
import { assert, expect } from "chai";
import {
  allowKey,
  ensureConfigInitialized,
  initPermissionList,
  makeHandle,
  program,
  provider,
} from "./utils";

describe("acl :: is_allowed", () => {
  before(ensureConfigInitialized);

  // .view() simulates the tx with the provider wallet as the (default) payer.
  // The program returns `payer.key ∈ allowed_accounts`, so to query "is the
  // wallet allowed?", we just call .view() after allow-ing the wallet.

  it("returns true when the caller is in allowed_accounts", async () => {
    const handle = makeHandle(20);
    const permissionList = await initPermissionList(handle);
    await allowKey(handle, permissionList.publicKey, provider.wallet.publicKey);

    const result = await program.methods
      .isAllowed(handle)
      .accountsPartial({ permissionList: permissionList.publicKey })
      .view();

    assert.strictEqual(result, true);
  });

  it("returns false when the caller is not in allowed_accounts", async () => {
    const handle = makeHandle(21);
    const permissionList = await initPermissionList(handle);
    // Allow some other key, not the provider wallet.
    await allowKey(
      handle,
      permissionList.publicKey,
      Keypair.generate().publicKey
    );

    const result = await program.methods
      .isAllowed(handle)
      .accountsPartial({ permissionList: permissionList.publicKey })
      .view();

    assert.strictEqual(result, false);
  });

  it("returns false for a freshly initialized permission_list", async () => {
    const handle = makeHandle(22);
    const permissionList = await initPermissionList(handle);

    const result = await program.methods
      .isAllowed(handle)
      .accountsPartial({ permissionList: permissionList.publicKey })
      .view();

    assert.strictEqual(result, false);
  });

  it("reverts with HandleMismatch when the handle argument doesn't match the account", async () => {
    const handle = makeHandle(23);
    const permissionList = await initPermissionList(handle);
    const wrongHandle = handle.map((b) => b ^ 0xff);

    try {
      await program.methods
        .isAllowed(wrongHandle)
        .accountsPartial({ permissionList: permissionList.publicKey })
        .view();
      assert.fail("expected HandleMismatch");
    } catch (err: any) {
      // .view() simulates the tx, so failures surface as a SimulateError whose
      // program logs Anchor never translates into an AnchorError. The error
      // identifier lives in `err.simulationResponse.logs`.
      const logs = (err?.simulationResponse?.logs ?? err?.logs ?? []).join("\n");
      expect(logs).to.match(/HandleMismatch/);
    }
  });

  it("rejects an account at a permission_list-shaped address that is not owned by the ACL program", async () => {
    // Set up a genuine permission_list so we have a real handle to mirror.
    const handle = makeHandle(24);
    await initPermissionList(handle);

    // Build a fake "permission_list" at a deterministic address derived from
    // a *different* program (here: the System Program). It is sized like a
    // real HandlerPermissions account, but its owner is the System Program,
    // not the ACL program.
    //
    // Note: we cannot actually write a matching HandlerPermissions
    // discriminator + handle into the data, because writing arbitrary bytes
    // requires a program-owned account. That doesn't weaken the test:
    // Anchor's `Account<HandlerPermissions>` wrapper checks the *owner*
    // before any contents, so owner-mismatch fires first. This pins down the
    // property we care about — an attacker cannot impersonate a
    // permission_list with an account they (or another program) own.
    const seed = "acl-fake-permission-list";
    const fakeAddr = await PublicKey.createWithSeed(
      provider.wallet.publicKey,
      seed,
      SystemProgram.programId
    );
    const space = 8 + 32 + 4 + 10 * 32; // matches HandlerPermissions::INIT_SPACE
    const lamports =
      await provider.connection.getMinimumBalanceForRentExemption(space);

    const setupTx = new Transaction().add(
      SystemProgram.createAccountWithSeed({
        fromPubkey: provider.wallet.publicKey,
        basePubkey: provider.wallet.publicKey,
        seed,
        newAccountPubkey: fakeAddr,
        lamports,
        space,
        programId: SystemProgram.programId,
      })
    );
    await provider.sendAndConfirm(setupTx);

    const info = await provider.connection.getAccountInfo(fakeAddr);
    assert.ok(info, "fake account should exist");
    assert.ok(
      info!.owner.equals(SystemProgram.programId),
      "fake account must be owned by something other than the ACL program"
    );

    try {
      await program.methods
        .isAllowed(handle)
        .accountsPartial({ permissionList: fakeAddr })
        .view();
      assert.fail("expected rejection due to wrong account owner");
    } catch (err: any) {
      const logs = (err?.simulationResponse?.logs ?? err?.logs ?? []).join("\n");
      expect(logs).to.match(
        /AccountOwnedByWrongProgram|ConstraintOwner|3007|3008/i
      );
    }
  });
});
