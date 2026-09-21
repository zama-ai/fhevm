#!/usr/bin/env node

require('dotenv').config({ path: require('path').resolve(__dirname, '../.env') });
const { ethers } = require('ethers');
const { isValidAddress } = require('./get-deployment-block');
const { OWNABLE2STEP_ABI, ZERO_ADDRESS, getOwnerAndPending } = require('./lib/owner-pending');

const CONTRACTS = {
  ethereumAcl: {
    name: 'Ethereum ACL',
    rpcEnv: 'RPC_ETHEREUM',
    addrEnv: 'ZAMA_ACL_ETHEREUM',
  },
  gatewayConfig: {
    name: 'Gateway GatewayConfig',
    rpcEnv: 'RPC_GATEWAY',
    addrEnv: 'ZAMA_GATEWAY_CONFIG_GATEWAY',
  },
};

function buildConfigs() {
  return Object.entries(CONTRACTS).map(([key, contract]) => ({
    key,
    name: contract.name,
    rpcUrl: process.env[contract.rpcEnv],
    contractAddress: process.env[contract.addrEnv],
  }));
}

async function getOwnerInfo(config) {
  const { name, rpcUrl, contractAddress } = config;

  if (!rpcUrl) throw new Error(`[${name}] RPC URL is not configured`);
  if (!contractAddress) throw new Error(`[${name}] Contract address is not configured`);
  if (!isValidAddress(contractAddress)) throw new Error(`[${name}] Invalid contract address format`);

  const provider = new ethers.JsonRpcProvider(rpcUrl);
  const contract = new ethers.Contract(contractAddress, OWNABLE2STEP_ABI, provider);

  const { owner, pendingOwner } = await getOwnerAndPending(contract);

  return { name, contractAddress, owner, pendingOwner };
}

function printOwnerInfo(info) {
  const { name, contractAddress, owner, pendingOwner } = info;
  console.log(`\n[${name}]`);
  console.log(`  Contract address : ${contractAddress}`);
  console.log(`  Owner            : ${owner}`);
  if (pendingOwner !== ZERO_ADDRESS) {
    console.log(`  Pending owner    : ${pendingOwner}`);
    console.log(`  WARNING: ownership handover in progress`);
  }
}

async function main() {
  const configs = buildConfigs();
  const toRun = configs.filter((c) => c.rpcUrl && c.contractAddress && isValidAddress(c.contractAddress));
  const toSkip = configs.filter((c) => !c.rpcUrl || !c.contractAddress || !isValidAddress(c.contractAddress || ''));

  for (const c of toSkip) {
    if (!c.rpcUrl) {
      console.log(`  Skipping ${c.name}: RPC not configured`);
    } else if (!c.contractAddress) {
      console.log(`  Skipping ${c.name}: Contract address not configured`);
    } else {
      console.log(`  Skipping ${c.name}: Invalid address format`);
    }
  }

  if (toRun.length === 0) {
    console.error('Error: No contracts configured. Set RPC and contract address env vars in .env');
    process.exit(1);
  }

  let hadError = false;
  const pending = [];

  console.log('\n=== Protocol Contract Owners ===');
  for (const config of toRun) {
    try {
      const info = await getOwnerInfo(config);
      printOwnerInfo(info);
      if (info.pendingOwner !== ZERO_ADDRESS) pending.push(info.name);
    } catch (error) {
      console.error(`\n[${config.name}] Error: ${error.message}`);
      hadError = true;
    }
  }

  if (hadError) process.exit(1);

  if (pending.length > 0) {
    console.warn(`\nWARNING: ${pending.length} contract(s) have a pending owner: ${pending.join(', ')}`);
  }
}

main();
