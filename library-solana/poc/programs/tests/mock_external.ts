import * as anchor from "@coral-xyz/anchor";
import { Keypair, PublicKey } from "@solana/web3.js";
import { assert, expect } from "chai";
import {
  configPda,
  ensureConfigInitialized,
  makeHandle,
  mockExternalProgram,
  program,
  provider,
  u128LeBytes,
} from "./utils";

// The mock seeds the permission_list PDA on (app, output_index), and the ACL
// program then owns the resulting account. Both programs must agree on the
// derivation, so we mirror the mock-external seeds here.
const derivePermissionList = (
  app: PublicKey,
  outputIndex: anchor.BN
): PublicKey =>
  PublicKey.findProgramAddressSync(
    [Buffer.from("permission_list"), app.toBuffer(), u128LeBytes(outputIndex)],
    program.programId
  )[0];

const callAllowExternalInput = (
  handle: number[],
  app: PublicKey,
  outputIndex: anchor.BN
) =>
  mockExternalProgram.methods
    .allowExternalInput(handle, app, outputIndex)
    .accountsPartial({
      payer: provider.wallet.publicKey,
      permissionList: derivePermissionList(app, outputIndex),
      aclConfig: configPda,
      aclProgram: program.programId,
    })
    .rpc();

describe("mock-external :: allow_external_input", () => {
  before(ensureConfigInitialized);

  it("inits the handle and allows it for the user (signer) and the specified app", async () => {
    const handle = makeHandle(7);
    const app = Keypair.generate().publicKey;
    const outputIndex = new anchor.BN(0);

    await callAllowExternalInput(handle, app, outputIndex);

    const acct = await program.account.handlerPermissions.fetch(
      derivePermissionList(app, outputIndex)
    );
    assert.deepEqual(Array.from(acct.handle), handle);
    assert.equal(acct.subjectCount, 2);
    assert.ok(
      acct.allowedAccounts[0].equals(provider.wallet.publicKey),
      "first entry should be the transaction signer (user)"
    );
    assert.ok(
      acct.allowedAccounts[1].equals(app),
      "second entry should be the specified app"
    );
  });

  it("reverts with HandleMismatch when the same (app, output_index) PDA is reused with a different handle", async () => {
    const handle = makeHandle(8);
    const app = Keypair.generate().publicKey;
    const outputIndex = new anchor.BN(0);

    await callAllowExternalInput(handle, app, outputIndex);

    try {
      const otherHandle = makeHandle(9);
      await callAllowExternalInput(otherHandle, app, outputIndex);
      assert.fail("expected HandleMismatch");
    } catch (err: any) {
      const msg = (err?.message ?? "") + JSON.stringify(err?.logs ?? []);
      expect(msg).to.match(/HandleMismatch/);
    }
  });
});
