#!/usr/bin/env node

require('dotenv').config({ path: require('path').resolve(__dirname, '../.env') });
const { ethers } = require('ethers');
const { findDeploymentBlock, isValidAddress } = require('./get-deployment-block');
const { queryEventsInChunks } = require('./lib/events');

// PauserSet event signatures
const PAUSER_SET_ABI = [
  'event AddPauser(address account)',
  'event RemovePauser(address account)',
  'event SwapPauser(address oldAccount, address newAccount)',
];

// Chain configurations from environment, grouped by network.
// `symbol` is the native gas token, used only for balance display.
// `fromBlock` (optional) is the PauserSet deployment block. When set, it skips
// the binary-search deployment-block detection, which needs an archive node and
// fails on pruned/public RPCs (e.g. most free Sepolia endpoints). Look the block
// up once on the relevant explorer and pin it via the *_FROM_BLOCK env var.
// Select a network with --mainnet (default) or --testnet.
const NETWORKS = {
  mainnet: {
    ethereum: {
      name: 'Ethereum',
      symbol: 'ETH',
      rpcUrl: process.env.RPC_ETHEREUM,
      pauserSetAddress: process.env.PAUSER_SET_ETHEREUM,
      fromBlock: process.env.PAUSER_SET_ETHEREUM_FROM_BLOCK,
    },
    gateway: {
      name: 'Gateway',
      symbol: 'ETH',
      rpcUrl: process.env.RPC_GATEWAY,
      pauserSetAddress: process.env.PAUSER_SET_GATEWAY,
      fromBlock: process.env.PAUSER_SET_GATEWAY_FROM_BLOCK,
    },
    polygon: {
      name: 'Polygon',
      symbol: 'POL',
      rpcUrl: process.env.RPC_POLYGON,
      pauserSetAddress: process.env.PAUSER_SET_POLYGON,
      fromBlock: process.env.PAUSER_SET_POLYGON_FROM_BLOCK,
    },
  },
  testnet: {
    ethereum: {
      name: 'Sepolia',
      symbol: 'ETH',
      rpcUrl: process.env.RPC_SEPOLIA,
      pauserSetAddress: process.env.PAUSER_SET_SEPOLIA,
      fromBlock: process.env.PAUSER_SET_SEPOLIA_FROM_BLOCK,
    },
    gateway: {
      name: 'Gateway Testnet',
      symbol: 'ETH',
      rpcUrl: process.env.RPC_GATEWAY_TESTNET,
      pauserSetAddress: process.env.PAUSER_SET_GATEWAY_TESTNET,
      fromBlock: process.env.PAUSER_SET_GATEWAY_TESTNET_FROM_BLOCK,
    },
    // Polygon Amoy: add once a testnet PauserSet is deployed.
    polygon: {
      name: 'Polygon Amoy',
      symbol: 'POL',
      rpcUrl: process.env.RPC_POLYGON_AMOY,
      pauserSetAddress: process.env.PAUSER_SET_POLYGON_AMOY,
      fromBlock: process.env.PAUSER_SET_POLYGON_AMOY_FROM_BLOCK,
    },
  },
};

function parseNetworkFlag(argv) {
  const flags = argv.filter((a) => a === '--mainnet' || a === '--testnet');
  if (flags.length > 1) {
    console.error('Error: pass only one of --mainnet or --testnet');
    process.exit(1);
  }
  return flags[0] === '--testnet' ? 'testnet' : 'mainnet';
}

const NETWORK = parseNetworkFlag(process.argv.slice(2));
const CHAINS = NETWORKS[NETWORK];

async function getPausersForChain(chainConfig) {
  const { name, rpcUrl, pauserSetAddress } = chainConfig;

  if (!rpcUrl) {
    console.log(`  Skipping ${name}: RPC_URL not configured`);
    return null;
  }

  if (!pauserSetAddress) {
    console.log(`  Skipping ${name}: PAUSER_SET address not configured`);
    return null;
  }

  if (!isValidAddress(pauserSetAddress)) {
    console.log(`  Skipping ${name}: Invalid address format`);
    return null;
  }

  const provider = new ethers.JsonRpcProvider(rpcUrl);
  const contract = new ethers.Contract(pauserSetAddress, PAUSER_SET_ABI, provider);

  // Determine the block to start scanning events from.
  // Priority: explicit *_FROM_BLOCK override > binary-search detection > scan from 0.
  // The binary search relies on eth_getCode at historical blocks, which requires an
  // archive node; pruned/public RPCs (e.g. most free Sepolia endpoints) either error
  // or return "0x" and would yield a wrong block. eth_getLogs, used below, is served
  // from genesis even by pruned nodes, so falling back to block 0 is safe (just slower).
  let fromBlock;
  if (chainConfig.fromBlock !== undefined && chainConfig.fromBlock !== '') {
    fromBlock = Number(chainConfig.fromBlock);
    if (!Number.isInteger(fromBlock) || fromBlock < 0) {
      console.log(`  Skipping ${name}: invalid fromBlock "${chainConfig.fromBlock}"`);
      return null;
    }
    console.log(`  Using configured deployment block: ${fromBlock}`);
  } else {
    console.log(`  Finding deployment block for ${pauserSetAddress}...`);
    try {
      fromBlock = await findDeploymentBlock(pauserSetAddress, { rpcUrl, silent: true });
      console.log(`  Deployment block: ${fromBlock}`);
    } catch (error) {
      fromBlock = 0;
      console.log(`  Could not detect deployment block (${error.message}).`);
      console.log('  Falling back to scanning from block 0 (slower). Set the *_FROM_BLOCK');
      console.log('  env var to the PauserSet deployment block to skip this.');
    }
  }

  // Get current block
  const toBlock = await provider.getBlockNumber();
  console.log(`  Current block: ${toBlock}`);

  // Query all pauser events in a single pass over the block range ('*' = every
  // event of the contract), then split by type — instead of one full sweep per
  // event signature. The contract only emits the three events below.
  console.log('  Fetching pauser events...');

  const logs = await queryEventsInChunks(contract, '*', fromBlock, toBlock, 'events');

  const allEvents = [];
  const counts = { AddPauser: 0, RemovePauser: 0, SwapPauser: 0 };

  for (const e of logs) {
    switch (e.eventName) {
      case 'AddPauser':
        counts.AddPauser++;
        allEvents.push({ type: 'add', account: e.args.account, block: e.blockNumber, logIndex: e.index });
        break;
      case 'RemovePauser':
        counts.RemovePauser++;
        allEvents.push({ type: 'remove', account: e.args.account, block: e.blockNumber, logIndex: e.index });
        break;
      case 'SwapPauser':
        counts.SwapPauser++;
        allEvents.push({
          type: 'swap',
          oldAccount: e.args.oldAccount,
          newAccount: e.args.newAccount,
          block: e.blockNumber,
          logIndex: e.index,
        });
        break;
      // Ignore any other / undecodable log the contract may emit.
    }
  }

  // Report per-type counts (preserves the previous "found N events" breakdown).
  for (const [name, count] of Object.entries(counts)) {
    console.log(`    ${name}: found ${count} events`);
  }

  // Sort all events by block number and log index
  allEvents.sort((a, b) => {
    if (a.block !== b.block) return a.block - b.block;
    return a.logIndex - b.logIndex;
  });

  // Process events chronologically to build current pauser set
  const pausers = new Set();

  for (const event of allEvents) {
    switch (event.type) {
      case 'add':
        pausers.add(event.account);
        break;
      case 'remove':
        pausers.delete(event.account);
        break;
      case 'swap':
        pausers.delete(event.oldAccount);
        pausers.add(event.newAccount);
        break;
    }
  }

  return { pausers: Array.from(pausers), provider };
}

function formatNative(balanceWei) {
  const balance = ethers.formatEther(balanceWei);
  // Format to 6 decimal places, removing trailing zeros
  const formatted = parseFloat(balance).toFixed(6).replace(/\.?0+$/, '');
  return formatted === '' ? '0' : formatted;
}

async function printPausers(chainName, symbol, pausers, provider) {
  console.log(`\n${chainName} pausers:`);
  if (pausers === null) {
    console.log('  (not configured)');
  } else if (pausers.length === 0) {
    console.log('  (none)');
  } else {
    for (let i = 0; i < pausers.length; i++) {
      const pauser = pausers[i];
      const balance = await provider.getBalance(pauser);
      const balanceFormatted = formatNative(balance);
      console.log(`  ${i + 1}. ${pauser} (${balanceFormatted} ${symbol})`);
    }
  }
  if (pausers !== null) {
    console.log(`  Total: ${pausers.length} pauser(s)`);
  }
}

// Compare the pauser sets across every chain that returned results.
function compareAcrossChains(results) {
  const configured = Object.keys(results).filter((key) => results[key]?.pausers);
  if (configured.length < 2) return;

  console.log('\n' + '-'.repeat(50));

  // Map each pauser to the set of chains it currently appears on.
  const presence = new Map(); // pauser => Set(chain keys)
  for (const key of configured) {
    for (const pauser of results[key].pausers) {
      if (!presence.has(pauser)) presence.set(pauser, new Set());
      presence.get(pauser).add(key);
    }
  }

  const chainCount = configured.length;
  const identical = [...presence.values()].every((chains) => chains.size === chainCount);

  if (identical) {
    console.log(`Pausers are IDENTICAL across all ${chainCount} configured chains.`);
    return;
  }

  console.log('WARNING: Pausers DIFFER between chains!');
  for (const [pauser, chains] of presence) {
    if (chains.size === chainCount) continue;
    const present = configured.filter((key) => chains.has(key)).map((key) => CHAINS[key].name);
    const missing = configured.filter((key) => !chains.has(key)).map((key) => CHAINS[key].name);
    console.log(`\n  ${pauser}`);
    console.log(`    on:      ${present.join(', ')}`);
    console.log(`    missing: ${missing.join(', ')}`);
  }
}

async function main() {
  // Determine which chains are fully configured (both RPC and address present).
  const configuredKeys = Object.keys(CHAINS).filter(
    (key) => CHAINS[key].rpcUrl && CHAINS[key].pauserSetAddress
  );

  console.log(`Network: ${NETWORK}`);

  if (configuredKeys.length === 0) {
    console.error(`Error: No chains configured for ${NETWORK}. Please set environment variables in .env file:`);
    for (const key of Object.keys(CHAINS)) {
      console.error(`  ${CHAINS[key].name}: RPC + PAUSER_SET env vars (see .env.example)`);
    }
    process.exit(1);
  }

  const results = {};

  try {
    // Fetch pausers for every configured chain.
    for (const key of configuredKeys) {
      console.log(`\n[${CHAINS[key].name}]`);
      results[key] = await getPausersForChain(CHAINS[key]);
    }

    // Print summary
    console.log('\n' + '='.repeat(50));
    console.log('SUMMARY');
    console.log('='.repeat(50));

    for (const key of configuredKeys) {
      await printPausers(
        CHAINS[key].name,
        CHAINS[key].symbol,
        results[key]?.pausers ?? null,
        results[key]?.provider
      );
    }

    // Compare pausers across all configured chains.
    compareAcrossChains(results);
  } catch (error) {
    console.error(`\nError: ${error.message}`);
    process.exit(1);
  }
}

main();
