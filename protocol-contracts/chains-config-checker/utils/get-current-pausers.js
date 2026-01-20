#!/usr/bin/env node

require('dotenv').config({ path: require('path').resolve(__dirname, '../.env') });
const { ethers } = require('ethers');
const { findDeploymentBlock, isValidAddress } = require('./get-deployment-block');

// PauserSet event signatures
const PAUSER_SET_ABI = [
  'event AddPauser(address account)',
  'event RemovePauser(address account)',
  'event SwapPauser(address oldAccount, address newAccount)',
];

// Max block range for eth_getLogs (most providers limit to 50k)
const MAX_BLOCK_RANGE = 49999;

// Chain configurations from environment
const CHAINS = {
  ethereum: {
    name: 'Ethereum',
    rpcUrl: process.env.RPC_ETHEREUM,
    pauserSetAddress: process.env.PAUSER_SET_ETHEREUM,
  },
  gateway: {
    name: 'Gateway',
    rpcUrl: process.env.RPC_GATEWAY,
    pauserSetAddress: process.env.PAUSER_SET_GATEWAY,
  },
};

async function queryEventsInChunks(contract, filter, fromBlock, toBlock, label) {
  const events = [];
  let currentFrom = fromBlock;

  while (currentFrom <= toBlock) {
    const currentTo = Math.min(currentFrom + MAX_BLOCK_RANGE, toBlock);
    const progress = Math.round(((currentFrom - fromBlock) / (toBlock - fromBlock)) * 100) || 0;
    process.stdout.write(`\r    ${label}: ${progress}% (block ${currentFrom})...`);

    const chunk = await contract.queryFilter(filter, currentFrom, currentTo);
    events.push(...chunk);

    currentFrom = currentTo + 1;
  }

  console.log(`\r    ${label}: 100% - found ${events.length} events`);
  return events;
}

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

  // Find deployment block
  console.log(`  Finding deployment block for ${pauserSetAddress}...`);
  const fromBlock = await findDeploymentBlock(pauserSetAddress, { rpcUrl, silent: true });
  console.log(`  Deployment block: ${fromBlock}`);

  // Get current block
  const toBlock = await provider.getBlockNumber();
  console.log(`  Current block: ${toBlock}`);

  // Query all events from deployment to latest (in chunks, sequentially for clean output)
  console.log('  Fetching pauser events...');

  const addEvents = await queryEventsInChunks(contract, contract.filters.AddPauser(), fromBlock, toBlock, 'AddPauser');
  const removeEvents = await queryEventsInChunks(contract, contract.filters.RemovePauser(), fromBlock, toBlock, 'RemovePauser');
  const swapEvents = await queryEventsInChunks(contract, contract.filters.SwapPauser(), fromBlock, toBlock, 'SwapPauser');

  // Combine and sort all events by block number and log index
  const allEvents = [
    ...addEvents.map((e) => ({ type: 'add', account: e.args.account, block: e.blockNumber, logIndex: e.index })),
    ...removeEvents.map((e) => ({ type: 'remove', account: e.args.account, block: e.blockNumber, logIndex: e.index })),
    ...swapEvents.map((e) => ({
      type: 'swap',
      oldAccount: e.args.oldAccount,
      newAccount: e.args.newAccount,
      block: e.blockNumber,
      logIndex: e.index,
    })),
  ].sort((a, b) => {
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

  return Array.from(pausers);
}

function printPausers(chainName, pausers) {
  console.log(`\n${chainName} pausers:`);
  if (pausers === null) {
    console.log('  (not configured)');
  } else if (pausers.length === 0) {
    console.log('  (none)');
  } else {
    pausers.forEach((pauser, i) => {
      console.log(`  ${i + 1}. ${pauser}`);
    });
  }
  if (pausers !== null) {
    console.log(`  Total: ${pausers.length} pauser(s)`);
  }
}

async function main() {
  // Check if any chain is configured
  const hasEthereumConfig = CHAINS.ethereum.rpcUrl && CHAINS.ethereum.pauserSetAddress;
  const hasGatewayConfig = CHAINS.gateway.rpcUrl && CHAINS.gateway.pauserSetAddress;

  if (!hasEthereumConfig && !hasGatewayConfig) {
    console.error('Error: No chains configured. Please set environment variables in .env file:');
    console.error('  RPC_ETHEREUM, PAUSER_SET_ETHEREUM');
    console.error('  RPC_GATEWAY, PAUSER_SET_GATEWAY');
    process.exit(1);
  }

  const results = {};

  try {
    // Process Ethereum
    if (hasEthereumConfig) {
      console.log('\n[Ethereum]');
      results.ethereum = await getPausersForChain(CHAINS.ethereum);
    }

    // Process Gateway
    if (hasGatewayConfig) {
      console.log('\n[Gateway]');
      results.gateway = await getPausersForChain(CHAINS.gateway);
    }

    // Print summary
    console.log('\n' + '='.repeat(50));
    console.log('SUMMARY');
    console.log('='.repeat(50));

    if (hasEthereumConfig) {
      printPausers('Ethereum', results.ethereum);
    }

    if (hasGatewayConfig) {
      printPausers('Gateway', results.gateway);
    }

    // Check if pausers match across chains
    if (results.ethereum && results.gateway) {
      const ethSet = new Set(results.ethereum);
      const gwSet = new Set(results.gateway);
      const match = ethSet.size === gwSet.size && [...ethSet].every((p) => gwSet.has(p));

      console.log('\n' + '-'.repeat(50));
      if (match) {
        console.log('Pausers are IDENTICAL on both chains.');
      } else {
        console.log('WARNING: Pausers DIFFER between chains!');

        const onlyEth = results.ethereum.filter((p) => !gwSet.has(p));
        const onlyGw = results.gateway.filter((p) => !ethSet.has(p));

        if (onlyEth.length > 0) {
          console.log('\n  Only on Ethereum:');
          onlyEth.forEach((p) => console.log(`    - ${p}`));
        }
        if (onlyGw.length > 0) {
          console.log('\n  Only on Gateway:');
          onlyGw.forEach((p) => console.log(`    - ${p}`));
        }
      }
    }
  } catch (error) {
    console.error(`\nError: ${error.message}`);
    process.exit(1);
  }
}

main();
