'use strict'

// ---------------------------------------------------------------------------
// Registry of cross-chain governance DESTINATIONS.
//
// The Protocol DAO on Ethereum sends proposals to EVM destination chains via
// LayerZero, one `GovernanceOAppSender` (on Ethereum / Sepolia) per
// destination, each paired with a `GovernanceOAppReceiver` + `AdminModule` +
// local multisig (Safe) on the destination chain.
//
// This file holds ONLY the fields the scripts actually consume. Everything else
// about a destination (LayerZero EID, EndpointV2, block explorer, environment,
// receiver/module addresses, …) lives in the protocol-registry repo (the
// source of truth) and the docs/governance runbooks — not here.
//
// Fields:
//   - displayName         : human-readable label, shown in script output.
//   - oappSender          : the GovernanceOAppSender address on the SOURCE
//                           chain (Ethereum mainnet or Sepolia). The fill
//                           script sets the proposal `to` to this.
//   - destinationExecutor : the local multisig (Safe) on the DESTINATION chain
//                           through which the AdminModule executes calls. The
//                           fill script uses it as the (unsigned) `from` when
//                           estimating lzReceive gas via eth_estimateGas.
//   - rpcEnvVar           : name of the .env variable holding the DESTINATION
//                           chain RPC URL (queried for gas estimation).
//   - defaultRpc          : fallback RPC used when rpcEnvVar is unset.
//
// To add a new EVM destination, append an entry here and an RPC env var in
// .env.example. No script change is required. Addresses are mirrored from
// protocol-registry (https://github.com/zama-ai/protocol-registry) — always
// re-verify there rather than trusting this file from memory. Note:
// protocol-registry only publishes mainnet/testnet; devnet addresses come from
// the devnet deployment.
// ---------------------------------------------------------------------------

const DESTINATIONS = {
  'gateway-mainnet': {
    displayName: 'Zama Gateway (mainnet)',
    // GovernanceOAppSender on Ethereum mainnet -> Gateway mainnet.
    oappSender: '0x1c5D750D18917064915901048cdFb2dB815e0910',
    // Gateway multisig (owns the receiver; AdminModule executes through it).
    destinationExecutor: '0x5f0F86BcEad6976711C9B131bCa5D30E767fe2bE',
    rpcEnvVar: 'RPC_GATEWAY_MAINNET',
    defaultRpc: 'https://rpc.mainnet.zama.org',
  },
  'gateway-testnet': {
    displayName: 'Zama Gateway (testnet)',
    // GovernanceOAppSender on Sepolia -> Gateway testnet.
    oappSender: '0x909692c2f4979ca3fa11B5859d499308A1ec4932',
    // Gateway testnet multisig.
    destinationExecutor: '0x3241b3A4036a356c5D7e36a432Da2B8e5739D9c9',
    rpcEnvVar: 'RPC_GATEWAY_TESTNET',
    defaultRpc: 'https://rpc-zama-testnet-0.t.conduit.xyz',
  },
  'gateway-devnet': {
    displayName: 'Zama Gateway (devnet)',
    // GovernanceOAppSender on Sepolia -> Gateway devnet (same chain as testnet).
    oappSender: '0x369CDAD997981C06aa02f82b74564C1F4A4D36ae',
    // Gateway devnet multisig.
    destinationExecutor: '0xb8E03De46F3539aEA7FEb072eEAE6A8f4A14913B',
    rpcEnvVar: 'RPC_GATEWAY_DEVNET',
    defaultRpc: 'https://rpc-zama-testnet-0.t.conduit.xyz',
  },
  'polygon-amoy-testnet': {
    displayName: 'Polygon Amoy (testnet)',
    // GovernanceOAppSender on Sepolia -> Polygon Amoy.
    oappSender: '0xe57ea2f14f3051296d3965Bae8caAF86acdd6050',
    // Amoy multisig.
    destinationExecutor: '0xF0b1FE5DecfFe400fb141BBEAF9B181bCF76E3Cb',
    rpcEnvVar: 'RPC_POLYGON_AMOY_TESTNET',
    defaultRpc: 'https://rpc-amoy.polygon.technology',
  },
  'polygon-amoy-devnet': {
    displayName: 'Polygon Amoy (devnet)',
    // GovernanceOAppSender on Sepolia -> Polygon Amoy (devnet).
    oappSender: '0xbB0D1F2982cb1073e934695c78ADC45dE46d873a',
    // Amoy devnet multisig.
    destinationExecutor: '0xb8E03De46F3539aEA7FEb072eEAE6A8f4A14913B',
    rpcEnvVar: 'RPC_POLYGON_AMOY_DEVNET',
    defaultRpc: 'https://rpc-amoy.polygon.technology',
  },
  'polygon-mainnet': {
    displayName: 'Polygon Mainnet',
    // GovernanceOAppSender on Ethereum -> Polygon Mainnet.
    oappSender: '0x8d3FbB9FB34E5dF40A293C2aA4ea49634e340935',
    // Polygon mainnet multisig.
    destinationExecutor: '0xeF0645fE2f53aE04d750d5C578BbDAd6cE5Afe55',
    rpcEnvVar: 'RPC_POLYGON_MAINNET',
    defaultRpc: 'https://rpc-mainnet.matic.quiknode.pro',
  },
}

function listDestinationIds() {
  return Object.keys(DESTINATIONS)
}

function resolveDestination(id) {
  const dest = DESTINATIONS[id]
  if (!dest) {
    throw new Error(
      `Unknown destination "${id}". Known destinations: ${listDestinationIds().join(', ')}.`
    )
  }
  return { id, ...dest }
}

module.exports = {
  DESTINATIONS,
  listDestinationIds,
  resolveDestination,
}

// `node destinations.js` prints the registry (handy for docs / a quick check).
if (require.main === module) {
  for (const [id, d] of Object.entries(DESTINATIONS)) {
    console.log(`${id}`)
    console.log(`  displayName:         ${d.displayName}`)
    console.log(`  oappSender (source): ${d.oappSender}`)
    console.log(`  destinationExecutor: ${d.destinationExecutor}`)
    console.log(`  rpcEnvVar:           ${d.rpcEnvVar} (default: ${d.defaultRpc})`)
    console.log('')
  }
}
