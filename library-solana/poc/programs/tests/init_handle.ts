import * as anchor from "@coral-xyz/anchor";
import { Keypair } from "@solana/web3.js";
import { assert, expect } from "chai";
import {
  authority,
  derivePermissionList,
  ensureConfigInitialized,
  makeHandle,
  program,
  provider,
} from "./utils";

describe("acl :: init_handle", () => {
  before(ensureConfigInitialized);

  it("creates a permission_list PDA owned by the program with the given handle and an empty subject list", async () => {
    const handle = makeHandle(10);
    const initialKey = Keypair.generate().publicKey;
    const outputIndex = new anchor.BN(0);
    const permissionList = derivePermissionList(initialKey, outputIndex);

    await program.methods
      .initHandle(handle, initialKey, outputIndex)
      .accountsPartial({
        payer: provider.wallet.publicKey,
        authority: authority.publicKey,
        permissionList,
      })
      .signers([authority])
      .rpc();

    const acct = await program.account.handlerPermissions.fetch(permissionList);
    assert.deepEqual(Array.from(acct.handle), handle);
    assert.equal(acct.subjectCount, 0);

    const info = await provider.connection.getAccountInfo(permissionList);
    assert.isNotNull(info);
    assert.ok(info!.owner.equals(program.programId));
    // INIT_SPACE = handle(32) + state(1) + allowed_accounts(16*32=512)
    //              + subject_count(1) + bump(1) = 547
    // + 8-byte discriminator = 555
    assert.equal(info!.data.length, 555);
  });

  it("is idempotent when called twice with the same (initial_key, output_index, handle)", async () => {
    const handle = makeHandle(11);
    const initialKey = Keypair.generate().publicKey;
    const outputIndex = new anchor.BN(1);
    const permissionList = derivePermissionList(initialKey, outputIndex);

    for (let i = 0; i < 2; i++) {
      await program.methods
        .initHandle(handle, initialKey, outputIndex)
        .accountsPartial({
          payer: provider.wallet.publicKey,
          authority: authority.publicKey,
          permissionList,
        })
        .signers([authority])
        .rpc();
    }

    const acct = await program.account.handlerPermissions.fetch(permissionList);
    assert.deepEqual(Array.from(acct.handle), handle);
  });

  it("reverts with HandleMismatch when an already-bound PDA is re-bound to a different handle", async () => {
    const handle = makeHandle(12);
    const initialKey = Keypair.generate().publicKey;
    const outputIndex = new anchor.BN(2);
    const permissionList = derivePermissionList(initialKey, outputIndex);

    await program.methods
      .initHandle(handle, initialKey, outputIndex)
      .accountsPartial({
        payer: provider.wallet.publicKey,
        authority: authority.publicKey,
        permissionList,
      })
      .signers([authority])
      .rpc();

    try {
      const wrongHandle = handle.map((b) => b ^ 0xff);
      await program.methods
        .initHandle(wrongHandle, initialKey, outputIndex)
        .accountsPartial({
          payer: provider.wallet.publicKey,
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

  it("reverts with UnauthorizedAccess when the authority signer is not the fhe authority", async () => {
    const handle = makeHandle(13);
    const initialKey = Keypair.generate().publicKey;
    const outputIndex = new anchor.BN(3);
    const permissionList = derivePermissionList(initialKey, outputIndex);
    const impostor = Keypair.generate();

    try {
      await program.methods
        .initHandle(handle, initialKey, outputIndex)
        .accountsPartial({
          payer: provider.wallet.publicKey,
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
});
