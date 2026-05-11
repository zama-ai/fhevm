import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, Keypair } from "@solana/web3.js";
import { Acl } from "../target/types/acl";
import { MockExternal } from "../target/types/mock_external";

anchor.setProvider(anchor.AnchorProvider.env());

export const program = anchor.workspace.acl as Program<Acl>;
export const mockExternalProgram = anchor.workspace
  .mockExternal as Program<MockExternal>;
export const provider = anchor.getProvider() as anchor.AnchorProvider;

// FHE authority keypair shared across every test file in the same
// `anchor test` run. Module caching guarantees a single instance — the first
// suite to run initializes `Config` with this pubkey, and every other suite
// reuses it.
export const authority = Keypair.generate();

// The external-input authority registered in `Config` is the PDA owned by
// `mock-external`, not a keypair. Only that program can sign for it via CPI.
export const [externalInputAuthorityPda] = PublicKey.findProgramAddressSync(
  [Buffer.from("external_input_authority")],
  mockExternalProgram.programId
);

export const [configPda] = PublicKey.findProgramAddressSync(
  [Buffer.from("acl_config")],
  program.programId
);

export const ensureConfigInitialized = async () => {
  const existing = await provider.connection.getAccountInfo(configPda);
  if (existing === null) {
    await program.methods
      .init(authority.publicKey, externalInputAuthorityPda)
      .rpc();
    return;
  }
  const config = await program.account.config.fetch(configPda);
  if (
    !config.fheAuthority.equals(authority.publicKey) ||
    !config.externalInputAuthority.equals(externalInputAuthorityPda)
  ) {
    throw new Error(
      "config_pda is initialized with unknown authorities — run " +
        "`anchor test` against a fresh validator so this suite can control " +
        "them."
    );
  }
};

export const initPermissionList = async (handle: number[]) => {
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
  return permissionList;
};

export const allowKey = async (
  handle: number[],
  permissionList: PublicKey,
  contextKey: PublicKey
) => {
  await program.methods
    .allow(handle, contextKey)
    .accountsPartial({
      payer: provider.wallet.publicKey,
      authority: authority.publicKey,
      permissionList,
    })
    .signers([authority])
    .rpc();
};

export const makeHandle = (seed: number): number[] =>
  Array.from({ length: 32 }, (_, i) => (seed + i) & 0xff);
