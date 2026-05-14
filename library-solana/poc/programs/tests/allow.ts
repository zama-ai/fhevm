import * as anchor from "@coral-xyz/anchor";
import { Keypair, PublicKey } from "@solana/web3.js";
import { assert, expect } from "chai";
import {
  authority,
  ensureConfigInitialized,
  initPermissionList,
  makeHandle,
  program,
} from "./utils";

const MAX_SUBJECTS = 16;

describe("acl :: allow", () => {
  before(ensureConfigInitialized);

  it("appends a context key to allowed_accounts", async () => {
    const handle = makeHandle(1);
    const initialKey = Keypair.generate().publicKey;
    const outputIndex = new anchor.BN(0);
    const permissionList = await initPermissionList(
      handle,
      initialKey,
      outputIndex
    );
    const contextKey = Keypair.generate().publicKey;

    await program.methods
      .allow(handle, contextKey, initialKey, outputIndex)
      .accountsPartial({
        authority: authority.publicKey,
        permissionList,
      })
      .signers([authority])
      .rpc();

    const acct = await program.account.handlerPermissions.fetch(permissionList);
    assert.equal(acct.subjectCount, 1);
    assert.ok(acct.allowedAccounts[0].equals(contextKey));
    assert.deepEqual(Array.from(acct.handle), handle);
  });

  it("appends multiple context keys in insertion order", async () => {
    const handle = makeHandle(2);
    const initialKey = Keypair.generate().publicKey;
    const outputIndex = new anchor.BN(0);
    const permissionList = await initPermissionList(
      handle,
      initialKey,
      outputIndex
    );
    const keys = [
      Keypair.generate().publicKey,
      Keypair.generate().publicKey,
      Keypair.generate().publicKey,
    ];

    for (const k of keys) {
      await program.methods
        .allow(handle, k, initialKey, outputIndex)
        .accountsPartial({
          authority: authority.publicKey,
          permissionList,
        })
        .signers([authority])
        .rpc();
    }

    const acct = await program.account.handlerPermissions.fetch(permissionList);
    assert.equal(acct.subjectCount, keys.length);
    keys.forEach((k, i) => {
      assert.ok(
        acct.allowedAccounts[i].equals(k),
        `expected key at index ${i} to match`
      );
    });
  });

  it("is idempotent for a key already in allowed_accounts", async () => {
    const handle = makeHandle(3);
    const initialKey = Keypair.generate().publicKey;
    const outputIndex = new anchor.BN(0);
    const permissionList = await initPermissionList(
      handle,
      initialKey,
      outputIndex
    );
    const contextKey = Keypair.generate().publicKey;

    for (let i = 0; i < 3; i++) {
      await program.methods
        .allow(handle, contextKey, initialKey, outputIndex)
        .accountsPartial({
          authority: authority.publicKey,
          permissionList,
        })
        .signers([authority])
        .rpc();
    }

    const acct = await program.account.handlerPermissions.fetch(permissionList);
    assert.equal(acct.subjectCount, 1);
  });

  it("reverts with HandleOverflow once MAX_SUBJECTS is reached", async () => {
    const handle = makeHandle(4);
    const initialKey = Keypair.generate().publicKey;
    const outputIndex = new anchor.BN(0);
    const permissionList = await initPermissionList(
      handle,
      initialKey,
      outputIndex
    );

    for (let i = 0; i < MAX_SUBJECTS; i++) {
      await program.methods
        .allow(
          handle,
          Keypair.generate().publicKey,
          initialKey,
          outputIndex
        )
        .accountsPartial({
          authority: authority.publicKey,
          permissionList,
        })
        .signers([authority])
        .rpc();
    }

    try {
      await program.methods
        .allow(
          handle,
          Keypair.generate().publicKey,
          initialKey,
          outputIndex
        )
        .accountsPartial({
          authority: authority.publicKey,
          permissionList,
        })
        .signers([authority])
        .rpc();
      assert.fail("expected HandleOverflow");
    } catch (err: any) {
      const code = err?.error?.errorCode?.code ?? "";
      const msg = code + " " + (err?.message ?? "");
      expect(msg).to.match(/HandleOverflow/);
    }
  });

  it("reverts with HandleMismatch when the handle argument doesn't match the account", async () => {
    const handle = makeHandle(5);
    const initialKey = Keypair.generate().publicKey;
    const outputIndex = new anchor.BN(0);
    const permissionList = await initPermissionList(
      handle,
      initialKey,
      outputIndex
    );
    const wrongHandle = handle.map((b) => b ^ 0xff);
    const contextKey = Keypair.generate().publicKey;

    try {
      await program.methods
        .allow(wrongHandle, contextKey, initialKey, outputIndex)
        .accountsPartial({
          authority: authority.publicKey,
          permissionList,
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
    const handle = makeHandle(6);
    const initialKey = Keypair.generate().publicKey;
    const outputIndex = new anchor.BN(0);
    const permissionList = await initPermissionList(
      handle,
      initialKey,
      outputIndex
    );
    const impostor = Keypair.generate();
    const contextKey = Keypair.generate().publicKey;

    try {
      await program.methods
        .allow(handle, contextKey, initialKey, outputIndex)
        .accountsPartial({
          authority: impostor.publicKey,
          permissionList,
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

  it("reverts with DefaultKeyAllow when context_key is the default pubkey", async () => {
    const handle = makeHandle(7);
    const initialKey = Keypair.generate().publicKey;
    const outputIndex = new anchor.BN(0);
    const permissionList = await initPermissionList(
      handle,
      initialKey,
      outputIndex
    );

    try {
      await program.methods
        .allow(handle, PublicKey.default, initialKey, outputIndex)
        .accountsPartial({
          authority: authority.publicKey,
          permissionList,
        })
        .signers([authority])
        .rpc();
      assert.fail("expected DefaultKeyAllow");
    } catch (err: any) {
      const code = err?.error?.errorCode?.code ?? "";
      const msg = code + " " + (err?.message ?? "");
      expect(msg).to.match(/DefaultKeyAllow/);
    }
  });
});
