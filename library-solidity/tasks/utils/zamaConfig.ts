import { getAddress } from 'ethers';
import fs from 'fs';
import path from 'path';

const ZAMA_CONFIG_SOL_PATH = path.resolve(__dirname, '../../config/ZamaConfig.sol');

// Address fields of one CoprocessorConfig in ZamaConfig.sol, e.g. { ACLAddress, CoprocessorAddress, KMSVerifierAddress },
// as checksummed addresses.
export type ZamaConfigAddresses = Readonly<Record<string, string>>;

// All CoprocessorConfig addresses of ZamaConfig.sol, keyed by chain id as a decimal string (JSON-compatible), e.g.
// { "1": { "ACLAddress": "0x…", … }, "31337": { … } }.
export type ZamaConfigAddressesByChainId = Readonly<Record<string, ZamaConfigAddresses>>;

// Parsed once, kept in memory only.
let cachedAddressesByChainId: ZamaConfigAddressesByChainId | undefined;

// Returns every address of config/ZamaConfig.sol, per chain id routed by getCoprocessorConfig().
// Throws if ZamaConfig.sol no longer has the expected structure.
export function getZamaConfigAddressesByChainId(): ZamaConfigAddressesByChainId {
  cachedAddressesByChainId ??= parseZamaConfig(fs.readFileSync(ZAMA_CONFIG_SOL_PATH, 'utf8'));
  return cachedAddressesByChainId;
}

// Returns the addresses ZamaConfig.sol hardcodes for `chainId`, or undefined if ZamaConfig.sol does not support it.
export function getZamaConfigAddresses(chainId: bigint): ZamaConfigAddresses | undefined {
  const addressesByChainId = getZamaConfigAddressesByChainId();
  const key = chainId.toString();
  return Object.prototype.hasOwnProperty.call(addressesByChainId, key) ? addressesByChainId[key] : undefined;
}

function parseZamaConfig(source: string): ZamaConfigAddressesByChainId {
  // getCoprocessorConfig() routes each chain id to a private config function:
  //   if (block.chainid == 1) { config = _getEthereumConfig(); } else if (...) ...
  const routing = /function getCoprocessorConfig\(\)[\s\S]*?revert ZamaProtocolUnsupported\(\);/.exec(source);
  if (routing === null) {
    throw new Error(`${ZAMA_CONFIG_SOL_PATH}: cannot find getCoprocessorConfig()`);
  }

  const addressesByChainId: Record<string, ZamaConfigAddresses> = {};
  for (const [, chainId, configFunction] of routing[0].matchAll(
    /block\.chainid == (\d+)\) \{\s*config = (_\w+)\(\);/g,
  )) {
    addressesByChainId[chainId] = parseConfigFunctionAddresses(source, configFunction);
  }
  if (Object.keys(addressesByChainId).length === 0) {
    throw new Error(`${ZAMA_CONFIG_SOL_PATH}: cannot parse the chain ids routed by getCoprocessorConfig()`);
  }
  return Object.freeze(addressesByChainId);
}

// Returns the `Name: 0x…` fields of the CoprocessorConfig({ ... }) returned by `configFunction`.
function parseConfigFunctionAddresses(source: string, configFunction: string): ZamaConfigAddresses {
  const body = new RegExp(
    `function ${configFunction}\\(\\) private pure returns \\(CoprocessorConfig memory\\) \\{[\\s\\S]*?CoprocessorConfig\\(\\{([\\s\\S]*?)\\}\\)`,
  ).exec(source);
  if (body === null) {
    throw new Error(`${ZAMA_CONFIG_SOL_PATH}: cannot find the CoprocessorConfig returned by ${configFunction}()`);
  }

  const addresses: Record<string, string> = {};
  for (const [, name, address] of body[1].matchAll(/(\w+):\s*(0x[0-9a-fA-F]{40})\b/g)) {
    addresses[name] = getAddress(address);
  }
  if (!('ACLAddress' in addresses)) {
    throw new Error(`${ZAMA_CONFIG_SOL_PATH}: no ACLAddress in the CoprocessorConfig returned by ${configFunction}()`);
  }
  return Object.freeze(addresses);
}
