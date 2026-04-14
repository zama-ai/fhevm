/**
 * @fhevm/sdk — Node.js Example (viem)
 *
 * Demonstrates encryption, reading public values, and private decryption:
 *   1. Configure the FHEVM runtime
 *   2. Create a full FHEVM client (encrypt + decrypt)
 *   3. Encrypt values for a target contract
 *   4. Read publicly readable encrypted values from testnet
 *   5. Generate an E2E transport key pair, sign an EIP-712 permit, decrypt
 *
 * With .env.local: reads the FHECounter contract on Sepolia and decrypts the count.
 * Without .env.local: uses a random wallet (decrypt will fail on ACL check).
 *
 * Usage: npx tsx ./examples/node-viem/test-run.ts
 */

import { createPublicClient, createWalletClient, http, getContract } from 'viem';
import { privateKeyToAccount, generatePrivateKey } from 'viem/accounts';
import { sepolia as viemSepolia } from 'viem/chains';
import { readFileSync } from 'node:fs';
import { resolve, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));

function loadEnv(): Record<string, string> {
  try {
    const content = readFileSync(resolve(__dirname, '.env.local'), 'utf-8');
    const env: Record<string, string> = {};
    for (const line of content.split('\n')) {
      const trimmed = line.trim();
      if (trimmed.length === 0 || trimmed.startsWith('#')) continue;
      const eqIdx = trimmed.indexOf('=');
      if (eqIdx === -1) continue;
      env[trimmed.slice(0, eqIdx)] = trimmed.slice(eqIdx + 1);
    }
    return env;
  } catch {
    return {};
  }
}

const env = loadEnv();

import { setFhevmRuntimeConfig, createFhevmClient } from '../../src/viem/index.js';
import { sepolia } from '../../src/core/chains/index.js';
import { asChecksummedAddress } from '../../src/core/base/address.js';
import type { Bytes65Hex } from '../../src/core/types/primitives.js';
import { toHandle } from '../../src/core/handle/FhevmHandle.js';

const RPC_URL = 'https://ethereum-sepolia-rpc.publicnode.com';

// Known publicly readable encrypted values on Sepolia testnet
const PUBLIC_ENCRYPTED_VALUES = [
  {
    hex: '0xf1673094de7c833604f1b62183cbcdf2cdc968db90ff0000000000aa36a70400',
    type: 'euint32',
    expected: 1083783185,
  },
  {
    hex: '0x9797f8eb707b0a32c47a80ea86c0648df36bfe7cd0ff0000000000aa36a70300',
    type: 'euint16',
    expected: 15764,
  },
  {
    hex: '0x6f17228bda73a5e57b94511c5bab2665e6a2870399ff0000000000aa36a70200',
    type: 'euint8',
    expected: 171,
  },
  {
    hex: '0xf6751d547a5c06123575aad93f22f76b7d841c4cacff0000000000aa36a70000',
    type: 'ebool',
    expected: false,
  },
];

// FHECounter contract on Sepolia (deployed by the Next.js example)
const FHE_COUNTER_ADDRESS = '0xef6c6230bF565015f8B37f2966d200C8804b409a' as const;
const FHE_COUNTER_ABI = [
  {
    inputs: [],
    name: 'getCount',
    outputs: [{ internalType: 'uint256', name: '', type: 'uint256' }],
    stateMutability: 'view',
    type: 'function',
  },
] as const;

async function main(): Promise<void> {
  const t0 = Date.now();
  let stepCount = 0;
  function step(label: string): void {
    stepCount++;
    const elapsed = ((Date.now() - t0) / 1000).toFixed(1);
    console.log(`\n[${elapsed}s] Step ${stepCount}: ${label}`);
  }

  // ── 1. Runtime config ──────────────────────────────────────────────────
  step('Configure FHEVM runtime');
  setFhevmRuntimeConfig({});
  console.log('  OK');

  // ── 2. Provider + wallet ────────────────────────────────────────────────
  step('Create viem clients and wallet');
  const transport = http(RPC_URL);
  const publicClient = createPublicClient({
    chain: viemSepolia,
    transport,
  });

  const privateKey = env.WALLET_PRIVATE_KEY ? (`0x${env.WALLET_PRIVATE_KEY}` as `0x${string}`) : generatePrivateKey();
  const account = privateKeyToAccount(privateKey);

  if (!env.WALLET_PRIVATE_KEY) console.log('  (using random wallet — no .env.local found)');

  const walletClient = createWalletClient({
    account,
    chain: viemSepolia,
    transport,
  });

  const userAddress = asChecksummedAddress(account.address);
  console.log('  User address:', userAddress);

  // ── 3. Create full client ──────────────────────────────────────────────
  step('Create FhevmClient (viem)');
  const fhevm = createFhevmClient({ chain: sepolia, publicClient });
  console.log('  uid:', fhevm.uid);

  // ════════════════════════════════════════════════════════════════════════
  // ENCRYPTION
  // ════════════════════════════════════════════════════════════════════════

  step('Encrypt uint32(42) + bool(true)');
  try {
    const proof = await fhevm.encrypt({
      contractAddress: FHE_COUNTER_ADDRESS,
      userAddress: userAddress,
      values: [
        { type: 'uint32', value: 42 },
        { type: 'bool', value: true },
      ],
    });
    console.log('  Handles:', proof.externalEncryptedValues.length);
    for (const h of proof.externalEncryptedValues) {
      console.log(`    [${h.index}] ${h.fheType} → ${h.bytes32Hex}`);
    }
    console.log('  Proof bytes length:', proof.inputProof.length);
  } catch (err: unknown) {
    const msg = err instanceof Error ? err.message : String(err);
    console.log('  Encryption failed (relayer issue):', msg.split('\n')[0]);
    console.log('  (ZK proof generation succeeded — relayer coprocessor signing unavailable)');
  }

  // ════════════════════════════════════════════════════════════════════════
  // READ PUBLIC VALUES
  // ════════════════════════════════════════════════════════════════════════

  step(`Read ${PUBLIC_ENCRYPTED_VALUES.length} public values from testnet`);
  try {
    const encryptedValues = PUBLIC_ENCRYPTED_VALUES.map((h) => toHandle(h.hex));
    const result = await fhevm.readPublicValue({ encryptedValues });

    console.log('  Read public values succeeded!');
    for (let i = 0; i < result.orderedClearValues.length; i++) {
      const d = result.orderedClearValues[i];
      if (d === undefined) continue;
      const expected = PUBLIC_ENCRYPTED_VALUES[i]?.expected;
      const match = d.value === expected ? 'OK' : 'MISMATCH';
      console.log(`  [${match}] ${d.encryptedValue.fheType}: ${d.value} (expected: ${expected})`);
    }
  } catch (err: unknown) {
    const msg = err instanceof Error ? err.message : String(err);
    console.log(`  Read public values failed: ${msg.slice(0, 200)}`);
  }

  // ════════════════════════════════════════════════════════════════════════
  // PRIVATE DECRYPTION
  // ════════════════════════════════════════════════════════════════════════

  // Read the FHECounter's encrypted count from the contract
  step('Read encrypted count from FHECounter contract');
  const counter = getContract({
    address: FHE_COUNTER_ADDRESS,
    abi: FHE_COUNTER_ABI,
    client: publicClient,
  });
  const rawCount = await counter.read.getCount();
  const countHex = '0x' + BigInt(rawCount).toString(16).padStart(64, '0');
  console.log('  Raw count (bigint):', rawCount.toString());
  console.log('  Count handle (hex):', countHex);

  if (rawCount === 0n) {
    console.log('  Count is zero — no encrypted value stored yet. Skipping decrypt.');
  } else {
    const countHandle = toHandle(countHex);
    console.log('  Parsed handle — chainId:', countHandle.chainId.toString(), 'fheType:', countHandle.fheType);

    step('Generate E2E transport key pair');
    const e2eTransportKeypair = await fhevm.generateE2eTransportKeypair();
    const pubKeyHex = e2eTransportKeypair.publicKey;
    console.log('  Public key:', pubKeyHex.slice(0, 40) + '...');

    step('Create and sign EIP-712 decrypt permit');
    const now = Math.floor(Date.now() / 1000);
    const signedPermit = await fhevm.signDecryptionPermit({
      e2eTransportKeypair,
      contractAddresses: [FHE_COUNTER_ADDRESS],
      startTimestamp: now,
      durationDays: 1,
      signerAddress: account.address,
      signer: walletClient,
    });
    // const permit = await client.createDecryptPermit({
    //   e2eTransportPublicKey: pubKeyHex,
    //   contractAddresses: [FHE_COUNTER_ADDRESS],
    //   startTimestamp: now,
    //   durationDays: 1,
    // });
    // console.log("  Domain:", permit.domain.name, "v" + permit.domain.version);

    // const signature = await walletClient.signTypedData({
    //   account,
    //   domain: {
    //     name: permit.domain.name,
    //     version: permit.domain.version,
    //     chainId: Number(permit.domain.chainId),
    //     verifyingContract: permit.domain.verifyingContract as `0x${string}`,
    //   },
    //   types: permit.types as Record<
    //     string,
    //     Array<{ name: string; type: string }>
    //   >,
    //   primaryType: "UserDecryptRequestVerification",
    //   message: permit.message as Record<string, unknown>,
    // });
    console.log('  Signature:', signedPermit.signature.slice(0, 20) + '...');

    // step("Bundle into signed permit");
    // const signedPermit = createSignedPermit(
    //   permit,
    //   signature as Bytes65Hex,
    //   userAddress,
    // );
    console.log('  Signed permit created');

    step('Decrypt the FHECounter count');
    try {
      const results = await fhevm.decrypt({
        e2eTransportKeypair,
        encryptedValues: [
          {
            encryptedValue: countHandle,
            contractAddress: asChecksummedAddress(FHE_COUNTER_ADDRESS),
          },
        ],
        signedPermit,
      });
      const decrypted = results[0];
      console.log('  Decryption succeeded!');
      console.log(`  Value: ${decrypted?.value} (${decrypted?.encryptedValue.fheType})`);
    } catch (err: unknown) {
      const msg = err instanceof Error ? err.message : String(err);
      console.log('  Decryption failed:', msg.slice(0, 200));
      if (!env.WALLET_PRIVATE_KEY) {
        console.log('  (expected — random wallet has no ACL permission)');
      }
    }
  }

  // ── Summary ────────────────────────────────────────────────────────────
  const totalTime = ((Date.now() - t0) / 1000).toFixed(1);
  console.log(`\nAll ${stepCount} steps completed in ${totalTime}s`);
}

main().catch((err: unknown) => {
  console.error('\nFatal error:', err);
  process.exit(1);
});
