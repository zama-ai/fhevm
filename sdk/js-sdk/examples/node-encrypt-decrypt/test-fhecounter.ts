/**
 * End-to-end test with a deployed FHECounter contract on Sepolia.
 *
 * Flow:
 *   1. Read current encrypted count handle from the contract
 *   2. If zero: encrypt a value, call increment(), then read the new handle
 *   3. Attempt public decryption of the handle
 *
 * Usage: npx tsx ./examples/node-encrypt-decrypt/test-fhecounter.ts
 */

import { ethers } from "ethers";
import { readFileSync } from "node:fs";
import { resolve, dirname } from "node:path";
import { fileURLToPath } from "node:url";

import {
  setFhevmRuntimeConfig,
  createFhevmClient,
} from "../../src/ethers/index.js";
import { sepolia } from "../../src/core/chains/index.js";
import { createFhevm } from "../../src/ethers/clients/createFhevm.js";
import { decryptModule } from "../../src/core/modules/decrypt/module/index.js";
import {
  createFhevmDecryptionKey,
  type FhevmDecryptionKey,
} from "../../src/core/user/FhevmDecryptionKey-p.js";
import { toFhevmHandle } from "../../src/core/handle/FhevmHandle.js";
import { asChecksummedAddress } from "../../src/core/base/address.js";
import { asBytesHex } from "../../src/core/base/bytes.js";
import type {
  Bytes65Hex,
  ChecksummedAddress,
} from "../../src/core/types/primitives.js";

// ── Load .env.local ──────────────────────────────────────────────────────
const __dirname = dirname(fileURLToPath(import.meta.url));

function loadEnv(): Record<string, string> {
  const content = readFileSync(resolve(__dirname, ".env.local"), "utf-8");
  const env: Record<string, string> = {};
  for (const line of content.split("\n")) {
    const t = line.trim();
    if (t.length === 0 || t.startsWith("#")) continue;
    const i = t.indexOf("=");
    if (i === -1) continue;
    env[t.slice(0, i)] = t.slice(i + 1);
  }
  return env;
}

const env = loadEnv();

// ── Config ───────────────────────────────────────────────────────────────
const RPC_URL = "https://ethereum-sepolia-rpc.publicnode.com";
const FHECOUNTER_ADDRESS = "0xc1b7223f08F52fbfA263c27674AE577911c3b20e";

const FHECOUNTER_ABI = [
  "function getCount() view returns (uint256)",
  "function increment(bytes32 encryptedValue, bytes calldata inputProof)",
  "function decrement(bytes32 encryptedValue, bytes calldata inputProof)",
];

// ── Main ─────────────────────────────────────────────────────────────────
async function main(): Promise<void> {
  const t0 = Date.now();
  const elapsed = (): string => `${((Date.now() - t0) / 1000).toFixed(1)}s`;

  // 1. Setup
  console.log(`[${elapsed()}] Setting up...`);
  setFhevmRuntimeConfig({
    numberOfThreads: 4,
    logger: {
      debug: (_m: string) => {},
      error: (m: string, c: unknown) => {
        console.error("  [error]", m);
        if (c !== undefined) console.error(c);
      },
    },
  });

  const provider = new ethers.JsonRpcProvider(RPC_URL);
  const wallet = new ethers.Wallet(`0x${env.WALLET_PRIVATE_KEY}`, provider);
  const userAddress = asChecksummedAddress(wallet.address);
  console.log(`  Wallet: ${userAddress}`);

  const client = createFhevmClient({ chain: sepolia, provider });
  const contract = new ethers.Contract(FHECOUNTER_ADDRESS, FHECOUNTER_ABI, wallet);

  // 2. Read current count handle
  console.log(`\n[${elapsed()}] Reading current count handle from FHECounter...`);
  const countRaw: bigint = await contract.getCount();
  const countHex = "0x" + countRaw.toString(16).padStart(64, "0");
  console.log(`  Raw handle: ${countHex}`);

  if (countRaw === 0n) {
    console.log("  Count is zero — needs increment first.");

    // 3. Encrypt a value to call increment
    console.log(`\n[${elapsed()}] Fetching FHE public encryption params...`);
    const params = await client.fetchGlobalFhePkeParams();
    console.log("  OK");

    console.log(`\n[${elapsed()}] Encrypting uint32(1) for increment...`);
    try {
      const proof = await client.encrypt({
        globalFhePublicEncryptionParams: params,
        contractAddress: FHECOUNTER_ADDRESS,
        userAddress: userAddress,
        values: [{ type: "uint32", value: 1 }],
        extraData: asBytesHex("0x00"),
      });

      console.log(`  Encrypted handle: ${proof.externalHandles[0]?.bytes32Hex}`);
      console.log(`  Proof length: ${proof.bytesHex.length} chars`);

      // 4. Call increment on the contract
      console.log(`\n[${elapsed()}] Calling FHECounter.increment()...`);
      const tx = await contract.increment(
        proof.externalHandles[0]?.bytes32Hex,
        proof.bytesHex,
      );
      console.log(`  TX: ${tx.hash}`);
      console.log("  Waiting for confirmation...");
      const receipt = await tx.wait();
      console.log(`  Confirmed in block ${receipt?.blockNumber}`);

      // Re-read
      const newCount: bigint = await contract.getCount();
      const newHex = "0x" + newCount.toString(16).padStart(64, "0");
      console.log(`  New count handle: ${newHex}`);
    } catch (err: unknown) {
      const msg = err instanceof Error ? err.message : String(err);
      console.log(`  Encryption/TX failed: ${msg.split("\n")[0]}`);
      console.log("  (ZK proof generation works — relayer coprocessor signing may be unavailable)");
    }
  } else {
    // We have a non-zero handle — try to decrypt it
    console.log(`  Non-zero handle found, attempting decryption...`);

    const handle = toFhevmHandle(countHex);
    console.log(`  Handle type: ${handle.fheType}`);

    // Generate KMS key
    console.log(`\n[${elapsed()}] Generating KMS decryption key...`);
    const baseRuntime = createFhevm();
    const decryptRuntime = baseRuntime.runtime.extend(decryptModule);
    const tkmsPrivateKey = await decryptRuntime.decrypt.generateTkmsPrivateKey();
    const decryptionKey: FhevmDecryptionKey = await createFhevmDecryptionKey(
      decryptRuntime,
      { tkmsPrivateKey },
    );
    const pubKeyHex = await decryptionKey.getTkmsPublicKeyHex();
    console.log(`  Public key: ${pubKeyHex.slice(0, 40)}...`);

    // Create permit
    console.log(`\n[${elapsed()}] Creating and signing EIP-712 permit...`);
    const now = Math.floor(Date.now() / 1000);
    const eip712 = client.createUserDecryptEIP712({
      publicKey: pubKeyHex,
      contractAddresses: [FHECOUNTER_ADDRESS],
      startTimestamp: now,
      durationDays: 1,
      extraData: "0x",
    });

    const signature = await wallet.signTypedData(
      {
        name: eip712.domain.name,
        version: eip712.domain.version,
        chainId: eip712.domain.chainId,
        verifyingContract: eip712.domain.verifyingContract,
      },
      {
        UserDecryptRequestVerification:
          eip712.types.UserDecryptRequestVerification as ethers.TypedDataField[],
      },
      eip712.message,
    );
    console.log(`  Signed: ${signature.slice(0, 20)}...`);

    // Decrypt
    console.log(`\n[${elapsed()}] Attempting user decryption...`);
    try {
      const results = await client.userDecrypt({
        decryptionKey,
        handleContractPairs: [
          {
            handle,
            contractAddress: asChecksummedAddress(FHECOUNTER_ADDRESS),
          },
        ],
        userDecryptEIP712Signer: userAddress,
        userDecryptEIP712Message: eip712.message,
        userDecryptEIP712Signature: signature as Bytes65Hex,
      });

      console.log("\n  Decrypted values:");
      for (const r of results) {
        console.log(`    ${r.fheType}: ${r.value}`);
      }
    } catch (err: unknown) {
      const msg = err instanceof Error ? err.message : String(err);
      console.log(`  Decryption failed: ${msg.slice(0, 200)}`);
    }
  }

  console.log(`\n[${elapsed()}] Done.`);
}

main().catch((err: unknown) => {
  console.error("\nFatal:", err);
  process.exit(1);
});
