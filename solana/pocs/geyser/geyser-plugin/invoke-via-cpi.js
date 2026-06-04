// Calls the `caller` program's `proxy_write`, which CPIs into the tracked
// `geyser` program's `write_data`. The Geyser tracker plugin is configured to
// watch only `geyser`, so this run should surface a `CPI` event (geyser as an
// inner instruction) — while `caller` itself stays untracked.
const fs = require("fs");
const os = require("os");
const path = require("path");
const {
  Connection,
  Keypair,
  PublicKey,
  Transaction,
  TransactionInstruction,
  SystemProgram,
  sendAndConfirmTransaction,
} = require("@solana/web3.js");

const GEYSER_PROGRAM = new PublicKey("H4Yc3MugAkJk2FEjLCfCr2J28hgMXzipJaSLq1Sa2SP8");
const CALLER_PROGRAM = new PublicKey("4RsnoEwKPWbZg4Z6NUGaqP355SvtGWjjUFqEdmEGiFAB");
// proxy_write discriminator from target/idl/caller.json
const PROXY_WRITE_DISC = Buffer.from([55, 164, 171, 206, 120, 41, 75, 3]);

function loadWallet() {
  const p = path.join(os.homedir(), ".config", "solana", "id.json");
  return Keypair.fromSecretKey(Uint8Array.from(JSON.parse(fs.readFileSync(p, "utf8"))));
}

function encodeArgs(value, message) {
  const msgBytes = Buffer.from(message, "utf8");
  const buf = Buffer.alloc(8 + 4 + msgBytes.length);
  buf.writeBigUInt64LE(BigInt(value), 0); // value: u64
  buf.writeUInt32LE(msgBytes.length, 8); // borsh string length prefix
  msgBytes.copy(buf, 12);
  return buf;
}

(async () => {
  const value = BigInt(process.argv[2] ?? 7777);
  const message = process.argv[3] ?? "via-cpi";

  const connection = new Connection("http://127.0.0.1:8899", "confirmed");
  const wallet = loadWallet();

  // The geyser PDA is derived under the GEYSER program, even though we call caller.
  const [pda, bump] = PublicKey.findProgramAddressSync(
    [Buffer.from("data"), wallet.publicKey.toBuffer()],
    GEYSER_PROGRAM
  );

  console.log("caller:    ", CALLER_PROGRAM.toBase58());
  console.log("authority: ", wallet.publicKey.toBase58());
  console.log("data PDA:  ", pda.toBase58(), "(bump", bump + ")");

  const ix = new TransactionInstruction({
    programId: CALLER_PROGRAM,
    keys: [
      { pubkey: wallet.publicKey, isSigner: true, isWritable: true }, // authority
      { pubkey: pda, isSigner: false, isWritable: true }, // data_account
      { pubkey: GEYSER_PROGRAM, isSigner: false, isWritable: false }, // geyser_program
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false }, // system_program
    ],
    data: Buffer.concat([PROXY_WRITE_DISC, encodeArgs(value, message)]),
  });

  const sig = await sendAndConfirmTransaction(connection, new Transaction().add(ix), [wallet]);
  console.log("confirmed:", sig);
})().catch((e) => {
  console.error(e);
  process.exit(1);
});
