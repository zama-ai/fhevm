#!/usr/bin/env node
// =============================================================================
// fhevm-info.mjs - Generate FHEVM host deployment information as JSON
//
// The ACL address is the root for resolving host addresses exposed by ACL and
// FHEVMExecutor. The KMSVerifier address is provided separately because it is not
// exposed by the current host contract getters.
// =============================================================================

import { spawnSync } from 'child_process';
import { env, argv, stderr, stdout } from 'process';

const USAGE = `Usage:
  node fhevm-info.mjs <acl-address> <kms-verifier-address> [options]
  node fhevm-info.mjs --acl <address> --kms-verifier <address> [options]

Generate FHEVM host deployment information as JSON from on-chain getters.

Options:
  --acl <address>             ACL contract address.
  --kms-verifier <address>    KMSVerifier contract address.
  --rpc-url <url>             Target RPC URL.
  --anvil-port <port>         Shorthand for http://127.0.0.1:<port>.
  -h, --help                  Show this help text.

Notes:
  - If no RPC target is provided, the script uses RPC_URL, then ANVIL_PORT, then 8545.

Example:
  node fhevm-info.mjs --acl 0x05fD9B5EFE0a996095f42Ed7e77c390810CF660c --kms-verifier 0xa1880e99d86F081E8D3868A8C4732C8f65dfdB11 --rpc-url http://localhost:8545`;

function die(msg) {
  stderr.write(`Error: ${msg}\n`);
  process.exit(1);
}

function isAddress(s) {
  return /^0x[0-9a-fA-F]{40}$/.test(s);
}

function requireAddress(name, address) {
  if (!isAddress(address)) die(`${name} must be a 20-byte hex address`);
}

function normalizeAddress(addr) {
  return addr.toLowerCase();
}

function requireCommand(name) {
  const result = spawnSync(name, ['--version'], { encoding: 'utf8' });
  if (result.error) die(`${name} is required`);
}

function castRun(args) {
  const result = spawnSync('cast', args, { encoding: 'utf8' });
  if (result.error) die(`cast is required`);
  if (result.status !== 0) {
    die(`cast ${args[0]} failed: ${(result.stderr || '').trim()}`);
  }
  return result.stdout.trim();
}

function castView(contractAddress, signature, rpcUrl) {
  return castRun(['call', contractAddress, signature, '--rpc-url', rpcUrl]);
}

function contractHasCode(contractAddress, rpcUrl) {
  const code = castRun(['code', contractAddress, '--rpc-url', rpcUrl]);
  return code !== '' && code !== '0x';
}

function requireContractCode(name, contractAddress, rpcUrl) {
  if (!contractHasCode(contractAddress, rpcUrl)) {
    die(`${name} has no code at ${contractAddress} on ${rpcUrl}`);
  }
}

function cleanCastString(value) {
  value = value.replace(/\r$/, '');
  if (value.startsWith('"') && value.endsWith('"')) {
    value = value.slice(1, -1);
  }
  return value;
}

function cleanCastUint(value) {
  value = value.replace(/\r$/, '');
  return value.split(/\s+/)[0];
}

function addressArrayJson(value) {
  return (value.match(/0x[0-9a-fA-F]{40}/gi) || []);
}

// ---------------------------------------------------------------------------
// Protocol version resolution (mirrors ProtocolVersionResolver-p.ts)
// ---------------------------------------------------------------------------

/**
 * ACL contract version → protocol version mapping table.
 * Kept in sync with src/core/runtime/ProtocolVersionResolver-p.ts.
 * Each entry is a closed lower bound [lower, upper) interval.
 */
const ACL_PROTOCOL_VERSION_TABLE = [
  { acl: { lower: '0.2.0', upper: '0.3.0' }, protocol: '0.11.0' },
  { acl: { lower: '0.3.0', upper: '0.4.0' }, protocol: '0.12.0' },
  { acl: { lower: '0.4.0', upper: '0.5.0' }, protocol: '0.13.0' },
  { acl: { lower: '0.5.0', upper: '0.6.0' }, protocol: '0.14.0' },
];

function compareSemver(a, b) {
  const parse = (s) => s.split('.').map(Number);
  const [aMaj, aMin, aPat] = parse(a);
  const [bMaj, bMin, bPat] = parse(b);
  if (aMaj !== bMaj) return aMaj < bMaj ? -1 : 1;
  if (aMin !== bMin) return aMin < bMin ? -1 : 1;
  if (aPat !== bPat) return aPat < bPat ? -1 : 1;
  return 0;
}

/**
 * Maps an ACL host-contract version string (e.g. "ACL v0.5.1") to a protocol
 * version resolution, mirroring protocolVersionFromAclVersion() in the SDK.
 *
 * Returns { version, comparator } where comparator is:
 *   'eq'  — exact match found in the table
 *   'lt'  — ACL is older than the table's oldest entry (protocol < version)
 *   'gt'  — ACL is newer than the table's newest entry (protocol > version)
 */
function protocolVersionFromAclVersion(aclVersionString) {
  const m = aclVersionString.match(/v(\d+\.\d+\.\d+)/);
  if (!m) die(`Cannot parse ACL version from: ${aclVersionString}`);
  const semver = m[1];

  const known = ACL_PROTOCOL_VERSION_TABLE.find(
    (entry) => compareSemver(semver, entry.acl.lower) >= 0 && compareSemver(semver, entry.acl.upper) < 0,
  );
  if (known !== undefined) return { version: known.protocol, comparator: 'eq' };

  const oldest = ACL_PROTOCOL_VERSION_TABLE[0];
  if (compareSemver(semver, oldest.acl.lower) < 0) {
    return { version: oldest.protocol, comparator: 'lt' };
  }

  const newest = ACL_PROTOCOL_VERSION_TABLE[ACL_PROTOCOL_VERSION_TABLE.length - 1];
  return { version: newest.protocol, comparator: 'gt' };
}

function eip712DomainJson(contractAddress, rpcUrl) {
  const raw = castView(contractAddress, 'eip712Domain()(bytes1,string,string,uint256,address,bytes32,uint256[])', rpcUrl);
  const lines = raw.split('\n');
  // line 0: bytes1 (fields[0]), line 1: name, line 2: version, line 3: chainId, line 4: verifyingContract
  const name = cleanCastString(lines[1] ?? '');
  const version = cleanCastString(lines[2] ?? '');
  const chainId = Number(cleanCastUint(lines[3] ?? '0'));
  const verifyingContract = (lines[4] ?? '').trim().replace(/\r$/, '');
  return { name, version, chainId, verifyingContract };
}

// ---------------------------------------------------------------------------
// Argument parsing
// ---------------------------------------------------------------------------

const cliArgs = argv.slice(2);
let aclAddress = '';
let kmsVerifierAddress = '';
let rpcUrl = '';
const positional = [];

for (let i = 0; i < cliArgs.length; i++) {
  const arg = cliArgs[i];
  switch (arg) {
    case '--acl': {
      const val = cliArgs[i + 1];
      if (!val || val.startsWith('-')) die('--acl requires a value');
      aclAddress = val;
      i++;
      break;
    }
    case '--kms-verifier': {
      const val = cliArgs[i + 1];
      if (!val || val.startsWith('-')) die('--kms-verifier requires a value');
      kmsVerifierAddress = val;
      i++;
      break;
    }
    case '--rpc-url': {
      const val = cliArgs[i + 1];
      if (!val || val.startsWith('-')) die('--rpc-url requires a value');
      rpcUrl = val;
      i++;
      break;
    }
    case '--anvil-port': {
      const val = cliArgs[i + 1];
      if (!val || val.startsWith('-')) die('--anvil-port requires a value');
      rpcUrl = `http://127.0.0.1:${val}`;
      i++;
      break;
    }
    case '-h':
    case '--help':
      stdout.write(USAGE + '\n');
      process.exit(0);
      break;
    default:
      if (arg.startsWith('-')) die(`unknown option: ${arg}`);
      positional.push(arg);
  }
}

if (positional.length > 2) die('too many positional arguments');

if (!aclAddress && positional.length >= 1) aclAddress = positional[0];
if (!kmsVerifierAddress && positional.length >= 2) kmsVerifierAddress = positional[1];

if (!aclAddress || !kmsVerifierAddress) {
  stderr.write(USAGE + '\n');
  process.exit(1);
}

if (!rpcUrl) {
  if (env['RPC_URL']) {
    rpcUrl = env['RPC_URL'];
  } else if (env['ANVIL_PORT']) {
    rpcUrl = `http://127.0.0.1:${env['ANVIL_PORT']}`;
  } else {
    rpcUrl = 'http://127.0.0.1:8545';
  }
}

// ---------------------------------------------------------------------------
// Validation and on-chain queries
// ---------------------------------------------------------------------------

requireCommand('cast');

requireAddress('ACL address', aclAddress);
requireAddress('KMSVerifier address', kmsVerifierAddress);

const chainId = Number(castRun(['chain-id', '--rpc-url', rpcUrl]));

requireContractCode('ACL', aclAddress, rpcUrl);
requireContractCode('KMSVerifier', kmsVerifierAddress, rpcUrl);

const aclOwnerAddress = castView(aclAddress, 'owner()(address)', rpcUrl);
const fhevmExecutorAddress = castView(aclAddress, 'getFHEVMExecutorAddress()(address)', rpcUrl);
const pauserSetAddress = castView(aclAddress, 'getPauserSetAddress()(address)', rpcUrl);

requireContractCode('FHEVMExecutor', fhevmExecutorAddress, rpcUrl);
requireContractCode('PauserSet', pauserSetAddress, rpcUrl);

const executorAclAddress = castView(fhevmExecutorAddress, 'getACLAddress()(address)', rpcUrl);
const inputVerifierAddress = castView(fhevmExecutorAddress, 'getInputVerifierAddress()(address)', rpcUrl);
const hcuLimitAddress = castView(fhevmExecutorAddress, 'getHCULimitAddress()(address)', rpcUrl);

requireContractCode('InputVerifier', inputVerifierAddress, rpcUrl);
requireContractCode('HCULimit', hcuLimitAddress, rpcUrl);

const hcuLimitExecutorAddress = castView(hcuLimitAddress, 'getFHEVMExecutorAddress()(address)', rpcUrl);

if (normalizeAddress(executorAclAddress) !== normalizeAddress(aclAddress)) {
  die(`FHEVMExecutor.getACLAddress() returned ${executorAclAddress}, expected ${aclAddress}`);
}

if (normalizeAddress(hcuLimitExecutorAddress) !== normalizeAddress(fhevmExecutorAddress)) {
  die(`HCULimit.getFHEVMExecutorAddress() returned ${hcuLimitExecutorAddress}, expected ${fhevmExecutorAddress}`);
}

const aclVersion = cleanCastString(castView(aclAddress, 'getVersion()(string)', rpcUrl));
const fhevmExecutorVersion = cleanCastString(castView(fhevmExecutorAddress, 'getVersion()(string)', rpcUrl));
const fhevmExecutorHandleVersion = Number(cleanCastUint(castView(fhevmExecutorAddress, 'getHandleVersion()(uint8)', rpcUrl)));
const kmsVerifierVersion = cleanCastString(castView(kmsVerifierAddress, 'getVersion()(string)', rpcUrl));
const inputVerifierVersion = cleanCastString(castView(inputVerifierAddress, 'getVersion()(string)', rpcUrl));
const inputVerifierHandleVersion = Number(cleanCastUint(castView(inputVerifierAddress, 'getHandleVersion()(uint8)', rpcUrl)));
const hcuLimitVersion = cleanCastString(castView(hcuLimitAddress, 'getVersion()(string)', rpcUrl));
const pauserSetVersion = cleanCastString(castView(pauserSetAddress, 'getVersion()(string)', rpcUrl));

if (fhevmExecutorHandleVersion !== inputVerifierHandleVersion) {
  die(`handle version mismatch: FHEVMExecutor=${fhevmExecutorHandleVersion}, InputVerifier=${inputVerifierHandleVersion}`);
}
const handleVersion = fhevmExecutorHandleVersion;

const kmsSigners = addressArrayJson(castView(kmsVerifierAddress, 'getKmsSigners()(address[])', rpcUrl));
const coprocessorSigners = addressArrayJson(castView(inputVerifierAddress, 'getCoprocessorSigners()(address[])', rpcUrl));

const kmsThreshold = Number(cleanCastUint(castView(kmsVerifierAddress, 'getThreshold()(uint256)', rpcUrl)));
const inputThreshold = Number(cleanCastUint(castView(inputVerifierAddress, 'getThreshold()(uint256)', rpcUrl)));

const kmsEip712Domain = eip712DomainJson(kmsVerifierAddress, rpcUrl);
const inputEip712Domain = eip712DomainJson(inputVerifierAddress, rpcUrl);
const gatewayChainId = kmsEip712Domain.chainId;

const protocolVersion = protocolVersionFromAclVersion(aclVersion);

// ---------------------------------------------------------------------------
// Output
// ---------------------------------------------------------------------------

const output = {
  rpc: {
    url: rpcUrl,
  },
  chainId,
  hostContracts: {
    acl: aclAddress,
    kmsVerifier: kmsVerifierAddress,
    fhevmExecutor: fhevmExecutorAddress,
    inputVerifier: inputVerifierAddress,
    hcuLimit: hcuLimitAddress,
    pauserSet: pauserSetAddress,
  },
  owners: {
    acl: aclOwnerAddress,
    kmsVerifier: aclOwnerAddress,
    fhevmExecutor: aclOwnerAddress,
    inputVerifier: aclOwnerAddress,
    hcuLimit: aclOwnerAddress,
    pauserSet: aclOwnerAddress,
  },
  handleVersion,
  gatewayChainId,
  protocolVersion,
  versions: {
    acl: aclVersion,
    fhevmExecutor: fhevmExecutorVersion,
    kmsVerifier: kmsVerifierVersion,
    inputVerifier: inputVerifierVersion,
    hcuLimit: hcuLimitVersion,
    pauserSet: pauserSetVersion,
  },
  kmsVerifier: {
    signers: kmsSigners,
    threshold: kmsThreshold,
  },
  inputVerifier: {
    signers: coprocessorSigners,
    threshold: inputThreshold,
  },
  eip712Domains: {
    kmsVerifier: kmsEip712Domain,
    inputVerifier: inputEip712Domain,
  },
};

stdout.write(JSON.stringify(output, null, 4) + '\n');
