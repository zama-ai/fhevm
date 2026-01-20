#!/usr/bin/env node

const { ethers } = require('ethers');

const DEFAULT_RPC_URL = 'https://ethereum-rpc.publicnode.com';

async function findDeploymentBlock(address, options = {}) {
  const rpcUrl = options.rpcUrl || DEFAULT_RPC_URL;
  const silent = options.silent || false;

  const provider = new ethers.JsonRpcProvider(rpcUrl);

  const currentBlock = await provider.getBlockNumber();
  if (!silent) console.log(`Current block: ${currentBlock}`);

  // Check if contract exists at current block
  const currentCode = await provider.getCode(address, currentBlock);
  if (currentCode === '0x') {
    throw new Error('No contract found at this address');
  }

  let low = 0;
  let high = currentBlock;

  // Binary search for the first block with code
  while (low < high) {
    const mid = Math.floor((low + high) / 2);
    const code = await provider.getCode(address, mid);

    if (code === '0x') {
      // No code at this block, deployment is after
      low = mid + 1;
    } else {
      // Code exists, deployment is at or before this block
      high = mid;
    }

    if (!silent) process.stdout.write(`\rSearching... block range: ${low} - ${high}`);
  }

  if (!silent) console.log(); // New line after progress
  return low;
}

function isValidAddress(address) {
  return ethers.isAddress(address);
}

async function main() {
  const address = process.argv[2];

  if (!address) {
    console.error('Usage: node get-deployment-block.js <contract-address>');
    process.exit(1);
  }

  if (!isValidAddress(address)) {
    console.error('Invalid Ethereum address format');
    process.exit(1);
  }

  try {
    console.log(`Finding deployment block for ${address}...`);
    const deploymentBlock = await findDeploymentBlock(address);
    console.log(`Deployment block: ${deploymentBlock}`);
  } catch (error) {
    console.error(`Error: ${error.message}`);
    process.exit(1);
  }
}

// Export for use as module
module.exports = { findDeploymentBlock, isValidAddress, DEFAULT_RPC_URL };

// Run main only if executed directly
if (require.main === module) {
  main();
}
