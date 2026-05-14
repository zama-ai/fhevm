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

// BPF Upgradeable Loader program ID — owns every upgradeable program's
// `ProgramData` account. The data account address is a PDA of the program
// id under this loader.
export const BPF_LOADER_UPGRADEABLE_PROGRAM_ID = new PublicKey(
  "BPFLoaderUpgradeab1e11111111111111111111111"
);

export const [aclProgramDataPda] = PublicKey.findProgramAddressSync(
  [program.programId.toBuffer()],
  BPF_LOADER_UPGRADEABLE_PROGRAM_ID
);

// Decode the upgrade authority from a `ProgramData` account's raw bytes.
// Layout: 4 bytes discriminator + 8 bytes slot + 1 byte Option tag
//          + (if Some) 32 bytes Pubkey.
const readUpgradeAuthority = (data: Buffer): PublicKey | null => {
  const optionTag = data[4 + 8];
  if (optionTag === 0) return null;
  return new PublicKey(data.subarray(13, 13 + 32));
};

export const ensureConfigInitialized = async () => {
  const existing = await provider.connection.getAccountInfo(configPda);
  if (existing === null) {
    // `init` requires the signer to be the ACL program's upgrade authority.
    // Surface a useful error instead of a raw ConstraintRaw if that's not
    // the wallet running the tests.
    const programDataInfo = await provider.connection.getAccountInfo(
      aclProgramDataPda
    );
    if (programDataInfo === null) {
      throw new Error(
        `acl program_data ${aclProgramDataPda.toBase58()} not found — ` +
          `has the ACL program been deployed?`
      );
    }
    const upgradeAuthority = readUpgradeAuthority(programDataInfo.data);
    if (
      upgradeAuthority === null ||
      !upgradeAuthority.equals(provider.wallet.publicKey)
    ) {
      throw new Error(
        `ACL upgrade authority is ${
          upgradeAuthority?.toBase58() ?? "<none — program is immutable>"
        } but the test wallet is ${provider.wallet.publicKey.toBase58()}. ` +
          `Make sure \`[test] upgradeable = true\` is set in Anchor.toml so ` +
          `\`anchor test\` deploys the program via the upgradeable loader.`
      );
    }
    await program.methods
      .init(authority.publicKey, externalInputAuthorityPda)
      .accountsPartial({
        programData: aclProgramDataPda,
      })
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

// Encode a u128 as a 16-byte little-endian Buffer to match
// `output_index.to_le_bytes()` used in the program's PDA seeds.
export const u128LeBytes = (n: anchor.BN): Buffer =>
  n.toArrayLike(Buffer, "le", 16);

export const derivePermissionList = (
  initialKey: PublicKey,
  outputIndex: anchor.BN
): PublicKey =>
  PublicKey.findProgramAddressSync(
    [
      Buffer.from("permission_list"),
      initialKey.toBuffer(),
      u128LeBytes(outputIndex),
    ],
    program.programId
  )[0];

export const initPermissionList = async (
  handle: number[],
  initialKey: PublicKey,
  outputIndex: anchor.BN
): Promise<PublicKey> => {
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
  return permissionList;
};

export const allowKey = async (
  handle: number[],
  permissionList: PublicKey,
  contextKey: PublicKey,
  initialKey: PublicKey,
  outputIndex: anchor.BN
) => {
  await program.methods
    .allow(handle, contextKey, initialKey, outputIndex)
    .accountsPartial({
      authority: authority.publicKey,
      permissionList,
    })
    .signers([authority])
    .rpc();
};

export const makeHandle = (seed: number): number[] =>
  Array.from({ length: 32 }, (_, i) => (seed + i) & 0xff);
