/**
 * Generates hardhat-deploy artifact files for the deployed ConfidentialBridge
 * proxies so `lz:oapp:wire` can resolve them via `contractName`. Reads:
 *
 *   - SEPOLIA_BRIDGE_ADDRESS         → deployments/sepolia/ConfidentialBridge.json
 *   - POLYGON_AMOY_BRIDGE_ADDRESS    → deployments/polygon-amoy/ConfidentialBridge.json
 *
 * The ABI is sourced from the parent host-contracts compile output. Run the
 * parent's `npm run compile` (or `npx hardhat compile`) at least once before
 * invoking this script — the script will tell you if the artifact is missing.
 *
 * This script is idempotent: re-running it just rewrites the per-network
 * `ConfidentialBridge.json` files. The `.chainId` files are generated too
 * because hardhat-deploy needs them to recognise a deployments directory.
 *
 * Run via `pnpm prewire` (auto-invoked by `pnpm wire`).
 */

import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'fs';
import { resolve } from 'path';

import dotenv from 'dotenv';

dotenv.config({ path: resolve(__dirname, '..', '..', '.env') });

interface NetworkDeployment {
  network: string; // hardhat-deploy directory name (must match hardhat.config.ts:networks key)
  chainId: number;
  envVar: string;
}

const NETWORKS: NetworkDeployment[] = [
  { network: 'sepolia', chainId: 11155111, envVar: 'SEPOLIA_BRIDGE_ADDRESS' },
  { network: 'polygon-amoy', chainId: 80002, envVar: 'POLYGON_AMOY_BRIDGE_ADDRESS' },
];

const PARENT_ARTIFACT = resolve(
  __dirname,
  '..',
  '..',
  'artifacts',
  'contracts',
  'bridge',
  'ConfidentialBridge.sol',
  'ConfidentialBridge.json',
);

function main() {
  if (!existsSync(PARENT_ARTIFACT)) {
    throw new Error(
      `Parent ConfidentialBridge artifact not found at ${PARENT_ARTIFACT}. ` +
        `Run \`npx hardhat compile\` in the parent host-contracts project first.`,
    );
  }
  const parentArtifact = JSON.parse(readFileSync(PARENT_ARTIFACT, 'utf8')) as {
    abi: unknown[];
    bytecode: string;
  };
  const abi = parentArtifact.abi;
  if (!Array.isArray(abi) || abi.length === 0) {
    throw new Error(`Parent ConfidentialBridge artifact has no ABI: ${PARENT_ARTIFACT}`);
  }

  for (const net of NETWORKS) {
    const address = process.env[net.envVar];
    if (!address) {
      console.log(
        `[sync-deployments] ${net.envVar} not set — skipping ${net.network}. ` +
          `(Set it once \`task:deployBridge\` has run on ${net.network}.)`,
      );
      continue;
    }
    const dir = resolve(__dirname, '..', 'deployments', net.network);
    mkdirSync(dir, { recursive: true });
    writeFileSync(resolve(dir, '.chainId'), String(net.chainId));
    const deployment = {
      address,
      abi,
      // hardhat-deploy doesn't strictly require these but it likes to see them.
      bytecode: parentArtifact.bytecode,
      transactionHash: '0x' + '0'.repeat(64),
      receipt: { contractAddress: address },
      args: [],
      numDeployments: 1,
    };
    writeFileSync(
      resolve(dir, 'ConfidentialBridge.json'),
      JSON.stringify(deployment, null, 2),
    );
    console.log(`[sync-deployments] ${net.network} ← ConfidentialBridge @ ${address}`);
  }
}

main();
