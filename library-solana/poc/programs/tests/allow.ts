import { Keypair } from "@solana/web3.js";
import { assert, expect } from "chai";
import {
  authority,
  ensureConfigInitialized,
  initPermissionList,
  makeHandle,
  program,
  provider,
} from "./utils";

describe("acl :: allow", () => {
  before(ensureConfigInitialized);

  it("appends a context key to allowed_accounts", async () => {
    const handle = makeHandle(1);
    const permissionList = await initPermissionList(handle);
    const contextKey = Keypair.generate().publicKey;

    await program.methods
      .allow(handle, contextKey)
      .accountsPartial({
        payer: provider.wallet.publicKey,
        authority: authority.publicKey,
        permissionList: permissionList.publicKey,
      })
      .signers([authority])
      .rpc();

    const acct = await program.account.handlerPermissions.fetch(
      permissionList.publicKey
    );
    assert.equal(acct.allowedAccounts.length, 1);
    assert.ok(acct.allowedAccounts[0].equals(contextKey));
    assert.deepEqual(Array.from(acct.handle), handle);
  });

  it("appends multiple context keys in insertion order", async () => {
    const handle = makeHandle(2);
    const permissionList = await initPermissionList(handle);
    const keys = [
      Keypair.generate().publicKey,
      Keypair.generate().publicKey,
      Keypair.generate().publicKey,
    ];

    for (const k of keys) {
      await program.methods
        .allow(handle, k)
        .accountsPartial({
          payer: provider.wallet.publicKey,
          authority: authority.publicKey,
          permissionList: permissionList.publicKey,
        })
        .signers([authority])
        .rpc();
    }

    const acct = await program.account.handlerPermissions.fetch(
      permissionList.publicKey
    );
    assert.equal(acct.allowedAccounts.length, keys.length);
    keys.forEach((k, i) => {
      assert.ok(
        acct.allowedAccounts[i].equals(k),
        `expected key at index ${i} to match`
      );
    });
  });

  it("reallocates across the CHUNK=10 boundary", async () => {
    const handle = makeHandle(3);
    const permissionList = await initPermissionList(handle);

    const sizeBefore = (
      await provider.connection.getAccountInfo(permissionList.publicKey)
    )!.data.length;

    for (let i = 0; i < 11; i++) {
      await program.methods
        .allow(handle, Keypair.generate().publicKey)
        .accountsPartial({
          payer: provider.wallet.publicKey,
          authority: authority.publicKey,
          permissionList: permissionList.publicKey,
        })
        .signers([authority])
        .rpc();
    }

    const sizeAfter = (
      await provider.connection.getAccountInfo(permissionList.publicKey)
    )!.data.length;

    const acct = await program.account.handlerPermissions.fetch(
      permissionList.publicKey
    );
    assert.equal(acct.allowedAccounts.length, 11);
    assert.isAbove(
      sizeAfter,
      sizeBefore,
      "permission_list account should have grown after exceeding the first chunk"
    );
  });

  it("reverts with HandleMismatch when the handle argument doesn't match the account", async () => {
    const handle = makeHandle(4);
    const permissionList = await initPermissionList(handle);
    const wrongHandle = handle.map((b) => b ^ 0xff);
    const contextKey = Keypair.generate().publicKey;

    try {
      await program.methods
        .allow(wrongHandle, contextKey)
        .accountsPartial({
          payer: provider.wallet.publicKey,
          authority: authority.publicKey,
          permissionList: permissionList.publicKey,
        })
        .signers([authority])
        .rpc();
      assert.fail("expected HandleMismatch");
    } catch (err: any) {
      const code = err?.error?.errorCode?.code ?? "";
      const msg = code + " " + (err?.message ?? "");
      expect(msg).to.match(/HandleMismatch/);
    }
  });

  it("reverts with UnauthorizedAccess when signer is not the fhe authority", async () => {
    const handle = makeHandle(5);
    const permissionList = await initPermissionList(handle);
    const impostor = Keypair.generate();
    const contextKey = Keypair.generate().publicKey;

    try {
      await program.methods
        .allow(handle, contextKey)
        .accountsPartial({
          payer: provider.wallet.publicKey,
          authority: impostor.publicKey,
          permissionList: permissionList.publicKey,
        })
        .signers([impostor])
        .rpc();
      assert.fail("expected UnauthorizedAccess");
    } catch (err: any) {
      const code = err?.error?.errorCode?.code ?? "";
      const msg = code + " " + (err?.message ?? "");
      expect(msg).to.match(/UnauthorizedAccess/);
    }
  });
});
