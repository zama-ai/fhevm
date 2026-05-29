// Minimal hardhat config for the LayerZero V2 wiring workspace. This project
// does NOT compile any Solidity contracts of its own — its only purpose is to
// host the `lz:oapp:*` tasks from `@layerzerolabs/toolbox-hardhat` so we can
// run `lz:oapp:wire` against a ConfidentialBridge that was already deployed
// by the parent host-contracts project (see ../tasks/taskDeploy.ts).
//
// The bridge proxy is referenced by `address` (read from env) in
// `layerzero.config.ts` — no hardhat-deploy artifacts needed here.

import 'dotenv/config';

import 'hardhat-deploy';
import '@nomiclabs/hardhat-ethers';
import '@layerzerolabs/toolbox-hardhat';

import { EndpointId } from '@layerzerolabs/lz-definitions';
import { resolve } from 'path';

import dotenv from 'dotenv';
import { HardhatUserConfig, HttpNetworkAccountsUserConfig } from 'hardhat/types';

// Load the parent project's .env so we can reuse DEPLOYER_PRIVATE_KEY, RPC URLs, etc.
dotenv.config({ path: resolve(__dirname, '../.env') });

const PRIVATE_KEY = process.env.DEPLOYER_PRIVATE_KEY;
const accounts: HttpNetworkAccountsUserConfig | undefined = PRIVATE_KEY
  ? [PRIVATE_KEY.startsWith('0x') ? PRIVATE_KEY : `0x${PRIVATE_KEY}`]
  : undefined;

if (!accounts) {
  console.warn(
    '[lz-wiring/hardhat.config.ts] DEPLOYER_PRIVATE_KEY not set — wiring transactions will not be able to broadcast.',
  );
}

const config: HardhatUserConfig = {
  paths: {
    // No contracts in this workspace — point sources at an empty dir so hardhat
    // doesn't try to compile anything in the parent project.
    sources: './noop',
    cache: './cache',
    artifacts: './artifacts',
  },
  solidity: '0.8.24',
  networks: {
    sepolia: {
      eid: EndpointId.SEPOLIA_V2_TESTNET,
      url: process.env.SEPOLIA_RPC_URL || 'https://sepolia.drpc.org',
      accounts,
    },
    'polygon-amoy': {
      eid: EndpointId.AMOY_V2_TESTNET,
      url: process.env.POLYGON_AMOY_RPC_URL || 'https://rpc-amoy.polygon.technology',
      accounts,
    },
  },
};

export default config;
