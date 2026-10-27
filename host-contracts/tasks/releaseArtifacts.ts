import { execSync } from 'child_process';
import { ZeroAddress, keccak256 } from 'ethers';
import fs from 'fs';
import { task } from 'hardhat/config';
import type { HardhatRuntimeEnvironment } from 'hardhat/types';
import path from 'path';

// Release artifacts: the host contracts as they are built for one chain.
//
// The host contracts compile the proxy addresses of their chain in as constants
// (addresses/FHEVMHostAddresses.sol), so their bytecode is per chain. Given a release tag and the
// chain's proxy addresses, taken from the protocol registry, the build is reproducible: the
// manifest written here holds the runtime bytecode hash of every upgradeable contract, which anyone
// can compare with the code of an implementation deployed on that chain.

// Read when the task runs, not when Hardhat loads: images that only ship contracts and tasks
// do not carry upgrade-manifest.json.
function upgradeableContracts(): string[] {
  return JSON.parse(fs.readFileSync(path.join(__dirname, '../upgrade-manifest.json'), 'utf8'));
}

// Registry contract type behind each address, in the order task:deployEmptyUUPSProxies and then
// task:deployPauserSet write them, so the generated address file, and with it the metadata hash,
// is byte for byte the one a deployment produces.
const ADDRESS_TASKS: { type: string; task: string; optional?: boolean; defaultsToZero?: boolean }[] = [
  { type: 'fhevm_acl', task: 'task:setACLAddress' },
  { type: 'fhevm_executor', task: 'task:setFHEVMExecutorAddress' },
  { type: 'fhevm_kms_verifier', task: 'task:setKMSVerifierAddress' },
  { type: 'fhevm_verifier', task: 'task:setInputVerifierAddress' },
  { type: 'fhevm_hcu_limit', task: 'task:setHCULimitAddress' },
  { type: 'fhevm_protocol_config', task: 'task:setProtocolConfigAddress' },
  { type: 'fhevm_kms_generation', task: 'task:setKMSGenerationAddress', optional: true },
  { type: 'confidential_bridge', task: 'task:setBridgeAddress', defaultsToZero: true },
  { type: 'pauser_set', task: 'task:setPauserSetAddress' },
];

type RegistryContract = { address: string; chain: string; type: string };

async function writeHostAddresses(hre: HardhatRuntimeEnvironment, registryFile: string, chain: string) {
  const registry = JSON.parse(fs.readFileSync(registryFile, 'utf8'));
  const onChain = Object.values(registry.contracts as Record<string, RegistryContract>).filter(
    (c) => c.chain === chain,
  );
  const addresses: Record<string, string> = {};
  for (const { type, task: taskName, optional, defaultsToZero } of ADDRESS_TASKS) {
    const matches = onChain.filter((c) => c.type === type);
    if (matches.length > 1) {
      throw new Error(`${registryFile} lists ${matches.length} ${type} contracts on ${chain}`);
    }
    const address = matches[0]?.address ?? (defaultsToZero ? ZeroAddress : undefined);
    if (address === undefined) {
      if (optional) continue;
      throw new Error(`${registryFile} lists no ${type} contract on ${chain}`);
    }
    if (!hre.ethers.isAddress(address)) {
      throw new Error(`${type} on ${chain} in ${registryFile} is not an address: ${address}`);
    }
    await hre.run(taskName, { address });
    addresses[type] = address;
  }
  return { addresses, chainId: registry.chains?.[chain]?.chain_id };
}

task('task:buildReleaseArtifacts')
  .addParam('registry', 'Protocol registry file, e.g. mainnet.json from zama-ai/protocol-registry')
  .addParam('chain', 'Registry chain key, e.g. ethereum')
  .addParam('outDir', 'Directory to write the manifest and build info to')
  .addOptionalParam('registryRef', 'Where the registry file came from, recorded in the manifest', '')
  .setAction(async function ({ registry, chain, outDir, registryRef }, hre) {
    const { addresses, chainId } = await writeHostAddresses(hre, registry, chain);
    await hre.run('compile', { force: true, quiet: true });

    const fullyQualifiedNames = await hre.artifacts.getAllFullyQualifiedNames();
    fs.mkdirSync(path.join(outDir, 'build-info'), { recursive: true });
    const contracts: Record<string, unknown> = {};
    for (const name of upgradeableContracts()) {
      const matches = fullyQualifiedNames.filter((fqn) => fqn.startsWith('contracts/') && fqn.endsWith(`:${name}`));
      if (matches.length !== 1) {
        throw new Error(`Expected one contract named ${name} under contracts/, found ${matches.length}`);
      }
      const [sourceName] = matches[0].split(':');
      const buildInfo = await hre.artifacts.getBuildInfo(matches[0]);
      if (!buildInfo) {
        throw new Error(`No build info for ${matches[0]}`);
      }
      const runtime = buildInfo.output.contracts[sourceName][name].evm.deployedBytecode;
      if (runtime.object.includes('__$')) {
        throw new Error(`${matches[0]} links external libraries, which a bytecode hash cannot pin`);
      }
      fs.writeFileSync(path.join(outDir, 'build-info', `${buildInfo.id}.json`), JSON.stringify(buildInfo));
      contracts[name] = {
        source: matches[0],
        compiler: buildInfo.solcLongVersion,
        buildInfo: `build-info/${buildInfo.id}.json`,
        // Hash of the compiler's runtime bytecode, in which immutables are zero. Zero the
        // immutableReferences ranges of the deployed code before hashing it to compare.
        runtimeBytecodeHash: keccak256(`0x${runtime.object}`),
        immutableReferences: runtime.immutableReferences ?? {},
      };
    }

    const manifest = {
      commit: execSync('git rev-parse HEAD').toString().trim(),
      registry: registryRef || path.basename(registry),
      chain,
      chainId,
      addresses,
      contracts,
    };
    fs.writeFileSync(path.join(outDir, 'manifest.json'), JSON.stringify(manifest, null, 2) + '\n');
    console.log(`Release artifacts for ${Object.keys(contracts).length} contracts on ${chain} written to ${outDir}`);
  });
