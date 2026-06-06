// Real zama-host bootstrap against the live local validator. Initializes the
// singleton HostConfig with the REAL gateway-derived values (read from the live
// GatewayConfig on chain) and defines the active KMS context from the REAL
// ProtocolConfig KMS signer set — no zeroed fields, mock-input OFF, test-shims
// OFF (the live secp256k1 paths are authoritative). Builds Anchor discriminators
// + borsh args by hand (no Anchor-TS dep).
//
// Real values are passed via env so this script stays source-of-truth-free:
//   GATEWAY_CHAIN_ID, INPUT_VERIFICATION_ADDRESS, COPROCESSOR_SIGNER,
//   DECRYPTION_ADDRESS, KMS_SIGNERS (comma-separated 0x EVM addresses),
//   SOLANA_HOST_CHAIN_ID (decimal u64 with the chain-type high bit set).
import {
  Connection, Keypair, PublicKey, SystemProgram,
  Transaction, TransactionInstruction, sendAndConfirmTransaction,
} from "@solana/web3.js";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { homedir } from "node:os";

const ZAMA_HOST = new PublicKey("BXsiKq6Jg4vgdBqSd75NbMbKaB7WFKK48NVXx4zoeLsW");
const conn = new Connection("http://127.0.0.1:8899", "confirmed");
const wallet = Keypair.fromSecretKey(
  Uint8Array.from(JSON.parse(readFileSync(`${homedir()}/.config/solana/id.json`, "utf8"))),
);

const env = (k, d) => {
  const v = process.env[k] ?? d;
  if (v === undefined) throw new Error(`missing required env ${k}`);
  return v;
};
// EVM 0x-hex address -> 20-byte Buffer.
const addr20 = (hex) => {
  const b = Buffer.from(hex.replace(/^0x/, ""), "hex");
  if (b.length !== 20) throw new Error(`expected 20-byte address, got ${b.length} from ${hex}`);
  return b;
};
const u64le = (n) => { const b = Buffer.alloc(8); b.writeBigUInt64LE(BigInt(n)); return b; };
// Anchor global instruction discriminator = sha256("global:<name>")[0..8].
const disc = (name) => createHash("sha256").update(`global:${name}`).digest().subarray(0, 8);

const GATEWAY_CHAIN_ID = env("GATEWAY_CHAIN_ID");
const INPUT_VERIFICATION = addr20(env("INPUT_VERIFICATION_ADDRESS"));
const COPROCESSOR_SIGNER = addr20(env("COPROCESSOR_SIGNER"));
const DECRYPTION = addr20(env("DECRYPTION_ADDRESS"));
const KMS_SIGNERS = env("KMS_SIGNERS").split(",").map((s) => addr20(s.trim()));
const SOLANA_HOST_CHAIN_ID = env("SOLANA_HOST_CHAIN_ID");

const [hostConfig] = PublicKey.findProgramAddressSync([Buffer.from("host-config")], ZAMA_HOST);

// --- initialize_host_config -------------------------------------------------
// borsh(InitializeHostConfigArgs): u64 chain_id | Pubkey input_verifier_authority |
// u64 gateway_chain_id | [u8;20] input_verification_contract | [u8;20] coprocessor_signer |
// [u8;20] decryption_contract | Pubkey material_authority | Pubkey test_authority |
// bool mock_input_enabled | bool test_shims_enabled | bool grant_deny_list_enabled.
const initArgs = Buffer.concat([
  u64le(SOLANA_HOST_CHAIN_ID),  // chain_id embedded into Solana handles (high-bit set)
  wallet.publicKey.toBuffer(),  // input_verifier_authority (inert: mock/signed paths OFF)
  u64le(GATEWAY_CHAIN_ID),      // gateway_chain_id (EIP-712 domain)
  INPUT_VERIFICATION,           // input_verification_contract (coprocessor cert domain)
  COPROCESSOR_SIGNER,           // coprocessor_signer (input attestation secp signer)
  DECRYPTION,                   // decryption_contract (KMS cert domain)
  wallet.publicKey.toBuffer(),  // material_authority
  wallet.publicKey.toBuffer(),  // test_authority (inert: test shims OFF)
  Buffer.from([0]),             // mock_input_enabled = false (real ZK-proof bind only)
  Buffer.from([0]),             // test_shims_enabled = false
  Buffer.from([0]),             // grant_deny_list_enabled = false
]);
const initIx = new TransactionInstruction({
  programId: ZAMA_HOST,
  keys: [
    { pubkey: wallet.publicKey, isSigner: true, isWritable: true },  // payer
    { pubkey: wallet.publicKey, isSigner: true, isWritable: false }, // admin
    { pubkey: hostConfig, isSigner: false, isWritable: true },       // host_config PDA (init)
    { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
  ],
  data: Buffer.concat([disc("initialize_host_config"), initArgs]),
});

// --- define_kms_context (context id 1) --------------------------------------
// borsh: u64 context_id | vec<[u8;20]> signers (u32 len + items) | KmsThresholds (3x u8).
const CONTEXT_ID = 1n;
const [kmsContext] = PublicKey.findProgramAddressSync(
  [Buffer.from("kms-context"), u64le(CONTEXT_ID)], ZAMA_HOST,
);
const signersLen = Buffer.alloc(4); signersLen.writeUInt32LE(KMS_SIGNERS.length);
const kmsArgs = Buffer.concat([
  u64le(CONTEXT_ID),
  signersLen, ...KMS_SIGNERS,
  Buffer.from([1, 1, 1, 1]), // thresholds: public_decryption, user_decryption, kms_gen, mpc
]);
const kmsIx = new TransactionInstruction({
  programId: ZAMA_HOST,
  keys: [
    { pubkey: wallet.publicKey, isSigner: true, isWritable: true }, // admin + payer
    { pubkey: hostConfig, isSigner: false, isWritable: true },      // host_config PDA
    { pubkey: kmsContext, isSigner: false, isWritable: true },      // kms_context PDA (init)
    { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
  ],
  data: Buffer.concat([disc("define_kms_context"), kmsArgs]),
});

// initialize_host_config is a one-time init; skip if the singleton already exists
// so the bootstrap is idempotent/re-runnable.
if (await conn.getAccountInfo(hostConfig)) {
  console.log("host_config already initialized — skipping initialize_host_config");
} else {
  const sig1 = await sendAndConfirmTransaction(conn, new Transaction().add(initIx), [wallet]);
  console.log("OK initialize_host_config:", sig1);
}
const sig2 = await sendAndConfirmTransaction(conn, new Transaction().add(kmsIx), [wallet]);
console.log("OK define_kms_context:", sig2);

const hc = await conn.getAccountInfo(hostConfig);
const kc = await conn.getAccountInfo(kmsContext);
console.log("host_config:", hostConfig.toBase58(), "bytes:", hc.data.length, "owner:", hc.owner.toBase58());
console.log("kms_context:", kmsContext.toBase58(), "bytes:", kc.data.length, "signers:", KMS_SIGNERS.length);
