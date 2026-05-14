import { Keypair, SystemProgram } from "@solana/web3.js";
import { assert, expect } from "chai";
import {
  aclProgramDataPda,
  configPda,
  program,
  provider,
} from "./utils";

describe("acl :: init", () => {
  it("creates the config PDA and stores both authorities", async () => {
    const fheAuthority = Keypair.generate().publicKey;
    const externalInputAuthority = Keypair.generate().publicKey;

    const existing = await provider.connection.getAccountInfo(configPda);
    if (existing !== null) {
      // Localnet state may persist across runs; skip if already initialized.
      // `init` is single-shot (PDA-seeded), so a fresh validator is required
      // to re-run this case.
      return;
    }

    await program.methods
      .init(fheAuthority, externalInputAuthority)
      .accounts({
        payer: provider.wallet.publicKey,
        configPda: configPda,
        acl: program.programId,
        programData: aclProgramDataPda,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    const config = await program.account.config.fetch(configPda);
    assert.ok(
      config.fheAuthority.equals(fheAuthority),
      "fhe_authority should match the value passed to init"
    );
    assert.ok(
      config.externalInputAuthority.equals(externalInputAuthority),
      "external_input_authority should match the value passed to init"
    );

    const info = await provider.connection.getAccountInfo(configPda);
    assert.isNotNull(info);
    assert.ok(info!.owner.equals(program.programId));
  });

  it("fails when re-initialized", async () => {
    const fheAuthority = Keypair.generate().publicKey;
    const externalInputAuthority = Keypair.generate().publicKey;

    try {
      await program.methods
        .init(fheAuthority, externalInputAuthority)
        .accountsPartial({ programData: aclProgramDataPda })
        .rpc();
      assert.fail("expected re-initialization to fail");
    } catch (err: any) {
      const msg = (err?.message ?? "") + JSON.stringify(err?.logs ?? []);
      expect(msg).to.match(/already in use|custom program error: 0x0/i);
    }
  });
});
