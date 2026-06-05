// Minimal raw client (no Anchor-TS) to drive zama-host instructions against the
// local validator. v0: initialize_host_config, to prove the deployed program
// executes real on-chain logic. Builds the Anchor discriminator + borsh args by hand.
import {
  Connection, Keypair, PublicKey, SystemProgram,
  Transaction, TransactionInstruction, sendAndConfirmTransaction,
} from "@solana/web3.js";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { homedir } from "node:os";

const ZAMA_HOST = new PublicKey("6rQaBev7B67LrQW7nJPBhJYt7rHSK38DWuz1LdiQRFcf");
const conn = new Connection("http://127.0.0.1:8899", "confirmed");
const wallet = Keypair.fromSecretKey(
  Uint8Array.from(JSON.parse(readFileSync(`${homedir()}/.config/solana/id.json`, "utf8"))),
);

// Anchor global instruction discriminator = sha256("global:<name>")[0..8].
const disc = (name) => createHash("sha256").update(`global:${name}`).digest().subarray(0, 8);

const [hostConfig] = PublicKey.findProgramAddressSync([Buffer.from("host-config")], ZAMA_HOST);

// borsh(InitializeHostConfigArgs): u64 chain_id | Pubkey input_verifier_authority |
// u64 gateway_chain_id | [u8;20] input_verification_contract | [u8;20] coprocessor_signer |
// 2x Pubkey (material, test) | 3x bool. Gateway verifier fields zeroed here (the secp
// input-bind path is unused by this bootstrap; the Rust live-client is canonical).
const chainId = Buffer.alloc(8);
chainId.writeBigUInt64LE(12345n); // SOLANA_POC_CHAIN_ID
const args = Buffer.concat([
  chainId,
  wallet.publicKey.toBuffer(), // input_verifier_authority
  Buffer.alloc(8), // gateway_chain_id
  Buffer.alloc(20), // input_verification_contract
  Buffer.alloc(20), // coprocessor_signer
  wallet.publicKey.toBuffer(), // material_authority
  wallet.publicKey.toBuffer(), // test_authority
  Buffer.from([1]), // mock_input_enabled
  Buffer.from([1]), // test_shims_enabled
  Buffer.from([0]), // grant_deny_list_enabled
]);
const data = Buffer.concat([disc("initialize_host_config"), args]);

const ix = new TransactionInstruction({
  programId: ZAMA_HOST,
  keys: [
    { pubkey: wallet.publicKey, isSigner: true, isWritable: true }, // payer
    { pubkey: wallet.publicKey, isSigner: true, isWritable: false }, // admin
    { pubkey: hostConfig, isSigner: false, isWritable: true }, // host_config PDA (init)
    { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
  ],
  data,
});

const sig = await sendAndConfirmTransaction(conn, new Transaction().add(ix), [wallet]);
console.log("OK initialize_host_config:", sig);
const acc = await conn.getAccountInfo(hostConfig);
console.log("host_config:", hostConfig.toBase58(), "owner:", acc.owner.toBase58(), "bytes:", acc.data.length);
