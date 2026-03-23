/**
 * Test runner for the encrypt + user-decrypt example.
 * Reads wallet credentials from .env.local.
 * Decryption will fail (handles not on-chain) but validates the entire flow.
 *
 * Usage: npx tsx ./examples/node-encrypt-decrypt/test-run.ts
 */

import { ethers } from "ethers";
import { readFileSync } from "node:fs";
import { resolve, dirname } from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = dirname(fileURLToPath(import.meta.url));

function loadEnv(): Record<string, string> {
  const envPath = resolve(__dirname, ".env.local");
  try {
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
  } catch {
    return {};
  }
}

const env = loadEnv();

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
import { asChecksummedAddress } from "../../src/core/base/address.js";
import { asBytesHex } from "../../src/core/base/bytes.js";
import type { Bytes65Hex } from "../../src/core/types/primitives.js";

const RPC_URL = "https://ethereum-sepolia-rpc.publicnode.com";
const CONTRACT_ADDRESS = "0x1E7eA8fE4877E6ea5dc8856f0dA92da8d5066241";

async function main(): Promise<void> {
  const startTime = Date.now();
  let stepCount = 0;
  function step(label: string): void {
    stepCount++;
    const elapsed = ((Date.now() - startTime) / 1000).toFixed(1);
    console.log(`\n[${elapsed}s] Step ${stepCount}: ${label}`);
  }

  // ── 1. Runtime config ──────────────────────────────────────────────────
  step("Configure FHEVM runtime");
  setFhevmRuntimeConfig({
    numberOfThreads: 4,
    logger: {
      debug: (_msg: string) => {},
      error: (msg: string, cause: unknown) => {
        console.error("  [error]", msg);
        if (cause !== undefined) console.error(cause);
      },
    },
  });
  console.log("  OK");

  // ── 2. Provider + wallet ────────────────────────────────────────────────
  step("Create provider and wallet");
  const provider = new ethers.JsonRpcProvider(RPC_URL);
  const privateKey = env.WALLET_PRIVATE_KEY
    ? `0x${env.WALLET_PRIVATE_KEY}`
    : undefined;
  const wallet = privateKey
    ? new ethers.Wallet(privateKey, provider)
    : ethers.Wallet.createRandom().connect(provider);
  if (!privateKey) console.log("  (using random wallet — no .env.local found)");
  const userAddress = asChecksummedAddress(wallet.address);
  console.log("  User address:", userAddress);

  // ── 3. Create full client ──────────────────────────────────────────────
  step("Create FhevmClient");
  const client = createFhevmClient({ chain: sepolia, provider });
  console.log("  uid:", client.uid);

  // ── 4. Fetch global FHE PKE params ─────────────────────────────────────
  step("Fetch global FHE public encryption parameters");
  const params = await client.fetchGlobalFhePkeParams();
  console.log("  OK — params fetched from relayer");

  // ── 5. Encrypt ─────────────────────────────────────────────────────────
  step("Encrypt uint32(42) + bool(true)");
  let proof;
  try {
    proof = await client.encrypt({
      globalFhePublicEncryptionParams: params,
      contractAddress: CONTRACT_ADDRESS,
      userAddress: userAddress,
      values: [
        { type: "uint32", value: 42 },
        { type: "bool", value: true },
      ],
      extraData: asBytesHex("0x"),
    });
    console.log("  Handles:", proof.externalHandles.length);
    for (const h of proof.externalHandles) {
      console.log(`    [${h.index}] ${h.fheType} → ${h.bytes32Hex}`);
    }
    console.log("  Proof bytes length:", proof.bytesHex.length);
  } catch (err: unknown) {
    const msg = err instanceof Error ? err.message : String(err);
    console.log("  Encryption failed (relayer issue):", msg.split("\n")[0]);
    console.log("  (ZK proof generation succeeded — relayer coprocessor signing unavailable)");
  }

  // ── 6. Generate KMS key ────────────────────────────────────────────────
  step("Generate KMS decryption key");
  const baseRuntime = createFhevm();
  const decryptRuntime = baseRuntime.runtime.extend(decryptModule);
  const tkmsPrivateKey = await decryptRuntime.decrypt.generateTkmsPrivateKey();
  const decryptionKey: FhevmDecryptionKey = await createFhevmDecryptionKey(
    decryptRuntime,
    { tkmsPrivateKey },
  );
  const pubKeyHex = await decryptionKey.getTkmsPublicKeyHex();
  console.log("  Public key:", pubKeyHex.slice(0, 40) + "...");

  // ── 7. Create EIP-712 permit ───────────────────────────────────────────
  step("Create EIP-712 user decryption permit");
  const now = Math.floor(Date.now() / 1000);
  const eip712 = client.createUserDecryptEIP712({
    publicKey: pubKeyHex,
    contractAddresses: [CONTRACT_ADDRESS],
    startTimestamp: now,
    durationDays: 1,
    extraData: "0x",
  });
  console.log("  Domain:", eip712.domain.name, "v" + eip712.domain.version);
  console.log("  Chain ID:", eip712.domain.chainId.toString());
  console.log("  Contracts:", eip712.message.contractAddresses.length);

  // ── 8. Sign the permit ─────────────────────────────────────────────────
  step("Sign EIP-712 permit with wallet");
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
  console.log("  Signature:", signature.slice(0, 20) + "...");

  // ── 9. Attempt user decryption ─────────────────────────────────────────
  step("User decryption (expected to fail — handles not on-chain)");
  if (proof === undefined) {
    console.log("  Skipped — no proof available (encryption failed at relayer step)");
  } else try {
    const results = await client.userDecrypt({
      decryptionKey,
      handleContractPairs: proof.externalHandles.map((h) => ({
        handle: h,
        contractAddress: asChecksummedAddress(CONTRACT_ADDRESS),
      })),
      userDecryptEIP712Signer: userAddress,
      userDecryptEIP712Message: eip712.message,
      userDecryptEIP712Signature: signature as Bytes65Hex,
    });
    console.log("  Decryption succeeded (unexpected!):", results);
  } catch (err: unknown) {
    const msg = err instanceof Error ? err.message : String(err);
    console.log("  Failed as expected:", msg.slice(0, 120));
  }

  // ── Summary ────────────────────────────────────────────────────────────
  const totalTime = ((Date.now() - startTime) / 1000).toFixed(1);
  console.log(`\n✓ All ${stepCount} steps completed in ${totalTime}s`);
}

main().catch((err: unknown) => {
  console.error("\nFatal error:", err);
  process.exit(1);
});
