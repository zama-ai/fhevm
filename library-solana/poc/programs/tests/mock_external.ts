import { Keypair, PublicKey } from "@solana/web3.js";
import { assert, expect } from "chai";
import {
  configPda,
  ensureConfigInitialized,
  makeHandle,
  mockExternalProgram,
  program,
  provider,
} from "./utils";

const derivePermissionList = (handle: number[]): PublicKey =>
  PublicKey.findProgramAddressSync(
    [Buffer.from("permission_list"), Buffer.from(handle)],
    mockExternalProgram.programId
  )[0];

const callAllowExternalInput = (handle: number[], app: PublicKey) =>
  mockExternalProgram.methods
    .allowExternalInput(handle, app)
    .accountsPartial({
      payer: provider.wallet.publicKey,
      permissionList: derivePermissionList(handle),
      aclConfig: configPda,
      aclProgram: program.programId,
    })
    .rpc();

describe("mock-external :: allow_external_input", () => {
  before(ensureConfigInitialized);

  it("inits the handle and allows it for the user (signer) and the specified app", async () => {
    const handle = makeHandle(7);
    const app = Keypair.generate().publicKey;

    await callAllowExternalInput(handle, app);

    const acct = await program.account.handlerPermissions.fetch(
      derivePermissionList(handle)
    );
    assert.deepEqual(Array.from(acct.handle), handle);
    assert.equal(acct.allowedAccounts.length, 2);
    assert.ok(
      acct.allowedAccounts[0].equals(provider.wallet.publicKey),
      "first entry should be the transaction signer (user)"
    );
    assert.ok(
      acct.allowedAccounts[1].equals(app),
      "second entry should be the specified app"
    );
  });

  it("reverts if the handle has already been initialized", async () => {
    const handle = makeHandle(8);
    const app = Keypair.generate().publicKey;

    await callAllowExternalInput(handle, app);

    try {
      await callAllowExternalInput(handle, Keypair.generate().publicKey);
      assert.fail("expected the second init_handle CPI to fail");
    } catch (err: any) {
      const msg = (err?.message ?? "") + JSON.stringify(err?.logs ?? []);
      expect(msg).to.match(/already in use|custom program error: 0x0/i);
    }
  });
});
