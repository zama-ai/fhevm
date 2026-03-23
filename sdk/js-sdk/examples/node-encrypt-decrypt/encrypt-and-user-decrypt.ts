/**
 * @fhevm/sdk — Node.js Example: Encryption & User Decryption
 *
 * This example demonstrates the full encrypt → user-decrypt flow:
 *   1. Configure the FHEVM runtime
 *   2. Create clients (encrypt + decrypt)
 *   3. Fetch the global FHE public encryption parameters
 *   4. Encrypt values for a target contract
 *   5. Create a KMS decryption key
 *   6. Build and sign an EIP-712 user decryption permit
 *   7. Decrypt the encrypted handles
 *
 * Prerequisites:
 *   - Node.js >= 22
 *   - A funded wallet with access to the target contract on Sepolia
 *   - The target contract must have called TFHE.allow() for the user address
 *
 * Usage:
 *   npx tsx ./examples/node-encrypt-decrypt/encrypt-and-user-decrypt.ts
 */

import { ethers } from "ethers";

// --- SDK imports (ethers adapter) ---
import {
  setFhevmRuntimeConfig,
  createFhevmClient,
} from "../../src/ethers/index.js";

// --- Chain definition ---
import { sepolia } from "../../src/core/chains/index.js";

// --- Core utilities ---
import {
  createFhevmDecryptionKey,
  type FhevmDecryptionKey,
} from "../../src/core/user/FhevmDecryptionKey-p.js";
import { createFhevm } from "../../src/ethers/clients/createFhevm.js";
import { decryptModule } from "../../src/core/modules/decrypt/module/index.js";
import { toFhevmHandle } from "../../src/core/handle/FhevmHandle.js";
import { asChecksummedAddress } from "../../src/core/base/address.js";
import { asBytesHex } from "../../src/core/base/bytes.js";

import type { Bytes65Hex, ChecksummedAddress } from "../../src/core/types/primitives.js";

// ============================================================================
// Configuration (reads from .env.local)
// ============================================================================

import { readFileSync } from "node:fs";
import { resolve, dirname } from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = dirname(fileURLToPath(import.meta.url));

function loadEnv(): Record<string, string> {
  const envPath = resolve(__dirname, ".env.local");
  const content = readFileSync(envPath, "utf-8");
  const env: Record<string, string> = {};
  for (const line of content.split("\n")) {
    const trimmed = line.trim();
    if (trimmed.length === 0 || trimmed.startsWith("#")) continue;
    const eqIdx = trimmed.indexOf("=");
    if (eqIdx === -1) continue;
    env[trimmed.slice(0, eqIdx)] = trimmed.slice(eqIdx + 1);
  }
  return env;
}

const env = loadEnv();

const RPC_URL = "https://ethereum-sepolia-rpc.publicnode.com";
// Replace with your contract address
const CONTRACT_ADDRESS = "0x1E7eA8fE4877E6ea5dc8856f0dA92da8d5066241";
// Read wallet private key from .env.local
const WALLET_PRIVATE_KEY = `0x${env.WALLET_PRIVATE_KEY}`;

// ============================================================================
// Main
// ============================================================================

async function main(): Promise<void> {
  // --------------------------------------------------------------------------
  // 1. Configure the FHEVM runtime (once, before creating any client)
  // --------------------------------------------------------------------------
  setFhevmRuntimeConfig({
    numberOfThreads: 4,
    logger: {
      debug: (msg: string) => console.log("[debug]", msg),
      error: (msg: string, cause: unknown) => {
        console.error("[error]", msg);
        if (cause !== undefined) console.error(cause);
      },
    },
  });

  // --------------------------------------------------------------------------
  // 2. Create an ethers provider and signer
  // --------------------------------------------------------------------------
  const provider = new ethers.JsonRpcProvider(RPC_URL);
  const wallet = new ethers.Wallet(WALLET_PRIVATE_KEY, provider);
  const userAddress = asChecksummedAddress(wallet.address);

  console.log("User address:", userAddress);

  // --------------------------------------------------------------------------
  // 3. Create the full FHEVM client (encrypt + decrypt + relayer)
  // --------------------------------------------------------------------------
  const client = createFhevmClient({
    chain: sepolia,
    provider,
  });

  console.log("FHEVM client created (uid:", client.uid, ")");

  // --------------------------------------------------------------------------
  // 4. Fetch the global FHE public encryption parameters
  // --------------------------------------------------------------------------
  console.log("Fetching global FHE public encryption parameters...");
  const globalFhePkeParams = await client.fetchGlobalFhePkeParams();
  console.log("Global FHE PKE params fetched.");

  // --------------------------------------------------------------------------
  // 5. Encrypt values
  // --------------------------------------------------------------------------
  console.log("Encrypting values...");

  const proof = await client.encrypt({
    globalFhePublicEncryptionParams: globalFhePkeParams,
    contractAddress: CONTRACT_ADDRESS,
    userAddress: userAddress,
    values: [
      { type: "uint32", value: 42 },
      { type: "bool", value: true },
    ],
    extraData: asBytesHex("0x"),
  });

  console.log("Encryption complete.");
  console.log("  Handles:", proof.externalHandles.length);
  for (const h of proof.externalHandles) {
    console.log(`    [${h.index}] ${h.fheType} → ${h.bytes32Hex}`);
  }
  console.log("  Proof bytes:", proof.bytesHex.slice(0, 40), "...");

  // --------------------------------------------------------------------------
  // 6. Create a KMS decryption key
  // --------------------------------------------------------------------------
  //
  // We need a lightweight runtime with the decrypt module to generate
  // and manage KMS keys.
  //
  const baseRuntime = createFhevm();
  const decryptRuntime = baseRuntime.runtime.extend(decryptModule);

  console.log("Generating KMS private key...");
  const tkmsPrivateKey = await decryptRuntime.decrypt.generateTkmsPrivateKey();

  // Wrap the private key in a FhevmDecryptionKey
  // (the private key is never exposed through this interface)
  const decryptionKey: FhevmDecryptionKey = await createFhevmDecryptionKey(
    decryptRuntime,
    { tkmsPrivateKey },
  );

  const publicKeyHex = await decryptionKey.getTkmsPublicKeyHex();
  console.log("KMS public key:", publicKeyHex.slice(0, 40), "...");

  // --------------------------------------------------------------------------
  // 7. Fetch extraData for KMS context
  // --------------------------------------------------------------------------
  console.log("Fetching extraData for KMS context...");
  const extraData = await client.getExtraData({});
  console.log("ExtraData:", extraData.slice(0, 20), "...");

  // --------------------------------------------------------------------------
  // 8. Create and sign an EIP-712 user decryption permit
  // --------------------------------------------------------------------------
  const now = Math.floor(Date.now() / 1000);

  const eip712 = client.createUserDecryptEIP712({
    publicKey: publicKeyHex,
    contractAddresses: [CONTRACT_ADDRESS],
    startTimestamp: now,
    durationDays: 1,
    extraData: extraData,
  });

  console.log("EIP-712 permit created. Signing with wallet...");

  // Sign the EIP-712 typed data with the user's wallet
  const signature = await wallet.signTypedData(
    // Domain — ethers wants plain object without readonly
    {
      name: eip712.domain.name,
      version: eip712.domain.version,
      chainId: eip712.domain.chainId,
      verifyingContract: eip712.domain.verifyingContract,
    },
    // Types (excluding EIP712Domain which ethers adds automatically)
    {
      UserDecryptRequestVerification:
        eip712.types.UserDecryptRequestVerification as ethers.TypedDataField[],
    },
    // Message
    eip712.message,
  );

  console.log("Permit signed:", signature.slice(0, 20), "...");

  // --------------------------------------------------------------------------
  // 9. Decrypt the encrypted handles via user decryption
  // --------------------------------------------------------------------------
  //
  // Note: This will only succeed if:
  //   - The handles exist on-chain (the contract stored them)
  //   - The ACL allows the user to decrypt (TFHE.allow was called)
  //   - The permit is valid (correct signer, valid time window, matching contract)
  //
  console.log("Decrypting handles...");

  const decryptedResults = await client.userDecrypt({
    decryptionKey,
    handleContractPairs: proof.externalHandles.map((h) => ({
      handle: h,
      contractAddress: asChecksummedAddress(CONTRACT_ADDRESS),
    })),
    userDecryptEIP712Signer: userAddress,
    userDecryptEIP712Message: eip712.message,
    userDecryptEIP712Signature: signature as Bytes65Hex,
  });

  // --------------------------------------------------------------------------
  // 10. Print results
  // --------------------------------------------------------------------------
  console.log("\nDecrypted values:");
  for (const result of decryptedResults) {
    console.log(`  ${result.fheType}: ${result.value}`);
  }
}

// ============================================================================
// Run
// ============================================================================

main().catch((err: unknown) => {
  console.error("Fatal error:", err);
  process.exit(1);
});
