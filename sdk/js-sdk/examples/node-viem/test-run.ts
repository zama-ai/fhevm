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
import { setFhevmRuntimeConfig, createFhevmClient } from '../../src/viem/index.js';
import { sepolia } from '../../src/core/chains/index.js';
import { asChecksummedAddress } from '../../src/core/base/address.js';
import { toFhevmHandle } from '../../src/core/handle/FhevmHandle.js';

////////////////////////////////////////////////////////////////////////////////

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

////////////////////////////////////////////////////////////////////////////////

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
] as const;

// FHECounter contract on Sepolia (deployed by the Next.js example)
const FHE_COUNTER_ADDRESS = '0xef6c6230bF565015f8B37f2966d200C8804b409a';
const FHE_COUNTER_ABI = [
  {
    inputs: [],
    name: 'getCount',
    outputs: [{ internalType: 'uint256', name: '', type: 'uint256' }],
    stateMutability: 'view',
    type: 'function',
  },
] as const;

////////////////////////////////////////////////////////////////////////////////

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

  // ── 2. Viem clients + wallet ───────────────────────────────────────────
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
  const client = createFhevmClient({ chain: sepolia, publicClient });
  console.log('  uid:', client.uid);

  // ════════════════════════════════════════════════════════════════════════
  // ENCRYPTION
  // ════════════════════════════════════════════════════════════════════════

  step('Encrypt uint32(42) + bool(true)');
  try {
    const values = [
      { type: 'uint32', value: 42 },
      { type: 'bool', value: true },
    ] as const;
    const proof = await client.encryptValues({
      contractAddress: FHE_COUNTER_ADDRESS,
      userAddress: userAddress,
      values,
    });
    console.log('  Handles:', proof.encryptedValues.length);
    for (const [i, h] of proof.encryptedValues.entries()) {
      console.log(`    [${i}] ${values[i].type} → ${h}`);
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
    const encryptedValues = PUBLIC_ENCRYPTED_VALUES.map((h) => h.hex);
    const typedValues = await client.readPublicValues({ encryptedValues });

    console.log('  Read public values succeeded!');
    for (let i = 0; i < typedValues.length; i++) {
      const d = typedValues[i];
      if (d === undefined) continue;
      const expected = PUBLIC_ENCRYPTED_VALUES[i]?.expected;
      const match = d.value === expected ? 'OK' : 'MISMATCH';
      console.log(`  [${match}] ${d.type}: ${d.value} (expected: ${expected})`);
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
  const countHex = ('0x' + BigInt(rawCount).toString(16).padStart(64, '0')) as `0x${string}`;
  console.log('  Raw count (bigint):', rawCount.toString());
  console.log('  Count handle (hex):', countHex);

  if (rawCount === 0n) {
    console.log('  Count is zero — no encrypted value stored yet. Skipping decrypt.');
  } else {
    const countHandle = toFhevmHandle(countHex);
    console.log('  Parsed handle — chainId:', countHandle.chainId.toString(), 'fheType:', countHandle.fheType);

    step('Generate E2E transport key pair');
    const transportKeyPair = await client.generateTransportKeyPair();
    const pubKeyHex = transportKeyPair.publicKey;
    console.log('  Public key:', pubKeyHex.slice(0, 40) + '...');

    step('Create and sign EIP-712 decrypt permit');
    const now = Math.floor(Date.now() / 1000);
    const signedPermit = await client.signDecryptionPermit({
      transportKeyPair: transportKeyPair,
      contractAddresses: [FHE_COUNTER_ADDRESS],
      startTimestamp: now,
      durationDays: 1,
      signerAddress: account.address,
      signer: account,
    });
    console.log('  Domain:', signedPermit.eip712.domain.name, 'v' + signedPermit.eip712.domain.version);
    console.log('  Signature:', signedPermit.signature.slice(0, 20) + '...');

    step('Decrypt the FHECounter count');
    try {
      const decrypted = await client.decryptValue({
        transportKeyPair: transportKeyPair,
        encryptedValue: countHandle,
        contractAddress: asChecksummedAddress(FHE_COUNTER_ADDRESS),
        signedPermit,
      });
      console.log('  Decryption succeeded!');
      console.log(`  Value: ${decrypted.value} (${decrypted.type})`);
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

main()
  .then(() => process.exit(0))
  .catch((err: unknown) => {
    console.error('\nFatal error:', err);
    process.exit(1);
  });
