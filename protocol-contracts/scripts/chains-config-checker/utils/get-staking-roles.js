#!/usr/bin/env node

require('dotenv').config({ path: require('path').resolve(__dirname, '../.env') });
const path = require('path');
const { ethers } = require('ethers');
const { findDeploymentBlock } = require('./get-deployment-block');
const { queryEventsInChunks } = require('./lib/events');

// Both networks read RPC_ETHEREUM; point it at an Ethereum mainnet or Sepolia
// archive RPC depending on which --mainnet/--testnet flag you pass.
const NETWORKS = {
  mainnet: {
    label: 'Mainnet (Ethereum)',
    configFile: 'staking-addresses.json',
  },
  testnet: {
    label: 'Testnet (Sepolia)',
    configFile: 'staking-addresses-testnet.json',
  },
};

const ROLE_EVENT_ABI = [
  'event RoleGranted(bytes32 indexed role, address indexed account, address indexed sender)',
  'event RoleRevoked(bytes32 indexed role, address indexed account, address indexed sender)',
];

const OPERATOR_STAKING_ABI = [
  'function rewarder() view returns (address)',
];

const OPERATOR_REWARDER_ABI = [
  'function beneficiary() view returns (address)',
];

const ROLE_HASHES = {
  DEFAULT_ADMIN_ROLE: ethers.ZeroHash,
  MANAGER_ROLE: ethers.keccak256(ethers.toUtf8Bytes('MANAGER_ROLE')),
  ELIGIBLE_ACCOUNT_ROLE: ethers.keccak256(ethers.toUtf8Bytes('ELIGIBLE_ACCOUNT_ROLE')),
};

const ROLE_NAMES_BY_HASH = new Map(
  Object.entries(ROLE_HASHES).map(([name, hash]) => [hash, name])
);

function parseNetworkFlag(argv) {
  const flags = argv.filter((a) => a === '--mainnet' || a === '--testnet');
  if (flags.length > 1) {
    console.error('Error: pass only one of --mainnet or --testnet');
    process.exit(1);
  }
  return flags[0] === '--testnet' ? 'testnet' : 'mainnet';
}

// Build a set of all known OperatorStaking addresses (lowercased) for cross-referencing
function getAllOperatorStakingAddresses(stakingAddresses) {
  const addresses = new Set();
  for (const roleOperators of Object.values(stakingAddresses.operatorStaking)) {
    for (const addr of Object.values(roleOperators)) {
      addresses.add(addr.toLowerCase());
    }
  }
  return addresses;
}

// Reverse-lookup: address -> "Name (role)"
function getOperatorStakingLabel(stakingAddresses, address) {
  const lower = address.toLowerCase();
  for (const [role, operators] of Object.entries(stakingAddresses.operatorStaking)) {
    for (const [name, addr] of Object.entries(operators)) {
      if (addr.toLowerCase() === lower) return `${name} (${role})`;
    }
  }
  return null;
}

async function getProtocolStakingRoles(provider, rpcUrl, contractAddress) {
  const contract = new ethers.Contract(contractAddress, ROLE_EVENT_ABI, provider);

  console.log(`  Finding deployment block for ${contractAddress}...`);
  const fromBlock = await findDeploymentBlock(contractAddress, { rpcUrl, silent: true });
  const toBlock = await provider.getBlockNumber();
  console.log(`  Deployment block: ${fromBlock}, current block: ${toBlock}`);

  console.log('  Fetching role events...');
  const grantedEvents = await queryEventsInChunks(contract, contract.filters.RoleGranted(), fromBlock, toBlock, 'RoleGranted');
  const revokedEvents = await queryEventsInChunks(contract, contract.filters.RoleRevoked(), fromBlock, toBlock, 'RoleRevoked');

  const allEvents = [
    ...grantedEvents.map((e) => ({ type: 'grant', role: e.args.role, account: e.args.account, block: e.blockNumber, logIndex: e.index })),
    ...revokedEvents.map((e) => ({ type: 'revoke', role: e.args.role, account: e.args.account, block: e.blockNumber, logIndex: e.index })),
  ]
    .filter((e) => ROLE_NAMES_BY_HASH.has(e.role))
    .sort((a, b) => (a.block !== b.block ? a.block - b.block : a.logIndex - b.logIndex));

  const roleSets = new Map(Object.keys(ROLE_HASHES).map((name) => [name, new Set()]));

  for (const event of allEvents) {
    const roleName = ROLE_NAMES_BY_HASH.get(event.role);
    const set = roleSets.get(roleName);
    if (event.type === 'grant') {
      set.add(event.account);
    } else {
      set.delete(event.account);
    }
  }

  return Object.fromEntries([...roleSets.entries()].map(([name, set]) => [name, Array.from(set)]));
}

async function getOperatorStakingBeneficiary(provider, address) {
  const opStaking = new ethers.Contract(address, OPERATOR_STAKING_ABI, provider);
  const rewarderAddr = await opStaking.rewarder();
  const rewarder = new ethers.Contract(rewarderAddr, OPERATOR_REWARDER_ABI, provider);
  return rewarder.beneficiary();
}

function printRoles(roles, stakingAddresses, allOpStakingAddresses) {
  for (const [roleName, holders] of Object.entries(roles)) {
    console.log(`\n  ${roleName}:`);
    if (holders.length === 0) {
      console.log('    (none)');
      continue;
    }
    for (let i = 0; i < holders.length; i++) {
      const addr = holders[i];
      const label = getOperatorStakingLabel(stakingAddresses, addr);
      const suffix = label ? ` <- ${label}` : '';
      console.log(`    ${i + 1}. ${addr}${suffix}`);
    }

    // For ELIGIBLE_ACCOUNT_ROLE, check that every holder is a known OperatorStaking address
    if (roleName === 'ELIGIBLE_ACCOUNT_ROLE') {
      const unknown = holders.filter((a) => !allOpStakingAddresses.has(a.toLowerCase()));
      if (unknown.length > 0) {
        console.log('    WARNING: Unknown addresses in ELIGIBLE_ACCOUNT_ROLE:');
        for (const addr of unknown) {
          console.log(`      - ${addr}`);
        }
      } else {
        console.log('    All eligible accounts are known OperatorStaking addresses.');
      }
    }
  }
}

async function main() {
  const networkKey = parseNetworkFlag(process.argv.slice(2));
  const network = NETWORKS[networkKey];
  const stakingAddresses = require(path.resolve(__dirname, '..', network.configFile));

  const rpcUrl = process.env.RPC_ETHEREUM;
  if (!rpcUrl) {
    console.error('Error: RPC_ETHEREUM not configured');
    process.exit(1);
  }

  console.log(`\n### ${network.label} (${network.configFile}) ###`);

  const provider = new ethers.JsonRpcProvider(rpcUrl);
  const allOpStakingAddresses = getAllOperatorStakingAddresses(stakingAddresses);
  let hadError = false;

  // Part 1: ProtocolStaking roles
  console.log('\n=== ProtocolStaking Roles ===');

  for (const [role, address] of Object.entries(stakingAddresses.protocolStaking)) {
    console.log(`\n[ProtocolStaking - ${role.toUpperCase()}]`);
    console.log(`  Contract: ${address}`);
    try {
      const roles = await getProtocolStakingRoles(provider, rpcUrl, address);
      printRoles(roles, stakingAddresses, allOpStakingAddresses);
    } catch (error) {
      console.error(`  Error: ${error.message}`);
      hadError = true;
    }
  }

  // Part 2: OperatorStaking beneficiaries
  console.log('\n=== OperatorStaking Beneficiaries ===');

  for (const [role, operators] of Object.entries(stakingAddresses.operatorStaking)) {
    console.log(`\n[${role.toUpperCase()}]`);

    for (const [name, address] of Object.entries(operators)) {
      try {
        const beneficiary = await getOperatorStakingBeneficiary(provider, address);
        console.log(`  ${name.padEnd(15)} : ${beneficiary}`);
      } catch (error) {
        console.error(`  ${name.padEnd(15)} : Error - ${error.message}`);
        hadError = true;
      }
    }
  }

  if (hadError) {
    process.exit(1);
  }
}

main();
