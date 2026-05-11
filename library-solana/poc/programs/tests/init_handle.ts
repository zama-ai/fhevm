import { Keypair } from "@solana/web3.js";
import { assert, expect } from "chai";
import {
  authority,
  ensureConfigInitialized,
  makeHandle,
  program,
  provider,
} from "./utils";

describe("acl :: init_handle", () => {
  before(ensureConfigInitialized);

  it("creates a permission_list owned by the program with the given handle and an empty allow list", async () => {
    const handle = makeHandle(10);
    const permissionList = Keypair.generate();

    await program.methods
      .initHandle(handle)
      .accountsPartial({
        payer: provider.wallet.publicKey,
        authority: authority.publicKey,
        permissionList: permissionList.publicKey,
      })
      .signers([authority, permissionList])
      .rpc();

    const acct = await program.account.handlerPermissions.fetch(
      permissionList.publicKey
    );
    assert.deepEqual(Array.from(acct.handle), handle);
    assert.equal(acct.allowedAccounts.length, 0);

    const info = await provider.connection.getAccountInfo(
      permissionList.publicKey
    );
    assert.isNotNull(info);
    assert.ok(info!.owner.equals(program.programId));
    // INIT_SPACE = HANDLE_SIZE + VEC_PREFIX + 1*CHUNK*PUBKEY_SIZE = 32+4+320 = 356
    // + 8-byte discriminator = 364
    assert.equal(info!.data.length, 364);
  });

  it("reverts with UnauthorizedAccess when the authority signer is not the fhe authority", async () => {
    const handle = makeHandle(12);
    const impostor = Keypair.generate();
    const permissionList = Keypair.generate();

    try {
      await program.methods
        .initHandle(handle)
        .accountsPartial({
          payer: provider.wallet.publicKey,
          authority: impostor.publicKey,
          permissionList: permissionList.publicKey,
        })
        .signers([impostor, permissionList])
        .rpc();
      assert.fail("expected UnauthorizedAccess");
    } catch (err: any) {
      const code = err?.error?.errorCode?.code ?? "";
      const msg = code + " " + (err?.message ?? "");
      expect(msg).to.match(/UnauthorizedAccess/);
    }
  });
});