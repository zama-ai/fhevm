// Checks that the host contract addresses used by the tests (fhevmTemp/addresses/FHEVMHostAddresses.sol) are the ones
// config/ZamaConfig.sol hardcodes for the local chain: otherwise contracts using ZamaConfig would point to other
// contracts than the deployed host contracts.
//
// Ported from the HH2 tasks (tasks/utils/zamaConfig.ts and _assertMatchesZamaConfig in tasks/taskDeploy.ts), which
// ran the same check at deployment time.
import { getAddress } from 'ethers';
import fs from 'node:fs';
import path from 'node:path';

const ZAMA_CONFIG_SOL = path.resolve(import.meta.dirname, '../config/ZamaConfig.sol');

/** CoprocessorConfig fields of ZamaConfig.sol, and the FHEVMHostAddresses.sol constant that must hold the same address. */
const ZAMA_CONFIG_FIELD_TO_HOST_CONSTANT = {
  ACLAddress: 'aclAdd',
  CoprocessorAddress: 'fhevmExecutorAdd',
  KMSVerifierAddress: 'kmsVerifierAdd',
} as const;

/**
 * Returns the addresses config/ZamaConfig.sol hardcodes for `chainId` (checksummed, by CoprocessorConfig field), or
 * undefined if ZamaConfig.sol does not support that chain. Throws if ZamaConfig.sol no longer has the expected structure.
 */
export function getZamaConfigAddresses(chainId: bigint): Readonly<Record<string, string>> | undefined {
  const source = fs.readFileSync(ZAMA_CONFIG_SOL, 'utf8');

  // getCoprocessorConfig() routes each chain id to a private config function:
  //   if (block.chainid == 1) { config = _getEthereumConfig(); } else if (...) ...
  const routing = /function getCoprocessorConfig\(\)[\s\S]*?revert ZamaProtocolUnsupported\(\);/.exec(source);
  if (routing === null) {
    throw new Error(`${ZAMA_CONFIG_SOL}: cannot find getCoprocessorConfig()`);
  }
  let configFunction: string | undefined;
  for (const [, routedChainId, routedFunction] of routing[0].matchAll(
    /block\.chainid == (\d+)\) \{\s*config = (_\w+)\(\);/g,
  )) {
    if (routedChainId === chainId.toString()) {
      configFunction = routedFunction;
    }
  }
  if (configFunction === undefined) {
    return undefined;
  }

  // The `Name: 0x…` fields of the CoprocessorConfig({ ... }) returned by `configFunction`.
  const body = new RegExp(
    `function ${configFunction}\\(\\) private pure returns \\(CoprocessorConfig memory\\) \\{[\\s\\S]*?CoprocessorConfig\\(\\{([\\s\\S]*?)\\}\\)`,
  ).exec(source);
  if (body === null) {
    throw new Error(`${ZAMA_CONFIG_SOL}: cannot find the CoprocessorConfig returned by ${configFunction}()`);
  }
  const addresses: Record<string, string> = {};
  for (const [, name, address] of body[1].matchAll(/(\w+):\s*(0x[0-9a-fA-F]{40})\b/g)) {
    addresses[name] = getAddress(address);
  }
  return addresses;
}

/** Returns the `address constant <name> = 0x…;` constants of a FHEVMHostAddresses.sol file (checksummed). */
export function readHostAddressConstants(hostAddressesSol: string): Readonly<Record<string, string>> {
  const constants: Record<string, string> = {};
  for (const [, name, address] of fs
    .readFileSync(hostAddressesSol, 'utf8')
    .matchAll(/address constant (\w+) = (0x[0-9a-fA-F]{40});/g)) {
    constants[name] = getAddress(address);
  }
  return constants;
}

/**
 * Throws, listing every mismatch, unless the ACL, FHEVMExecutor and KMSVerifier addresses of `hostAddressesSol` are
 * the ones ZamaConfig.sol hardcodes for `chainId`.
 */
export function assertHostAddressesMatchZamaConfig(hostAddressesSol: string, chainId: bigint): void {
  const expected = getZamaConfigAddresses(chainId);
  if (expected === undefined) {
    throw new Error(`config/ZamaConfig.sol has no config for chain ${chainId}: cannot check ${hostAddressesSol}`);
  }
  const actual = readHostAddressConstants(hostAddressesSol);

  const mismatches: string[] = [];
  for (const [field, constant] of Object.entries(ZAMA_CONFIG_FIELD_TO_HOST_CONSTANT)) {
    const expectedAddress: string | undefined = Object.hasOwn(expected, field) ? expected[field] : undefined;
    const actualAddress: string | undefined = Object.hasOwn(actual, constant) ? actual[constant] : undefined;
    if (expectedAddress === undefined) {
      mismatches.push(`ZamaConfig.sol has no ${field} for chain ${chainId}`);
    } else if (actualAddress === undefined) {
      mismatches.push(`${constant} is missing from ${hostAddressesSol}`);
    } else if (actualAddress !== expectedAddress) {
      mismatches.push(`${constant} = ${actualAddress}, but ZamaConfig.sol ${field} = ${expectedAddress}`);
    }
  }
  if (mismatches.length > 0) {
    throw new Error(
      `The host contract addresses do not match config/ZamaConfig.sol (chain ${chainId}):\n` +
        mismatches.map((m) => `  - ${m}`).join('\n'),
    );
  }
}
