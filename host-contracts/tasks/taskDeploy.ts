import { HardhatUpgrades } from '@openzeppelin/hardhat-upgrades';
import dotenv from 'dotenv';
import { JsonRpcProvider, type Signer, Wallet } from 'ethers';
import fs from 'fs';
import { task, types } from 'hardhat/config';
import type { HardhatEthersHelpers, HardhatRuntimeEnvironment, TaskArguments } from 'hardhat/types';
import path from 'path';

import {
  type CanonicalSnapshot,
  buildCanonicalUpgradeProposal,
  buildSnapshotArtifact,
  parseSnapshotArtifact,
  readCanonicalSnapshot,
} from './protocolConfigMirror';
import { assertContractMatchesVersionPrefix } from './utils/contractVersion';
import { formatError } from './utils/formatError';
import { CRS_COUNTER_BASE, KEY_COUNTER_BASE } from './utils/kmsGenerationConstants';
import { getRequiredEnvVar } from './utils/loadVariables';
import { executeUpgradeProposal, printUpgradeProposal, verifyProposalImplementation } from './utils/upgradeProposal';

const ADDRESSES_DIR = path.join(__dirname, '../addresses');
const HOST_ENV_FILE = path.join(ADDRESSES_DIR, '.env.host');
const HOST_ADDRESSES_FILE = path.join(ADDRESSES_DIR, 'FHEVMHostAddresses.sol');
const LEGACY_DEPLOY_ALL_HOST_CONTRACTS_WARNING = `task:deployLegacyAllHostContracts is deprecated and will be removed after the v0.13 rollout.
It deploys KMSGeneration and is valid only for canonical-host deployments.
Use task:deployAllHostContracts --with-kms-generation true instead.`;

export function ensureAddressesDirectoryExists() {
  fs.mkdirSync(ADDRESSES_DIR, { recursive: true });
}

export function readHostEnv() {
  return dotenv.parse(fs.readFileSync(HOST_ENV_FILE));
}

export function writeHostEnvLine(content: string, mode: 'w' | 'a') {
  fs.writeFileSync(HOST_ENV_FILE, content, { flag: mode });
}

function writeHostAddressesSol(content: string, mode: 'w' | 'a') {
  fs.writeFileSync(HOST_ADDRESSES_FILE, content, { encoding: 'utf8', flag: mode });
}

export function readExistingHostEnv(): Record {
  if (!fs.existsSync(HOST_ENV_FILE)) {
    return {};
  }
  return readHostEnv();
}

export async function waitForTaskReady(hre: HardhatRuntimeEnvironment, taskName: string, timeoutMs = 60_000): Promise {
  const deadline = Date.now() + timeoutMs;

  while (true) {
    try {
      await hre.run(taskName);
      return;
    } catch (err) {
      if (Date.now() >= deadline) {
        throw new Error(`${taskName} did not become ready after ${timeoutMs}ms: ${formatError(err)}`);
      }
      await new Promise((resolve) => setTimeout(resolve, 500));
    }
  }
}

// Re-exported for existing call sites. Lives in utils/contractVersion to avoid a cyclic import:
// protocolConfigMirror needs it too, and taskDeploy already imports from there.
export { assertContractMatchesVersionPrefix };

////////////////////////////////////////////////////////////////////////////////
// All Host Contracts
////////////////////////////////////////////////////////////////////////////////

const PROTOCOL_CONFIG_SOURCES = ['fresh', 'canonical'] as const;
type ProtocolConfigSource = (typeof PROTOCOL_CONFIG_SOURCES)[number];

task('task:deployAllHostContracts')
  .addParam(
    'withKmsGeneration',
    'Whether to deploy canonical-host-only KMSGeneration. Required: true for canonical host, false for non-canonical host.',
    undefined,
    types.boolean,
  )
  .addOptionalParam(
    'protocolConfigSource',
    "How to initialize ProtocolConfig: 'fresh' (default) calls initializeFromEmptyProxy with env-driven KMS nodes/thresholds; 'canonical' mirrors the canonical chain's ProtocolConfig via task:deployProtocolConfigFromCanonical (non-canonical hosts only).",
    'fresh',
    types.string,
  )
  .addOptionalParam(
    'canonicalRpcUrl',
    'RPC URL of the canonical host chain. Required with --protocol-config-source canonical.',
    undefined,
    types.string,
  )
  .addOptionalParam(
    'canonicalProtocolConfigAddress',
    "Address of the canonical chain's ProtocolConfig. Required with --protocol-config-source canonical.",
    undefined,
    types.string,
  )
  .setAction(async function (
    {
      withKmsGeneration,
      protocolConfigSource,
      canonicalRpcUrl,
      canonicalProtocolConfigAddress,
    }: {
      withKmsGeneration: boolean;
      protocolConfigSource: string;
      canonicalRpcUrl?: string;
      canonicalProtocolConfigAddress?: string;
    },
    hre,
  ) {
    if (!PROTOCOL_CONFIG_SOURCES.includes(protocolConfigSource as ProtocolConfigSource)) {
      throw new Error(
        `Invalid --protocol-config-source "${protocolConfigSource}". Allowed values: ${PROTOCOL_CONFIG_SOURCES.join(
          ', ',
        )}.`,
      );
    }
    if (protocolConfigSource === 'canonical') {
      if (withKmsGeneration) {
        throw new Error(
          '--protocol-config-source canonical seeds a non-canonical replica; it cannot be combined with --with-kms-generation true.',
        );
      }
      if (!(canonicalRpcUrl && canonicalProtocolConfigAddress)) {
        throw new Error(
          '--protocol-config-source canonical requires --canonical-rpc-url and --canonical-protocol-config-address.',
        );
      }
    }

    if (process.env.SOLIDITY_COVERAGE !== 'true') {
      await hre.run('clean');
    }

    await hre.run('task:deployEmptyUUPSProxies', { withKmsGeneration });
    await hre.run('compile:specific', { contract: 'contracts/immutable' });
    await hre.run('task:deployPauserSet');

    // The deployEmptyUUPSProxies task may have updated the contracts' addresses in `addresses/*.sol`.
    // Thus, we must re-compile the contracts with these new addresses, otherwise the old ones will be
    // used.
    await hre.run('compile:specific', { contract: 'contracts' });

    await hre.run('task:deployACL');
    await hre.run('task:deployFHEVMExecutor');
    if (protocolConfigSource === 'canonical') {
      await hre.run('task:deployProtocolConfigFromCanonical', { canonicalRpcUrl, canonicalProtocolConfigAddress });
    } else {
      await hre.run('task:deployProtocolConfig');
    }
    if (withKmsGeneration) {
      await hre.run('task:deployKMSGeneration');
    }
    await hre.run('task:deployKMSVerifier');
    await hre.run('task:deployInputVerifier');
    await hre.run('task:deployHCULimit');

    await hre.run('task:deployBridge');

    console.log('Contract deployment done!');
  });

task('task:deployLegacyAllHostContracts').setAction(async function (_, hre) {
  console.warn(LEGACY_DEPLOY_ALL_HOST_CONTRACTS_WARNING);
  await hre.run('task:deployAllHostContracts', { withKmsGeneration: true });
});

////////////////////////////////////////////////////////////////////////////////
// UUPS
////////////////////////////////////////////////////////////////////////////////

async function deployEmptyUUPSForACL(ethers: HardhatEthersHelpers, upgrades: HardhatUpgrades, deployer: Wallet) {
  console.log('Deploying an EmptyUUPSProxyACL proxy contract...');
  const factory = await ethers.getContractFactory('EmptyUUPSProxyACL', deployer);
  const UUPSEmptyACL = await upgrades.deployProxy(factory, [deployer.address], {
    initializer: 'initialize',
    kind: 'uups',
  });
  await UUPSEmptyACL.waitForDeployment();
  const UUPSEmptyACLAddress = await UUPSEmptyACL.getAddress();
  console.log('EmptyUUPSProxyACL proxy contract successfully deployed!');
  return UUPSEmptyACLAddress;
}

export async function deployEmptyUUPS(ethers: HardhatEthersHelpers, upgrades: HardhatUpgrades, deployer: Wallet) {
  console.log('Deploying an EmptyUUPSProxy proxy contract...');
  const factory = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
  const UUPSEmpty = await upgrades.deployProxy(factory, {
    initializer: 'initialize',
    kind: 'uups',
  });
  await UUPSEmpty.waitForDeployment();
  const UUPSEmptyAddress = await UUPSEmpty.getAddress();
  console.log('EmptyUUPSProxy proxy contract successfully deployed!');
  return UUPSEmptyAddress;
}

task('task:deployEmptyUUPSProxies')
  .addParam(
    'withKmsGeneration',
    'Whether to deploy the canonical-host-only KMSGeneration proxy. Required: true for canonical host, false for non-canonical host.',
    undefined,
    types.boolean,
  )
  .setAction(async function (
    { withKmsGeneration }: { withKmsGeneration: boolean },
    { ethers, upgrades, run, network },
  ) {
    // Compile the EmptyUUPS proxy contract for ACL
    await run('compile:specific', { contract: 'contracts/emptyProxyACL' });

    const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
    const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);

    // Ensure the addresses directory exists
    ensureAddressesDirectoryExists();

    // Set ACL Address
    const aclAddress = await deployEmptyUUPSForACL(ethers, upgrades, deployer);
    await run('task:setACLAddress', { address: aclAddress });

    // Compile the EmptyUUPS proxy contract for other contracts
    await run('compile:specific', { contract: 'contracts/emptyProxy' });

    // Set FHEVMExecutor Address
    const fhevmExecutorAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
    await run('task:setFHEVMExecutorAddress', { address: fhevmExecutorAddress });

    // Set KMSVerifier Address
    const kmsVerifierAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
    await run('task:setKMSVerifierAddress', { address: kmsVerifierAddress });

    // Set InputVerifier Address
    const inputVerifierAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
    await run('task:setInputVerifierAddress', { address: inputVerifierAddress });

    // Set HCULimit Address
    const HCULimitAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
    await run('task:setHCULimitAddress', { address: HCULimitAddress });

    // Set ProtocolConfig Address
    const protocolConfigAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
    await run('task:setProtocolConfigAddress', { address: protocolConfigAddress });

    if (withKmsGeneration) {
      // Set KMSGeneration Address
      const kmsGenerationAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
      await run('task:setKMSGenerationAddress', { address: kmsGenerationAddress });
    }

    // ConfidentialBridge is opt-in via LZ_ENDPOINT_ADDRESS. We only deploy its empty proxy when a
    // real LayerZero endpoint is configured (env var set and code deployed at that address). When
    // it isn't, we deploy nothing for the bridge — not even an empty proxy — so dev tooling can
    // detect the absence of a confidential bridge simply by calling ACL:getConfidentialBridgeAddress()
    // and noticing that it returns the null address.
    const lzEndpoint = process.env.LZ_ENDPOINT_ADDRESS;
    if (!lzEndpoint) {
      console.log(
        '[task:deployEmptyUUPSProxies] LZ_ENDPOINT_ADDRESS not set; ' +
          'skipping ConfidentialBridge empty-proxy deployment. This will set the constant bridge address in ACL as the null address, ' +
          'otherwise if it was a mistake please set LZ_ENDPOINT_ADDRESS and re-run to deploy the bridge.',
      );
      await run('task:setBridgeAddress', { address: ethers.ZeroAddress });
    } else if (!ethers.isAddress(lzEndpoint)) {
      throw new Error(
        `[task:deployEmptyUUPSProxies] LZ_ENDPOINT_ADDRESS (${lzEndpoint}) is not a valid address. ` +
          'Fix LZ_ENDPOINT_ADDRESS, or unset it to skip the bridge, and re-run.',
      );
    } else if ((await ethers.provider.getCode(lzEndpoint)) === '0x') {
      // No code at the endpoint is a fatal misconfiguration on real networks, but expected on the
      // in-memory 'hardhat' network (bridge tests deploy their own proxies), so there we skip and
      // pin the null bridge address instead of panicking.
      if (network.name !== 'hardhat') {
        throw new Error(
          `[task:deployEmptyUUPSProxies] No contract deployed at LZ_ENDPOINT_ADDRESS (${lzEndpoint}) on network "${network.name}". ` +
            'Point LZ_ENDPOINT_ADDRESS at a real LayerZero endpoint, or unset it to skip the bridge, and re-run.',
        );
      }
      console.log(
        `[task:deployEmptyUUPSProxies] No contract deployed at LZ_ENDPOINT_ADDRESS (${lzEndpoint}) ` +
          "on the in-memory 'hardhat' network; skipping ConfidentialBridge empty-proxy deployment. " +
          'This will set the constant bridge address in ACL as the null address.',
      );
      await run('task:setBridgeAddress', { address: ethers.ZeroAddress });
    } else {
      const confidentialBridgeAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
      await run('task:setBridgeAddress', { address: confidentialBridgeAddress });
    }
  });

task('task:deployEmptyProxiesProtocolConfigKMSGeneration').setAction(async function (_, { ethers, upgrades, run }) {
  ensureAddressesDirectoryExists();

  const existingEnv = readExistingHostEnv();

  const targets = [
    { envKey: 'PROTOCOL_CONFIG_CONTRACT_ADDRESS', setterTask: 'task:setProtocolConfigAddress' },
    { envKey: 'KMS_GENERATION_CONTRACT_ADDRESS', setterTask: 'task:setKMSGenerationAddress' },
    { envKey: 'CONFIDENTIAL_BRIDGE_CONTRACT_ADDRESS', setterTask: 'task:setBridgeAddress' },
  ] as const;

  const missingTargets = targets.filter(({ envKey }) => !existingEnv[envKey]);

  if (missingTargets.length === 0) {
    console.warn(
      'Empty-proxy bootstrap is a no-op; addresses/.env.host already contains ProtocolConfig, KMSGeneration and ConfidentialBridge.',
    );
    return;
  }

  const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);

  await run('compile:specific', { contract: 'contracts/emptyProxy' });

  for (const { envKey, setterTask } of missingTargets) {
    const proxyAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
    await run(setterTask, { address: proxyAddress });
    process.env[envKey] = proxyAddress;
  }
});

////////////////////////////////////////////////////////////////////////////////
// ACL
////////////////////////////////////////////////////////////////////////////////

task('task:deployACL').setAction(async function (taskArguments: TaskArguments, { ethers, upgrades }) {
  const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);
  const currentImplementation = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
  const newImplem = await ethers.getContractFactory('ACL', deployer);
  const parsedEnv = readHostEnv();
  const proxyAddress = parsedEnv.ACL_CONTRACT_ADDRESS;
  const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);
  await upgrades.upgradeProxy(proxy, newImplem, {
    call: { fn: 'initializeFromEmptyProxy' },
  });
  console.log('ACL code set successfully at address:', proxyAddress);
});

////////////////////////////////////////////////////////////////////////////////
// FHEVMExecutor
////////////////////////////////////////////////////////////////////////////////

task('task:deployFHEVMExecutor').setAction(async function (taskArguments: TaskArguments, { ethers, upgrades }) {
  const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);
  const currentImplementation = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
  const newImplem = await ethers.getContractFactory('./contracts/FHEVMExecutor.sol:FHEVMExecutor', deployer);
  const parsedEnv = readHostEnv();
  const proxyAddress = parsedEnv.FHEVM_EXECUTOR_CONTRACT_ADDRESS;
  const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);
  await upgrades.upgradeProxy(proxy, newImplem, { call: { fn: 'initializeFromEmptyProxy' } });
  console.log('FHEVMExecutor code set successfully at address:', proxyAddress);
});

////////////////////////////////////////////////////////////////////////////////
// KMSVerifier
////////////////////////////////////////////////////////////////////////////////

task('task:deployKMSVerifier').setAction(async function (_taskArguments: TaskArguments, hre) {
  const { ethers, upgrades } = hre;
  const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);
  const currentImplementation = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
  const newImplem = await ethers.getContractFactory('contracts/KMSVerifier.sol:KMSVerifier', deployer);
  const parsedEnv = readHostEnv();
  const proxyAddress = parsedEnv.KMS_VERIFIER_CONTRACT_ADDRESS;
  const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);
  const verifyingContractSource = getRequiredEnvVar('DECRYPTION_ADDRESS');
  const chainIDSource = +getRequiredEnvVar('CHAIN_ID_GATEWAY');
  await hre.run('task:assertProtocolConfigReady');
  await upgrades.upgradeProxy(proxy, newImplem, {
    call: {
      fn: 'initializeFromEmptyProxy',
      args: [verifyingContractSource, chainIDSource],
    },
  });
  console.log('KMSVerifier code set successfully at address:', proxyAddress);
});

////////////////////////////////////////////////////////////////////////////////
// InputVerifier
////////////////////////////////////////////////////////////////////////////////

task('task:deployInputVerifier')
  .addOptionalParam(
    'useAddress',
    'Use addresses instead of private keys env variables for kms signers',
    true,
    types.boolean,
  )
  .setAction(async function (taskArguments: TaskArguments, { ethers, upgrades }) {
    const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
    const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);
    const currentImplementation = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
    const newImplem = await ethers.getContractFactory('./contracts/InputVerifier.sol:InputVerifier', deployer);
    const parsedEnv = readHostEnv();
    const proxyAddress = parsedEnv.INPUT_VERIFIER_CONTRACT_ADDRESS;
    const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);
    const verifyingContractSource = getRequiredEnvVar('INPUT_VERIFICATION_ADDRESS');
    const chainIDSource = +getRequiredEnvVar('CHAIN_ID_GATEWAY');
    const initialThreshold = +getRequiredEnvVar('COPROCESSOR_THRESHOLD');

    let initialSigners: string[] = [];
    const numSigners = getRequiredEnvVar('NUM_COPROCESSORS');
    for (let idx = 0; idx < +numSigners; idx++) {
      if (!taskArguments.useAddress) {
        const privKeySigner = getRequiredEnvVar(`PRIVATE_KEY_COPROCESSOR_ACCOUNT_${idx}`);
        const inputSigner = new ethers.Wallet(privKeySigner).connect(ethers.provider);
        initialSigners.push(inputSigner.address);
      } else {
        const inputSignerAddress = getRequiredEnvVar(`COPROCESSOR_SIGNER_ADDRESS_${idx}`);
        initialSigners.push(inputSignerAddress);
      }
    }

    await upgrades.upgradeProxy(proxy, newImplem, {
      call: {
        fn: 'initializeFromEmptyProxy',
        args: [verifyingContractSource, chainIDSource, initialSigners, initialThreshold],
      },
    });
    console.log('InputVerifier code set successfully at address:', proxyAddress);
    console.log(
      `${numSigners} Coprocessor signers were added to InputVerifier at initialization, list of Coprocessor signers is:`,
      initialSigners,
    );
    console.log('Threshold for InputVerifier is:', initialThreshold);
  });

////////////////////////////////////////////////////////////////////////////////
// HCULimit
////////////////////////////////////////////////////////////////////////////////

task('task:deployHCULimit').setAction(async function (taskArguments: TaskArguments, { ethers, upgrades }) {
  const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);
  const currentImplementation = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
  const newImplem = await ethers.getContractFactory('HCULimit', deployer);
  const parsedEnv = readHostEnv();
  const proxyAddress = parsedEnv.HCU_LIMIT_CONTRACT_ADDRESS;
  const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);
  await upgrades.upgradeProxy(proxy, newImplem, {
    call: { fn: 'initializeFromEmptyProxy', args: [BigInt('281474976710655'), BigInt('5000000'), BigInt('20000000')] },
  });
  console.log('HCULimit code set successfully at address:', proxyAddress);
});

////////////////////////////////////////////////////////////////////////////////
// PauserSet
////////////////////////////////////////////////////////////////////////////////

task('task:deployPauserSet').setAction(async function (_, hre) {
  // Get a deployer wallet
  const deployerPrivateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new Wallet(deployerPrivateKey).connect(hre.ethers.provider);

  console.log('Deploying PauserSet...');
  const pauserSetFactory = await hre.ethers.getContractFactory('PauserSet', deployer);
  const pauserSet = await pauserSetFactory.deploy();
  const pauserSetAddress = await pauserSet.getAddress();

  await hre.run('task:setPauserSetAddress', {
    address: pauserSetAddress,
  });
});

////////////////////////////////////////////////////////////////////////////////
// ConfidentialBridge
////////////////////////////////////////////////////////////////////////////////

task('task:deployBridge').setAction(async function (_, { ethers, upgrades }) {
  const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);

  const parsedEnv = readHostEnv();
  const proxyAddress = parsedEnv.CONFIDENTIAL_BRIDGE_CONTRACT_ADDRESS;
  if (!proxyAddress) {
    throw new Error(
      'CONFIDENTIAL_BRIDGE_CONTRACT_ADDRESS not found in addresses/.env.host. ' +
        'Run task:deployEmptyUUPSProxies first.',
    );
  }
  // task:deployEmptyUUPSProxies only provisions a bridge empty proxy when a real LayerZero endpoint
  // is configured; otherwise it pins the null address. A null address means "no bridge on this host",
  // so there is nothing to upgrade — skip instead of failing the forceImport on the zero address.
  if (proxyAddress === ethers.ZeroAddress) {
    console.log(
      '[task:deployBridge] CONFIDENTIAL_BRIDGE_CONTRACT_ADDRESS is the null address; ' +
        'no ConfidentialBridge proxy was provisioned (LZ_ENDPOINT_ADDRESS unset or has no code). Skipping bridge upgrade.',
    );
    return;
  }

  const lzEndpoint = getRequiredEnvVar('LZ_ENDPOINT_ADDRESS');
  if (!ethers.isAddress(lzEndpoint)) {
    throw new Error(`LZ_ENDPOINT_ADDRESS is not a valid address: ${lzEndpoint}`);
  }

  const currentImplementation = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
  const newImplem = await ethers.getContractFactory('ConfidentialBridge', deployer);

  const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);
  await upgrades.upgradeProxy(proxy, newImplem, {
    constructorArgs: [lzEndpoint],
    // - constructor / state-variable-immutable: LayerZero's `OAppCoreUpgradeable`
    //   stores the endpoint as an immutable in the implementation's constructor.
    // - missing-initializer-call: `__OApp(Sender|Receiver)_init_unchained()` are no-ops
    //   and we call them explicitly; OZ's static validator doesn't recognize the
    //   `_unchained` variants as satisfying the `_init` requirement.
    unsafeAllow: ['constructor', 'state-variable-immutable', 'missing-initializer-call'],
    call: { fn: 'initializeFromEmptyProxy', args: [[], []] },
  });
  console.log(`ConfidentialBridge upgraded at ${proxyAddress} (lzEndpoint=${lzEndpoint})`);
});

////////////////////////////////////////////////////////////////////////////////
// ProtocolConfig helpers
////////////////////////////////////////////////////////////////////////////////

export function buildKmsNodeParams(): {
  txSenderAddress: string;
  signerAddress: string;
  ipAddress: string;
  storageUrl: string;
  partyId: number;
  mpcIdentity: string;
  caCert: string;
  storagePrefix: string;
}[] {
  const numNodes = +getRequiredEnvVar('NUM_KMS_NODES');
  const nodes: {
    txSenderAddress: string;
    signerAddress: string;
    ipAddress: string;
    storageUrl: string;
    partyId: number;
    mpcIdentity: string;
    caCert: string;
    storagePrefix: string;
  }[] = [];
  for (let idx = 0; idx < numNodes; idx++) {
    const txSenderAddress = getRequiredEnvVar(`KMS_TX_SENDER_ADDRESS_${idx}`);
    const signerAddress = getRequiredEnvVar(`KMS_SIGNER_ADDRESS_${idx}`);
    const ipAddress = getRequiredEnvVar(`KMS_NODE_IP_${idx}`);
    const storageUrl = getRequiredEnvVar(`KMS_NODE_STORAGE_URL_${idx}`);
    const partyId = +getRequiredEnvVar(`KMS_NODE_PARTY_ID_${idx}`);
    const mpcIdentity = getRequiredEnvVar(`KMS_NODE_MPC_IDENTITY_${idx}`);
    const caCert = getRequiredEnvVar(`KMS_NODE_CA_CERT_${idx}`);
    const storagePrefix = getRequiredEnvVar(`KMS_NODE_STORAGE_PREFIX_${idx}`);
    nodes.push({
      txSenderAddress,
      signerAddress,
      ipAddress,
      storageUrl,
      partyId,
      mpcIdentity,
      caCert,
      storagePrefix,
    });
  }
  return nodes;
}

export function buildKmsThresholds() {
  return {
    publicDecryption: +getRequiredEnvVar('PUBLIC_DECRYPTION_THRESHOLD'),
    userDecryption: +getRequiredEnvVar('USER_DECRYPTION_THRESHOLD'),
    kmsGen: +getRequiredEnvVar('KMS_GEN_THRESHOLD'),
    mpc: +getRequiredEnvVar('MPC_THRESHOLD'),
  };
}

export function buildPcrValues(): { pcr0: string; pcr1: string; pcr2: string }[] {
  return JSON.parse(getRequiredEnvVar('KMS_PCR_VALUES'));
}

export function buildProtocolConfigContextArgs(): [ReturnType, ReturnType, string, ReturnType] {
  return [buildKmsNodeParams(), buildKmsThresholds(), getRequiredEnvVar('KMS_SOFTWARE_VERSION'), buildPcrValues()];
}

// Reinitialize takes the same argument tuple as context construction; alias to avoid drift.
export const buildProtocolConfigReinitializeArgs = buildProtocolConfigContextArgs;

task('task:assertProtocolConfigReady').setAction(async function (_, hre) {
  const parsedEnv = readHostEnv();
  const protocolConfigAddress = parsedEnv.PROTOCOL_CONFIG_CONTRACT_ADDRESS;

  try {
    await assertContractMatchesVersionPrefix(hre, protocolConfigAddress, 'ProtocolConfig');
  } catch (err) {
    throw new Error(`Cannot deploy KMSVerifier: ${formatError(err)}`);
  }

  const protocolConfig = new hre.ethers.Contract(
    protocolConfigAddress,
    [
      'function getCurrentKmsContextId() view returns (uint256)',
      'function getKmsSignersForContext(uint256) view returns (address[])',
      'function getPublicDecryptionThresholdForContext(uint256) view returns (uint256)',
    ],
    hre.ethers.provider,
  );

  let currentKmsContextId: bigint;
  try {
    currentKmsContextId = await protocolConfig.getCurrentKmsContextId();
  } catch (err) {
    throw new Error(
      `Cannot deploy KMSVerifier: ProtocolConfig at ${protocolConfigAddress} is not initialized (reading active context reverted: ${formatError(
        err,
      )}).`,
    );
  }

  if (currentKmsContextId === 0n) {
    throw new Error(
      `Cannot deploy KMSVerifier: ProtocolConfig at ${protocolConfigAddress} has no active KMS context (currentKmsContextId=0).`,
    );
  }

  try {
    const [signers, threshold] = await Promise.all([
      protocolConfig.getKmsSignersForContext(currentKmsContextId),
      protocolConfig.getPublicDecryptionThresholdForContext(currentKmsContextId),
    ]);
    if (signers.length === 0) {
      throw new Error('current context has no KMS signers');
    }
    if (threshold === 0n) {
      throw new Error('current context public decryption threshold is zero');
    }
  } catch (err) {
    throw new Error(
      `Cannot deploy KMSVerifier: ProtocolConfig at ${protocolConfigAddress} has unreadable active context ${currentKmsContextId.toString()}: ${formatError(
        err,
      )}.`,
    );
  }
});

// Off-chain pre-flight that asserts no pending key management request on KMSGeneration.
//
// Confirms the address is actually KMSGeneration before reading the request counters, so a
// wrong code-bearing address cannot silently return a false green.
task('task:assertNoPendingKeyManagementRequest')
  .addOptionalParam(
    'address',
    'KMSGeneration proxy address. Falls back to env var then addresses/.env.host.',
    undefined,
    types.string,
  )
  .setAction(async function (taskArguments: TaskArguments, hre) {
    const kmsGenAddress: string | undefined =
      taskArguments.address ??
      process.env.KMS_GENERATION_CONTRACT_ADDRESS ??
      readExistingHostEnv().KMS_GENERATION_CONTRACT_ADDRESS;
    if (!kmsGenAddress) {
      throw new Error(
        'KMSGeneration address not resolved. Pass --address 0x…, or set KMS_GENERATION_CONTRACT_ADDRESS, or generate addresses/.env.host via a deploy task first.',
      );
    }

    await assertContractMatchesVersionPrefix(hre, kmsGenAddress, 'KMSGeneration');

    const kmsGen = await hre.ethers.getContractAt('KMSGeneration', kmsGenAddress);
    const readKmsStatusView = async <T>(viewLabel: string, read: () => Promise): Promise => {
      try {
        return await read();
      } catch (err) {
        const wrapped = new Error(
          `Failed reading ${viewLabel} from KMSGeneration at ${kmsGenAddress}. Re-check the configured address and confirm this KMSGeneration version exposes ${viewLabel}. (${formatError(
            err,
          )})`,
        ) as Error & { cause?: unknown };
        wrapped.cause = err;
        throw wrapped;
      }
    };

    const [keyCounter, crsCounter] = await Promise.all([
      readKmsStatusView('getKeyCounter()', () => kmsGen.getKeyCounter()),
      readKmsStatusView('getCrsCounter()', () => kmsGen.getCrsCounter()),
    ]);

    const [keyDone, crsDone] = await Promise.all([
      keyCounter === KEY_COUNTER_BASE
        ? Promise.resolve(true)
        : readKmsStatusView(`isRequestDone(${keyCounter.toString()})`, () => kmsGen.isRequestDone(keyCounter)),
      crsCounter === CRS_COUNTER_BASE
        ? Promise.resolve(true)
        : readKmsStatusView(`isRequestDone(${crsCounter.toString()})`, () => kmsGen.isRequestDone(crsCounter)),
    ]);

    if (keyCounter !== KEY_COUNTER_BASE && !keyDone) {
      throw new Error(
        `Keygen pending on ${kmsGenAddress}: keyCounter=${keyCounter} has not completed (isRequestDone=false). Complete or abort before proposing a new key management request.`,
      );
    }

    if (crsCounter !== CRS_COUNTER_BASE && !crsDone) {
      throw new Error(
        `CRS generation pending on ${kmsGenAddress}: crsCounter=${crsCounter} has not completed (isRequestDone=false). Complete or abort before proposing a new key management request.`,
      );
    }

    console.log(`No pending key management requests on ${kmsGenAddress}.`);
  });

////////////////////////////////////////////////////////////////////////////////
// ProtocolConfig
////////////////////////////////////////////////////////////////////////////////

task('task:deployProtocolConfig').setAction(async function (_, hre) {
  const { ethers, upgrades } = hre;
  const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);
  const currentImplementation = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
  const initArgs = buildProtocolConfigContextArgs();
  const newImplem = await ethers.getContractFactory('ProtocolConfig', deployer);
  const parsedEnv = readHostEnv();
  const proxyAddress = parsedEnv.PROTOCOL_CONFIG_CONTRACT_ADDRESS;
  const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);

  await upgrades.upgradeProxy(proxy, newImplem, {
    call: {
      fn: 'initializeFromEmptyProxy',
      args: initArgs,
    },
  });
  // On interval-mining networks, upgradeProxy can return before the tx is mined.
  await waitForTaskReady(hre, 'task:assertProtocolConfigReady');
  console.log('ProtocolConfig code set successfully at address:', proxyAddress);
});

// DAO path for initializing a non-canonical ProtocolConfig replica from the canonical chain
// (Ethereum). Consumes a reviewed task:exportCanonicalProtocolConfig artifact — not a live RPC
// read — so the DAO executes exactly the state its signers reproduced and diffed. Devnet
// equivalent: task:deployProtocolConfigFromCanonical.
task(
  'task:prepareDeployProtocolConfigFromCanonical',
  'Deploys a ProtocolConfig implementation and prints DAO upgrade calldata from a reviewed canonical snapshot artifact',
)
  .addParam(
    'snapshot',
    'Path to the reviewed task:exportCanonicalProtocolConfig artifact to encode into the DAO payload.',
    undefined,
    types.string,
  )
  .addOptionalParam(
    'verifyContract',
    'Verify new implementation on Etherscan (for eg if deploying on Sepolia or Mainnet)',
    true,
    types.boolean,
  )
  .setAction(async function ({ snapshot: snapshotPath, verifyContract }, hre) {
    const parsedEnv = readHostEnv();
    const proxyAddress = parsedEnv.PROTOCOL_CONFIG_CONTRACT_ADDRESS;
    // The bootstrap task may have updated addresses/FHEVMHostAddresses.sol, so rebuild.
    await hre.run('compile:specific', { contract: 'contracts' });
    const snapshot = parseSnapshotArtifact(fs.readFileSync(snapshotPath, 'utf-8'));
    const preparedUpgrade = await buildCanonicalUpgradeProposal(hre, { snapshot, proxyAddress });

    printUpgradeProposal(preparedUpgrade);
    if (verifyContract) {
      await verifyProposalImplementation(hre, preparedUpgrade, 'contracts/ProtocolConfig.sol:ProtocolConfig');
    }
    return preparedUpgrade;
  });

// Initializes the local (non-canonical) ProtocolConfig replica from the canonical chain's current
// KMS context — from a reviewed export artifact (--snapshot) or a live block-pinned RPC read.
task(
  'task:deployProtocolConfigFromCanonical',
  "Upgrades the existing ProtocolConfig proxy from the canonical chain's state (reviewed snapshot artifact, or live read).",
)
  .addOptionalParam(
    'snapshot',
    'Path to a reviewed task:exportCanonicalProtocolConfig artifact to apply. When set, canonical RPC access is not needed and exactly the reviewed state is deployed.',
    undefined,
    types.string,
  )
  .addOptionalParam(
    'canonicalRpcUrl',
    'RPC URL of the canonical host chain to read the current ProtocolConfig state from. Required without --snapshot.',
    undefined,
    types.string,
  )
  .addOptionalParam(
    'canonicalProtocolConfigAddress',
    'Address of the ProtocolConfig contract on the canonical host chain. Required without --snapshot.',
    undefined,
    types.string,
  )
  .setAction(async function (
    {
      snapshot: snapshotPath,
      canonicalRpcUrl,
      canonicalProtocolConfigAddress,
    }: { snapshot?: string; canonicalRpcUrl?: string; canonicalProtocolConfigAddress?: string },
    hre,
  ) {
    if (!snapshotPath && !(canonicalRpcUrl && canonicalProtocolConfigAddress)) {
      throw new Error(
        'Pass either --snapshot <artifact.json> (reviewed export) or both --canonical-rpc-url and --canonical-protocol-config-address (live read).',
      );
    }

    // ProtocolConfig embeds aclAdd from addresses/FHEVMHostAddresses.sol at compile time; a stale
    // artifact would deploy bytecode authorized against the wrong ACL (same as FromMigration).
    await hre.run('compile:specific', { contract: 'contracts' });
    const parsedEnv = readHostEnv();
    const secondaryProxyAddress = parsedEnv.PROTOCOL_CONFIG_CONTRACT_ADDRESS;

    let snapshot: CanonicalSnapshot;
    if (snapshotPath) {
      console.log(`Applying reviewed canonical snapshot from ${snapshotPath}.`);
      snapshot = parseSnapshotArtifact(fs.readFileSync(snapshotPath, 'utf-8'));
    } else {
      snapshot = await readCanonicalSnapshot(hre, {
        canonicalProvider: new hre.ethers.JsonRpcProvider(canonicalRpcUrl),
        canonicalProtocolConfigAddress: canonicalProtocolConfigAddress as string,
      });
    }

    // Same prepare step as the DAO path (task:prepareDeployProtocolConfigFromCanonical), then
    // execute the produced payload directly: devnet runs byte-identical calldata to what the DAO
    // would sign.
    const prepared = await buildCanonicalUpgradeProposal(hre, { snapshot, proxyAddress: secondaryProxyAddress });
    await executeUpgradeProposal(hre, prepared);

    // On interval-mining networks, upgradeProxy can return before the tx is mined.
    await waitForTaskReady(hre, 'task:assertProtocolConfigReady');
    console.log(
      `ProtocolConfig code set successfully at ${secondaryProxyAddress}, mirroring canonical chain ${snapshot.canonicalChainId} context ${snapshot.currentKmsContextId} epoch ${snapshot.currentEpochId} (block ${snapshot.blockNumber}) with ${snapshot.kmsNodes.length} KMS nodes.`,
    );
  });

// Reads the canonical ProtocolConfig context at a pinned block and writes a JSON snapshot, without
// deploying anything. DAO signers re-run this at the same block and diff the snapshot against
// canonical before accepting secondary-host ownership.
task(
  'task:exportCanonicalProtocolConfig',
  'Exports the canonical ProtocolConfig KMS context to a JSON snapshot for DAO review.',
)
  .addParam(
    'canonicalRpcUrl',
    'RPC URL of the canonical host chain to read ProtocolConfig from.',
    undefined,
    types.string,
  )
  .addParam(
    'canonicalProtocolConfigAddress',
    'Address of the ProtocolConfig contract on the canonical host chain.',
    undefined,
    types.string,
  )
  .addOptionalParam(
    'blockNumber',
    "Canonical block height to pin the snapshot to. Defaults to the latest finalized block; pass the artifact's blockNumber to reproduce a prior export for DAO review.",
    undefined,
    types.int,
  )
  .addOptionalParam('out', 'Path to write the snapshot JSON.', 'canonical-protocol-config-snapshot.json', types.string)
  .setAction(async function (
    {
      canonicalRpcUrl,
      canonicalProtocolConfigAddress,
      blockNumber,
      out,
    }: { canonicalRpcUrl: string; canonicalProtocolConfigAddress: string; blockNumber?: number; out: string },
    hre,
  ) {
    // readCanonicalSnapshot needs the ProtocolConfig artifact for its ABI; compile so the export
    // also works from a clean checkout.
    await hre.run('compile:specific', { contract: 'contracts' });
    const canonicalProvider = new hre.ethers.JsonRpcProvider(canonicalRpcUrl);
    const snapshot = await readCanonicalSnapshot(hre, {
      canonicalProvider,
      canonicalProtocolConfigAddress,
      blockNumber,
    });

    const artifact = buildSnapshotArtifact(snapshot, canonicalProtocolConfigAddress);
    fs.writeFileSync(out, JSON.stringify(artifact, null, 2));
    console.log(
      `Canonical ProtocolConfig snapshot written to ${out}: chain ${snapshot.canonicalChainId}, block ${snapshot.blockNumber}, context ${snapshot.currentKmsContextId}, epoch ${snapshot.currentEpochId}, ${snapshot.kmsNodes.length} KMS nodes.`,
    );
    return artifact;
  });

////////////////////////////////////////////////////////////////////////////////
// KMSGeneration (host-side)
////////////////////////////////////////////////////////////////////////////////

task('task:deployKMSGeneration').setAction(async function (taskArguments: TaskArguments, { ethers, upgrades }) {
  const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);
  const currentImplementation = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
  const newImplem = await ethers.getContractFactory('KMSGeneration', deployer);
  const parsedEnv = readHostEnv();
  const proxyAddress = parsedEnv.KMS_GENERATION_CONTRACT_ADDRESS;
  const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);
  await upgrades.upgradeProxy(proxy, newImplem, {
    call: { fn: 'initializeFromEmptyProxy' },
  });
  console.log('KMSGeneration code set successfully at address:', proxyAddress);
});

////////////////////////////////////////////////////////////////////////////////
// Setup ACL Address
////////////////////////////////////////////////////////////////////////////////

task('task:setACLAddress')
  .addParam('address', 'The address of the contract')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    ensureAddressesDirectoryExists();
    const content = `ACL_CONTRACT_ADDRESS=${taskArguments.address}\n`;
    try {
      writeHostEnvLine(content, 'w');
      console.log(`ACL address ${taskArguments.address} written successfully!`);
    } catch (err) {
      throw new Error(`Failed to write ACL address: ${String(err)}`);
    }

    const solidityTemplate = `// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

address constant aclAdd = ${taskArguments.address};\n`;

    try {
      writeHostAddressesSol(solidityTemplate, 'w');
      console.log(`${HOST_ADDRESSES_FILE} generated successfully!`);
    } catch (error) {
      throw new Error(`Failed to write ${HOST_ADDRESSES_FILE}: ${String(error)}`);
    }
  });

////////////////////////////////////////////////////////////////////////////////
// Setup FHEVMExecutor Address
////////////////////////////////////////////////////////////////////////////////

task('task:setFHEVMExecutorAddress')
  .addParam('address', 'The address of the contract')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    ensureAddressesDirectoryExists();
    const content = `FHEVM_EXECUTOR_CONTRACT_ADDRESS=${taskArguments.address}\n`;
    try {
      writeHostEnvLine(content, 'a');
      console.log(`FHEVMExecutor address ${taskArguments.address} written successfully!`);
    } catch (err) {
      throw new Error(`Failed to write FHEVMExecutor address: ${String(err)}`);
    }

    const solidityTemplate = `
address constant fhevmExecutorAdd = ${taskArguments.address};\n`;

    try {
      writeHostAddressesSol(solidityTemplate, 'a');
      console.log(`${HOST_ADDRESSES_FILE} appended with fhevmExecutorAdd successfully!`);
    } catch (error) {
      throw new Error(`Failed to write ${HOST_ADDRESSES_FILE}: ${String(error)}`);
    }
  });

////////////////////////////////////////////////////////////////////////////////
// Setup KMSVerifier Address
////////////////////////////////////////////////////////////////////////////////

task('task:setKMSVerifierAddress')
  .addParam('address', 'The address of the contract')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    ensureAddressesDirectoryExists();
    const content = `KMS_VERIFIER_CONTRACT_ADDRESS=${taskArguments.address}\n`;
    try {
      writeHostEnvLine(content, 'a');
      console.log(`KMSVerifier address ${taskArguments.address} written successfully!`);
    } catch (err) {
      throw new Error(`Failed to write KMSVerifier address: ${String(err)}`);
    }

    const solidityTemplate = `
address constant kmsVerifierAdd = ${taskArguments.address};\n`;

    try {
      writeHostAddressesSol(solidityTemplate, 'a');
      console.log(`${HOST_ADDRESSES_FILE} appended with kmsVerifierAdd successfully!`);
    } catch (error) {
      throw new Error(`Failed to write ${HOST_ADDRESSES_FILE}: ${String(error)}`);
    }
  });

////////////////////////////////////////////////////////////////////////////////
// Setup InputVerifier Address
////////////////////////////////////////////////////////////////////////////////

task('task:setInputVerifierAddress')
  .addParam('address', 'The address of the contract')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    ensureAddressesDirectoryExists();
    // this script also computes the coprocessor address from its private key
    const content = `INPUT_VERIFIER_CONTRACT_ADDRESS=${taskArguments.address}\n`;
    try {
      writeHostEnvLine(content, 'a');
      console.log(`InputVerifier address ${taskArguments.address} written successfully!`);
    } catch (err) {
      throw new Error(`Failed to write InputVerifier address: ${String(err)}`);
    }

    const solidityTemplate = `
address constant inputVerifierAdd = ${taskArguments.address};\n`;

    try {
      writeHostAddressesSol(solidityTemplate, 'a');
      console.log(`${HOST_ADDRESSES_FILE} appended with inputVerifierAdd successfully!`);
    } catch (error) {
      throw new Error(`Failed to write ${HOST_ADDRESSES_FILE}: ${String(error)}`);
    }
  });

////////////////////////////////////////////////////////////////////////////////
// Setup HCULimit Address
////////////////////////////////////////////////////////////////////////////////

task('task:setHCULimitAddress')
  .addParam('address', 'The address of the contract')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    ensureAddressesDirectoryExists();
    const content = `HCU_LIMIT_CONTRACT_ADDRESS=${taskArguments.address}\n`;
    try {
      writeHostEnvLine(content, 'a');
      console.log(`HCULimit address ${taskArguments.address} written successfully!`);
    } catch (err) {
      throw new Error(`Failed to write HCULimit address: ${String(err)}`);
    }

    const solidityTemplate = `
address constant hcuLimitAdd = ${taskArguments.address};\n`;

    try {
      writeHostAddressesSol(solidityTemplate, 'a');
      console.log(`${HOST_ADDRESSES_FILE} appended with hcuLimitAdd successfully!`);
    } catch (error) {
      throw new Error(`Failed to write ${HOST_ADDRESSES_FILE}: ${String(error)}`);
    }
  });

////////////////////////////////////////////////////////////////////////////////
// Setup PauserSet Address
////////////////////////////////////////////////////////////////////////////////

task('task:setPauserSetAddress')
  .addParam('address', 'The address of the contract')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    ensureAddressesDirectoryExists();
    const content = `PAUSER_SET_CONTRACT_ADDRESS=${taskArguments.address}\n`;
    try {
      writeHostEnvLine(content, 'a');
      console.log(`PauserSet address ${taskArguments.address} written successfully!`);
    } catch (err) {
      throw new Error(`Failed to write PauserSet address: ${String(err)}`);
    }

    const solidityTemplate = `
address constant pauserSetAdd = ${taskArguments.address};\n`;

    try {
      writeHostAddressesSol(solidityTemplate, 'a');
      console.log(`${HOST_ADDRESSES_FILE} appended with pauserSetAdd successfully!`);
    } catch (error) {
      throw new Error(`Failed to write ${HOST_ADDRESSES_FILE}: ${String(error)}`);
    }
  });

////////////////////////////////////////////////////////////////////////////////
// Setup ProtocolConfig Address
////////////////////////////////////////////////////////////////////////////////

task('task:setProtocolConfigAddress')
  .addParam('address', 'The address of the contract')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    ensureAddressesDirectoryExists();
    const content = `PROTOCOL_CONFIG_CONTRACT_ADDRESS=${taskArguments.address}\n`;
    try {
      writeHostEnvLine(content, 'a');
      console.log(`ProtocolConfig address ${taskArguments.address} written successfully!`);
    } catch (err) {
      throw new Error(`Failed to write ProtocolConfig address: ${String(err)}`);
    }

    const solidityTemplate = `
address constant protocolConfigAdd = ${taskArguments.address};\n`;

    try {
      writeHostAddressesSol(solidityTemplate, 'a');
      console.log(`${HOST_ADDRESSES_FILE} appended with protocolConfigAdd successfully!`);
    } catch (error) {
      throw new Error(`Failed to write ${HOST_ADDRESSES_FILE}: ${String(error)}`);
    }
  });

////////////////////////////////////////////////////////////////////////////////
// Setup KMSGeneration Address
////////////////////////////////////////////////////////////////////////////////

task('task:setKMSGenerationAddress')
  .addParam('address', 'The address of the contract')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    ensureAddressesDirectoryExists();
    const content = `KMS_GENERATION_CONTRACT_ADDRESS=${taskArguments.address}\n`;
    try {
      writeHostEnvLine(content, 'a');
      console.log(`KMSGeneration address ${taskArguments.address} written successfully!`);
    } catch (err) {
      throw new Error(`Failed to write KMSGeneration address: ${String(err)}`);
    }

    const solidityTemplate = `
address constant kmsGenerationAdd = ${taskArguments.address};\n`;

    try {
      writeHostAddressesSol(solidityTemplate, 'a');
      console.log(`${HOST_ADDRESSES_FILE} appended with kmsGenerationAdd successfully!`);
    } catch (error) {
      throw new Error(`Failed to write ${HOST_ADDRESSES_FILE}: ${String(error)}`);
    }
  });

////////////////////////////////////////////////////////////////////////////////
// Setup ConfidentialBridge Address
////////////////////////////////////////////////////////////////////////////////

task('task:setBridgeAddress')
  .addParam('address', 'The address of the contract')
  .setAction(async function (taskArguments: TaskArguments) {
    ensureAddressesDirectoryExists();
    const content = `CONFIDENTIAL_BRIDGE_CONTRACT_ADDRESS=${taskArguments.address}\n`;
    try {
      writeHostEnvLine(content, 'a');
      console.log(`ConfidentialBridge address ${taskArguments.address} written successfully!`);
    } catch (err) {
      throw new Error(`Failed to write ConfidentialBridge address: ${String(err)}`);
    }

    const solidityTemplate = `
address constant confidentialBridgeAdd = ${taskArguments.address};\n`;
    try {
      writeHostAddressesSol(solidityTemplate, 'a');
      console.log(`${HOST_ADDRESSES_FILE} appended with confidentialBridgeAdd successfully!`);
    } catch (error) {
      throw new Error(`Failed to write ${HOST_ADDRESSES_FILE}: ${String(error)}`);
    }
  });

////////////////////////////////////////////////////////////////////////////////
// Set the bridge-specific `dstChainId`
////////////////////////////////////////////////////////////////////////////////

task('task:setDstChainId')
  .addParam('bridgeAddress', 'The address of the contract')
  .addParam('remoteEid', 'The remote EID')
  .addParam('remoteChainId', 'The remote chain ID')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const deployerPrivateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
    const deployer = new Wallet(deployerPrivateKey).connect(ethers.provider);
    const confidentialBridgeAddress = taskArguments.bridgeAddress;
    const confidentialBridge = await ethers.getContractAt('ConfidentialBridge', confidentialBridgeAddress, deployer);
    await confidentialBridge.setDstChainId(taskArguments.remoteEid, taskArguments.remoteChainId);
    console.log('setDstChainId done successfully!');
  });

////////////////////////////////////////////////////////////////////////////////
// Set custom per-dstEid `lzReceive` gas overrides
////////////////////////////////////////////////////////////////////////////////

task('task:setLzReceiveBaseGas')
  .addParam('bridgeAddress', 'The address of the contract')
  .addParam('remoteEid', 'The remote EID')
  .addParam('baseGas', 'The custom base lzReceive gas (0 to clear and fall back to the default)')
  .setAction(async function (taskArguments: TaskArguments, { ethers, network }) {
    const deployerPrivateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
    const deployer = new Wallet(deployerPrivateKey).connect(ethers.provider);
    const confidentialBridgeAddress = taskArguments.bridgeAddress;
    const confidentialBridge = await ethers.getContractAt('ConfidentialBridge', confidentialBridgeAddress, deployer);
    const oldBaseGas = await confidentialBridge.getLzReceiveBaseGas(taskArguments.remoteEid);
    const receipt = await confidentialBridge.setLzReceiveBaseGas(taskArguments.remoteEid, taskArguments.baseGas);
    await receipt.wait(1);
    const newBaseGas = await confidentialBridge.getLzReceiveBaseGas(taskArguments.remoteEid);
    console.log(
      `setLzReceiveBaseGas done on network "${network.name}" for bridge ${confidentialBridgeAddress} ` +
        `(remoteEid=${
          taskArguments.remoteEid
        }): effective base gas ${oldBaseGas.toString()} -> ${newBaseGas.toString()}`,
    );
  });

task('task:setLzReceivePerHandleGas')
  .addParam('bridgeAddress', 'The address of the contract')
  .addParam('remoteEid', 'The remote EID')
  .addParam('perHandleGas', 'The custom per-handle lzReceive gas (0 to clear and fall back to the default)')
  .setAction(async function (taskArguments: TaskArguments, { ethers, network }) {
    const deployerPrivateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
    const deployer = new Wallet(deployerPrivateKey).connect(ethers.provider);
    const confidentialBridgeAddress = taskArguments.bridgeAddress;
    const confidentialBridge = await ethers.getContractAt('ConfidentialBridge', confidentialBridgeAddress, deployer);
    const oldPerHandleGas = await confidentialBridge.getLzReceivePerHandleGas(taskArguments.remoteEid);
    const receipt = await confidentialBridge.setLzReceivePerHandleGas(
      taskArguments.remoteEid,
      taskArguments.perHandleGas,
    );
    await receipt.wait(1);
    const newPerHandleGas = await confidentialBridge.getLzReceivePerHandleGas(taskArguments.remoteEid);
    console.log(
      `setLzReceivePerHandleGas done on network "${network.name}" for bridge ${confidentialBridgeAddress} ` +
        `(remoteEid=${
          taskArguments.remoteEid
        }): effective per-handle gas ${oldPerHandleGas.toString()} -> ${newPerHandleGas.toString()}`,
    );
  });

task('task:setLzReceivePerPayloadByteGas')
  .addParam('bridgeAddress', 'The address of the contract')
  .addParam('remoteEid', 'The remote EID')
  .addParam('perPayloadByteGas', 'The custom per-payload-byte lzReceive gas (0 to clear and fall back to the default)')
  .setAction(async function (taskArguments: TaskArguments, { ethers, network }) {
    const deployerPrivateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
    const deployer = new Wallet(deployerPrivateKey).connect(ethers.provider);
    const confidentialBridgeAddress = taskArguments.bridgeAddress;
    const confidentialBridge = await ethers.getContractAt('ConfidentialBridge', confidentialBridgeAddress, deployer);
    const oldPerPayloadByteGas = await confidentialBridge.getLzReceivePerPayloadByteGas(taskArguments.remoteEid);
    const receipt = await confidentialBridge.setLzReceivePerPayloadByteGas(
      taskArguments.remoteEid,
      taskArguments.perPayloadByteGas,
    );
    await receipt.wait(1);
    const newPerPayloadByteGas = await confidentialBridge.getLzReceivePerPayloadByteGas(taskArguments.remoteEid);
    console.log(
      `setLzReceivePerPayloadByteGas done on network "${network.name}" for bridge ${confidentialBridgeAddress} ` +
        `(remoteEid=${
          taskArguments.remoteEid
        }): effective per-payload-byte gas ${oldPerPayloadByteGas.toString()} -> ${newPerPayloadByteGas.toString()}`,
    );
  });

////////////////////////////////////////////////////////////////////////////////
// Local LayerZero endpoint (e2e only)
////////////////////////////////////////////////////////////////////////////////

// Deploys a local EndpointV2Mock + LocalSimpleMessageLib for the e2e setup (real networks use
// the canonical LayerZero endpoint), giving task:deployBridge an LZ_ENDPOINT_ADDRESS.
task('task:deployLocalLzEndpoint')
  .addParam('eid', 'This chain LayerZero endpoint id', undefined, types.int)
  .addParam('remoteEid', 'Remote endpoint id to default the libraries for', undefined, types.int)
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    // LZ_ENDPOINT_DEPLOYER (anvil only): deploy from this impersonated account via a RAW JSON-RPC
    // signer. hardhat signs locally (the network is configured with a private key), so it can't send
    // from an unmanaged account; a raw provider makes anvil sign for the unlocked account instead.
    // Keeps the endpoint deploy off the host deployer's nonce, so host-contract addresses stay fixed.
    const impersonatedDeployer = process.env.LZ_ENDPOINT_DEPLOYER;
    let deployer: Signer;
    if (impersonatedDeployer) {
      const rpcProvider = new JsonRpcProvider(getRequiredEnvVar('RPC_URL'));
      await rpcProvider.send('anvil_impersonateAccount', [impersonatedDeployer]);
      await rpcProvider.send('anvil_setBalance', [impersonatedDeployer, '0x21e19e0c9bab2400000']); // 10000 ETH
      deployer = await rpcProvider.getSigner(impersonatedDeployer);
    } else {
      deployer = new Wallet(getRequiredEnvVar('DEPLOYER_PRIVATE_KEY')).connect(ethers.provider);
    }
    const deployerAddress = await deployer.getAddress();

    const endpoint = await (
      await ethers.getContractFactory('EndpointV2Mock', deployer)
    ).deploy(taskArguments.eid, deployerAddress);
    await endpoint.waitForDeployment();
    const endpointAddress = await endpoint.getAddress();

    const lib = await (await ethers.getContractFactory('LocalSimpleMessageLib', deployer)).deploy(endpointAddress);
    await lib.waitForDeployment();
    const libAddress = await lib.getAddress();

    await (await endpoint.registerLibrary(libAddress)).wait();
    await (await endpoint.setDefaultSendLibrary(taskArguments.remoteEid, libAddress)).wait();
    await (await endpoint.setDefaultReceiveLibrary(taskArguments.remoteEid, libAddress, 0)).wait();

    // Persist for task:deployBridge, both in-process and across runs (via addresses/.env.host).
    process.env.LZ_ENDPOINT_ADDRESS = endpointAddress;
    ensureAddressesDirectoryExists();
    writeHostEnvLine(`LZ_ENDPOINT_ADDRESS=${endpointAddress}\n`, 'a');

    console.log(`Local LZ endpoint deployed at ${endpointAddress} (message lib ${libAddress})`);
  });

// Sets the LayerZero peer on the local bridge for a remote chain (run once per chain).
task('task:setBridgePeer')
  .addParam('bridgeAddress', 'Local ConfidentialBridge address')
  .addParam('remoteEid', 'Remote endpoint id', undefined, types.int)
  .addParam('remoteBridge', 'Remote ConfidentialBridge address')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const deployer = new Wallet(getRequiredEnvVar('DEPLOYER_PRIVATE_KEY')).connect(ethers.provider);
    const bridge = await ethers.getContractAt('ConfidentialBridge', taskArguments.bridgeAddress, deployer);
    const peer = ethers.zeroPadValue(taskArguments.remoteBridge, 32);
    await (await bridge.setPeer(taskArguments.remoteEid, peer)).wait();
    console.log(`setPeer(${taskArguments.remoteEid}, ${taskArguments.remoteBridge}) done successfully!`);
  });
