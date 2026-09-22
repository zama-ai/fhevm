#!/usr/bin/env node

require('dotenv').config({ path: require('path').resolve(__dirname, '../.env') });
const { isValidAddress } = require('./get-deployment-block');
const { getOwnerAndDelegateForChain } = require('./lib/owner-delegate');

// Chain configurations from environment (OFT/OFTAdapter per chain)
const CHAINS = {
  ethereumOftAdapter: {
    name: 'Ethereum OFT Adapter',
    rpcEnv: 'RPC_ETHEREUM',
    addrEnv: 'ZAMA_OFT_ADAPTER_ETHEREUM',
  },
  gatewayOft: {
    name: 'Gateway OFT',
    rpcEnv: 'RPC_GATEWAY',
    addrEnv: 'ZAMA_OFT_GATEWAY',
  },
  bscOft: {
    name: 'BSC OFT',
    rpcEnv: 'RPC_BSC',
    addrEnv: 'ZAMA_OFT_BSC',
  },
  hyperEvmOft: {
    name: 'HyperEVM OFT',
    rpcEnv: 'RPC_HYPEREVM',
    addrEnv: 'ZAMA_OFT_HYPEREVM',
  },
};

function buildChainConfigs() {
  const configs = [];
  for (const [key, chain] of Object.entries(CHAINS)) {
    const rpcUrl = process.env[chain.rpcEnv];
    const contractAddress = process.env[chain.addrEnv];
    configs.push({
      key,
      name: chain.name,
      rpcUrl,
      contractAddress,
    });
  }
  return configs;
}

async function printOwnerAndDelegate(info) {
  const { name, contractAddress, owner, delegate } = info;

  console.log(`\n[${name}]`);
  console.log(`  Adapter/OFT address : ${contractAddress}`);
  console.log(`  Owner               : ${owner}`);
  console.log(`  Delegate            : ${delegate}`);
}

async function main() {
  const configs = buildChainConfigs();
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
    console.error('Error: No chains configured. Set RPC and contract address env vars in .env');
    process.exit(1);
  }

  let hadError = false;
  const equalityResults = []; // { name, equal }

  console.log('\n=== EVM OFT ===');
  for (const chainConfig of toRun) {
    const resolved = {
      name: chainConfig.name,
      rpcUrl: chainConfig.rpcUrl,
      contractAddress: chainConfig.contractAddress,
    };
    try {
      const info = await getOwnerAndDelegateForChain(resolved);
      await printOwnerAndDelegate(info);
      equalityResults.push({ name: chainConfig.name, equal: info.equal });
    } catch (error) {
      console.error(`\n[${chainConfig.name}] Error: ${error.message}`);
      hadError = true;
    }
  }

  if (hadError) {
    process.exit(1);
  }

  const failed = equalityResults.filter((r) => !r.equal);

  if (failed.length > 0) {
    for (const { name } of failed) {
      console.error(`Owner and Delegate are NOT IDENTICAL on ${name}`);
    }
    process.exit(1);
  } else {
    console.log('\nOwner and Delegate should be IDENTICAL on EVM chains,');
    console.log('and it should be the Zama DAO or a Safe multisig wallet owned by Zama FB_i operators');
  }
}

main();
