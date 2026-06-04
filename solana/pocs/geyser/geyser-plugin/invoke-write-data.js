// Minimal client that calls the geyser program's `write_data` instruction,
// which creates (or updates) a PDA derived from [b"data", authority].
// Run against a local validator to exercise the Geyser tracker plugin.
const fs = require("fs");
const os = require("os");
const path = require("path");
const {
  Connection,
  Keypair,
  PublicKey,
  Transaction,
  TransactionInstruction,
  sendAndConfirmTransaction,
} = require("@solana/web3.js");

const PROGRAM_ID = new PublicKey("H4Yc3MugAkJk2FEjLCfCr2J28hgMXzipJaSLq1Sa2SP8");
// write_data discriminator from target/idl/geyser.json
const DISCRIMINATOR = Buffer.from([211, 152, 195, 131, 83, 179, 248, 77]);
const SYSTEM_PROGRAM = new PublicKey("11111111111111111111111111111111");

function loadWallet() {
  const p = path.join(os.homedir(), ".config", "solana", "id.json");
  return Keypair.fromSecretKey(Uint8Array.from(JSON.parse(fs.readFileSync(p, "utf8"))));
}

function encodeArgs(value, message) {
  const msgBytes = Buffer.from(message, "utf8");
  const buf = Buffer.alloc(8 + 4 + msgBytes.length);
  buf.writeBigUInt64LE(BigInt(value), 0); // value: u64
  buf.writeUInt32LE(msgBytes.length, 8); // string length prefix (borsh)
  msgBytes.copy(buf, 12);
  return buf;
}

(async () => {
  const value = BigInt(process.argv[2] ?? 42);
  const message = process.argv[3] ?? "hello-from-geyser";

  const connection = new Connection("http://127.0.0.1:8899", "confirmed");
  const wallet = loadWallet();

  const [pda, bump] = PublicKey.findProgramAddressSync(
    [Buffer.from("data"), wallet.publicKey.toBuffer()],
    PROGRAM_ID
  );

  console.log("authority:", wallet.publicKey.toBase58());
  console.log("data PDA: ", pda.toBase58(), "(bump", bump + ")");

  const ix = new TransactionInstruction({
    programId: PROGRAM_ID,
    keys: [
      { pubkey: wallet.publicKey, isSigner: true, isWritable: true },
      { pubkey: pda, isSigner: false, isWritable: true },
      { pubkey: SYSTEM_PROGRAM, isSigner: false, isWritable: false },
    ],
    data: Buffer.concat([DISCRIMINATOR, encodeArgs(value, message)]),
  });

  const sig = await sendAndConfirmTransaction(connection, new Transaction().add(ix), [wallet]);
  console.log("confirmed:", sig);
})().catch((e) => {
  console.error(e);
  process.exit(1);
});
