#!/usr/bin/env node

require('dotenv').config({ path: require('path').resolve(__dirname, '../.env') });
const { ethers } = require('ethers');
const { isValidAddress } = require('./get-deployment-block');
const { ZERO_ADDRESS, getOwnerAndPending } = require('./lib/owner-pending');

const REGISTRY_ABI = [
  'function owner() view returns (address)',
  'function pendingOwner() view returns (address)',
  'function getTokenConfidentialTokenPairsLength() view returns (uint256)',
  'function getTokenConfidentialTokenPair(uint256) view returns (tuple(address tokenAddress, address confidentialTokenAddress, bool isValid))',
];

const WRAPPER_ABI = [
  'function owner() view returns (address)',
  'function pendingOwner() view returns (address)',
];

const ERC20_SYMBOL_ABI = ['function symbol() view returns (string)'];

function printEntry(label, address, owner, pendingOwner) {
  console.log(`\n[${label}]`);
  console.log(`  Address       : ${address}`);
  console.log(`  Owner         : ${owner}`);
  if (pendingOwner !== ZERO_ADDRESS) {
    console.log(`  Pending owner : ${pendingOwner}`);
    console.log(`  WARNING: ownership handover in progress`);
  }
}

async function main() {
  const rpcUrl = process.env.RPC_ETHEREUM;
  const registryAddress = process.env.ZAMA_WRAPPERS_REGISTRY_ETHEREUM;

  if (!rpcUrl) {
    console.error('Error: RPC_ETHEREUM is not configured');
    process.exit(1);
  }
  if (!registryAddress || !isValidAddress(registryAddress)) {
    console.error('Error: ZAMA_WRAPPERS_REGISTRY_ETHEREUM is missing or invalid');
    process.exit(1);
  }

  const provider = new ethers.JsonRpcProvider(rpcUrl);
  const registry = new ethers.Contract(registryAddress, REGISTRY_ABI, provider);

  console.log('\n=== Wrappers Registry & Confidential Wrappers ===');

  const { owner: registryOwner, pendingOwner: registryPending } = await getOwnerAndPending(registry);
  printEntry('Wrappers Registry', registryAddress, registryOwner, registryPending);

  const length = Number(await registry.getTokenConfidentialTokenPairsLength());
  if (length === 0) {
    console.log('\nNo confidential wrappers registered.');
    return;
  }

  const pairs = await Promise.all(
    Array.from({ length }, (_, i) => registry.getTokenConfidentialTokenPair(i))
  );

  const mismatches = [];
  const pendingHandovers = registryPending !== ZERO_ADDRESS ? [registryAddress] : [];

  for (const pair of pairs) {
    const wrapperAddress = pair.confidentialTokenAddress;
    const wrapper = new ethers.Contract(wrapperAddress, WRAPPER_ABI, provider);
    const underlying = new ethers.Contract(pair.tokenAddress, ERC20_SYMBOL_ABI, provider);
    const [{ owner, pendingOwner }, underlyingSymbol] = await Promise.all([
      getOwnerAndPending(wrapper),
      underlying.symbol(),
    ]);
    const label = `Confidential wrapper (underlying ${underlyingSymbol})`;
    printEntry(label, wrapperAddress, owner, pendingOwner);

    if (owner !== registryOwner) {
      mismatches.push({ wrapperAddress, owner });
    }
    if (pendingOwner !== ZERO_ADDRESS) {
      pendingHandovers.push(wrapperAddress);
    }
  }

  console.log('\n--------------------------------------------------');
  console.log(`Registry owner : ${registryOwner}`);
  console.log(`Wrappers       : ${pairs.length}`);

  if (mismatches.length > 0) {
    console.log(`\nNOTE: ${mismatches.length} wrapper(s) have an owner different from the registry owner:`);
    for (const m of mismatches) {
      console.log(`  ${m.wrapperAddress} -> ${m.owner}`);
    }
  } else {
    console.log('\nAll wrapper owners are IDENTICAL to the registry owner.');
  }

  if (pendingHandovers.length > 0) {
    console.warn(`\nWARNING: ${pendingHandovers.length} contract(s) have a pending owner:`);
    for (const a of pendingHandovers) {
      console.warn(`  ${a}`);
    }
  }
}

main().catch((error) => {
  console.error(`\nError: ${error.message}`);
  process.exit(1);
});
