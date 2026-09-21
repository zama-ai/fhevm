#!/usr/bin/env node

require('dotenv').config({ path: require('path').resolve(__dirname, '../.env') });
const { ethers } = require('ethers');
const { findDeploymentBlock, isValidAddress } = require('./get-deployment-block');
const { queryEventsInChunks } = require('./lib/events');

const ETHEREUM_RPC_URL = process.env.RPC_ETHEREUM;
const ZAMA_TOKEN_ERC20_ETHEREUM = process.env.ZAMA_TOKEN_ERC20_ETHEREUM;

const TOKEN_ROLE_ABI = [
  'function DEFAULT_ADMIN_ROLE() view returns (bytes32)',
  'function MINTER_ROLE() view returns (bytes32)',
  'function MINTING_PAUSER_ROLE() view returns (bytes32)',
  'event RoleGranted(bytes32 indexed role, address indexed account, address indexed sender)',
  'event RoleRevoked(bytes32 indexed role, address indexed account, address indexed sender)',
];

async function getRolesForEthereum() {
  if (!ETHEREUM_RPC_URL) {
    throw new Error('RPC_ETHEREUM is not configured');
  }

  if (!ZAMA_TOKEN_ERC20_ETHEREUM) {
    throw new Error('ZAMA_TOKEN_ERC20_ETHEREUM is not configured');
  }

  if (!isValidAddress(ZAMA_TOKEN_ERC20_ETHEREUM)) {
    throw new Error('Invalid ZAMA_TOKEN_ERC20_ETHEREUM address format');
  }

  console.log('\n[Ethereum]');

  const provider = new ethers.JsonRpcProvider(ETHEREUM_RPC_URL);
  const contract = new ethers.Contract(ZAMA_TOKEN_ERC20_ETHEREUM, TOKEN_ROLE_ABI, provider);

  console.log(`  Finding deployment block for ${ZAMA_TOKEN_ERC20_ETHEREUM}...`);
  const fromBlock = await findDeploymentBlock(ZAMA_TOKEN_ERC20_ETHEREUM, { rpcUrl: ETHEREUM_RPC_URL, silent: true });
  console.log(`  Deployment block: ${fromBlock}`);

  const toBlock = await provider.getBlockNumber();
  console.log(`  Current block: ${toBlock}`);

  console.log('  Fetching role events...');

  const [defaultAdminRole, minterRole, mintingPauserRole] = await Promise.all([
    contract.DEFAULT_ADMIN_ROLE(),
    contract.MINTER_ROLE(),
    contract.MINTING_PAUSER_ROLE(),
  ]);

  const rolesById = new Map([
    [defaultAdminRole, 'DEFAULT_ADMIN_ROLE'],
    [minterRole, 'MINTER_ROLE'],
    [mintingPauserRole, 'MINTING_PAUSER_ROLE'],
  ]);

  const roleGrantedFilter = contract.filters.RoleGranted();
  const roleRevokedFilter = contract.filters.RoleRevoked();

  const grantedEvents = await queryEventsInChunks(
    contract,
    roleGrantedFilter,
    fromBlock,
    toBlock,
    'RoleGranted'
  );
  const revokedEvents = await queryEventsInChunks(
    contract,
    roleRevokedFilter,
    fromBlock,
    toBlock,
    'RoleRevoked'
  );

  const allEvents = [
    ...grantedEvents.map((e) => ({
      type: 'grant',
      role: e.args.role,
      account: e.args.account,
      block: e.blockNumber,
      logIndex: e.index,
    })),
    ...revokedEvents.map((e) => ({
      type: 'revoke',
      role: e.args.role,
      account: e.args.account,
      block: e.blockNumber,
      logIndex: e.index,
    })),
  ]
    .filter((event) => rolesById.has(event.role))
    .sort((a, b) => {
      if (a.block !== b.block) return a.block - b.block;
      return a.logIndex - b.logIndex;
    });

  const roleNames = ['DEFAULT_ADMIN_ROLE', 'MINTER_ROLE', 'MINTING_PAUSER_ROLE'];
  const roleSets = new Map(roleNames.map((name) => [name, new Set()]));
  const roleStats = new Map(roleNames.map((name) => [name, { granted: 0, revoked: 0 }]));

  for (const event of allEvents) {
    const roleName = rolesById.get(event.role);
    if (!roleName) continue;

    const set = roleSets.get(roleName);
    const stats = roleStats.get(roleName);

    if (event.type === 'grant') {
      set.add(event.account);
      stats.granted += 1;
    } else if (event.type === 'revoke') {
      set.delete(event.account);
      stats.revoked += 1;
    }
  }

  const rolesByName = {};
  for (const [name, set] of roleSets.entries()) {
    rolesByName[name] = Array.from(set);
  }

  const totalEvents = {
    granted: grantedEvents.length,
    revoked: revokedEvents.length,
  };

  return { rolesByName, provider, roleStats, totalEvents };
}

async function printRoles(rolesByName, provider, roleStats, totalEvents) {
  console.log('\n' + '='.repeat(50));
  console.log('CURRENT ROLE HOLDERS');
  console.log('='.repeat(50));

  const orderedRoles = ['DEFAULT_ADMIN_ROLE', 'MINTER_ROLE', 'MINTING_PAUSER_ROLE'];

  for (const roleName of orderedRoles) {
    const addresses = rolesByName[roleName] || [];
    console.log(`\n${roleName}:`);

    if (addresses.length === 0) {
      console.log('  (none)');
    } else {
      for (let i = 0; i < addresses.length; i++) {
        const addr = addresses[i];
        console.log(`  ${i + 1}. ${addr} `);
      }
    }

    const stats = roleStats.get(roleName);
    if (stats) {
      console.log(`  Events: ${stats.granted} granted, ${stats.revoked} revoked`);
    }
    console.log(`  Total: ${addresses.length} address(es)`);
  }

  if (totalEvents) {
    console.log('\n' + '-'.repeat(50));
    console.log(`Total RoleGranted events: ${totalEvents.granted}`);
    console.log(`Total RoleRevoked events: ${totalEvents.revoked}`);
  }
}

async function main() {
  try {
    const { rolesByName, provider, roleStats, totalEvents } = await getRolesForEthereum();
    await printRoles(rolesByName, provider, roleStats, totalEvents);
  } catch (error) {
    console.error(`\nError: ${error.message}`);
    process.exit(1);
  }
}

main();

