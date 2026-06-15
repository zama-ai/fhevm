import { HardhatUpgrades } from '@openzeppelin/hardhat-upgrades';
import dotenv from 'dotenv';
import { Wallet } from 'ethers';
import fs from 'fs';
import { task, types } from 'hardhat/config';
import type { HardhatEthersHelpers, HardhatRuntimeEnvironment, TaskArguments } from 'hardhat/types';
import path from 'path';

import { CRS_COUNTER_BASE, KEY_COUNTER_BASE } from './utils/kmsGenerationConstants';
import { getRequiredEnvVar } from './utils/loadVariables';

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

export function readExistingHostEnv(): Record<string, string> {
  if (!fs.existsSync(HOST_ENV_FILE)) {
    return {};
  }
  return readHostEnv();
}

function formatError(err: unknown): string {
  return err instanceof Error ? err.message : String(err);
}

export async function waitForTaskReady(
  hre: HardhatRuntimeEnvironment,
  taskName: string,
  timeoutMs = 60_000,
): Promise<void> {
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

export async function assertContractMatchesVersionPrefix(
  hre: HardhatRuntimeEnvironment,
  address: string,
  versionPrefix: string,
): Promise<void> {
  const contract = new hre.ethers.Contract(
    address,
    ['function getVersion() view returns (string)'],
    hre.ethers.provider,
  );

  let version: string;
  try {
    version = await contract.getVersion();
  } catch (err) {
    throw new Error(
      `Contract at ${address} does not expose getVersion(); it is not a ${versionPrefix} proxy. (${formatError(err)})`,
    );
  }

  if (!version.startsWith(versionPrefix)) {
    throw new Error(`Contract at ${address} reports version "${version}"; expected "${versionPrefix} v…".`);
  }
}

////////////////////////////////////////////////////////////////////////////////
// All Host Contracts
////////////////////////////////////////////////////////////////////////////////

const PROTOCOL_CONFIG_SOURCES = ['fresh', 'migration'] as const;
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
    "How to initialize ProtocolConfig: 'fresh' (default) calls initializeFromEmptyProxy with env-driven KMS nodes/thresholds; 'migration' calls initializeFromMigration consuming MIGRATION_CONTEXT_ID / MIGRATION_KMS_NODES / MIGRATION_KMS_THRESHOLDS.",
    'fresh',
    types.string,
  )
  .setAction(async function (
    { withKmsGeneration, protocolConfigSource }: { withKmsGeneration: boolean; protocolConfigSource: string },
    hre,
  ) {
    if (!PROTOCOL_CONFIG_SOURCES.includes(protocolConfigSource as ProtocolConfigSource)) {
      throw new Error(
        `Invalid --protocol-config-source "${protocolConfigSource}". Allowed values: ${PROTOCOL_CONFIG_SOURCES.join(
          ', ',
        )}.`,
      );
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
    if (protocolConfigSource === 'migration') {
      await hre.run('task:deployProtocolConfigFromMigration');
    } else {
      await hre.run('task:deployProtocolConfig');
    }
    if (withKmsGeneration) {
      await hre.run('task:deployKMSGeneration');
    }
    await hre.run('task:deployKMSVerifier');
    await hre.run('task:deployInputVerifier');
    await hre.run('task:deployHCULimit');

    // ConfidentialBridge upgrade is opt-in via the LZ_ENDPOINT_ADDRESS env var.
    // When unset, the bridge stays at its empty-proxy stage and can be upgraded
    // later by running `task:deployBridge` once LZ_ENDPOINT_ADDRESS is configured.
    //
    // We also skip the bridge upgrade on the in-memory `hardhat` network: the
    // canonical LZ endpoint doesn't exist there, so initializeFromEmptyProxy →
    // __OAppCore_init → endpoint.setDelegate(...) would revert. Test fixtures
    // that need the bridge deploy their own proxies (see test/bridge/fixture.ts
    // and test/bridge/Bridge.t.sol::_deployBridgeProxy).
    if (!process.env.LZ_ENDPOINT_ADDRESS) {
      console.log(
        '[task:deployAllHostContracts] LZ_ENDPOINT_ADDRESS not set; ' +
          'ConfidentialBridge stays at its empty-proxy stage. Set LZ_ENDPOINT_ADDRESS and run task:deployBridge when ready.',
      );
    } else if (hre.network.name === 'hardhat') {
      console.log(
        "[task:deployAllHostContracts] Skipping bridge upgrade on the in-memory 'hardhat' network " +
          '(no LayerZero endpoint contract exists there). Run on a real network to upgrade the bridge.',
      );
    } else if ((await hre.ethers.provider.getCode(process.env.LZ_ENDPOINT_ADDRESS)) === '0x') {
      console.log(
        `[task:deployAllHostContracts] No contract deployed at LZ_ENDPOINT_ADDRESS (${process.env.LZ_ENDPOINT_ADDRESS}) ` +
          'on this network; ConfidentialBridge stays at its empty-proxy stage. ' +
          'Run task:deployBridge against a network with a real LayerZero endpoint to upgrade the bridge.',
      );
    } else {
      await hre.run('task:deployBridge');
    }

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
  .setAction(async function ({ withKmsGeneration }: { withKmsGeneration: boolean }, { ethers, upgrades, run }) {
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

    const confidentialBridgeAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
    await run('task:setBridgeAddress', { address: confidentialBridgeAddress });
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
  const lzEndpoint = getRequiredEnvVar('LZ_ENDPOINT_ADDRESS');
  if (!ethers.isAddress(lzEndpoint)) {
    throw new Error(`LZ_ENDPOINT_ADDRESS is not a valid address: ${lzEndpoint}`);
  }

  const currentImplementation = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
  const newImplem = await ethers.getContractFactory('ConfidentialBridge', deployer);
  const parsedEnv = readHostEnv();
  const proxyAddress = parsedEnv.CONFIDENTIAL_BRIDGE_CONTRACT_ADDRESS;
  if (!proxyAddress) {
    throw new Error(
      'CONFIDENTIAL_BRIDGE_CONTRACT_ADDRESS not found in addresses/.env.host. ' +
        'Run task:deployEmptyUUPSProxies first.',
    );
  }

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

export function buildKmsNodes(): {
  txSenderAddress: string;
  signerAddress: string;
  ipAddress: string;
  storageUrl: string;
}[] {
  const numNodes = +getRequiredEnvVar('NUM_KMS_NODES');
  const nodes: { txSenderAddress: string; signerAddress: string; ipAddress: string; storageUrl: string }[] = [];
  for (let idx = 0; idx < numNodes; idx++) {
    const txSenderAddress = getRequiredEnvVar(`KMS_TX_SENDER_ADDRESS_${idx}`);
    const signerAddress = getRequiredEnvVar(`KMS_SIGNER_ADDRESS_${idx}`);
    const ipAddress = process.env[`KMS_NODE_IP_${idx}`] || '';
    const storageUrl = getRequiredEnvVar(`KMS_NODE_STORAGE_URL_${idx}`);
    nodes.push({ txSenderAddress, signerAddress, ipAddress, storageUrl });
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
    ['function getCurrentKmsContextId() view returns (uint256)'],
    hre.ethers.provider,
  );

  let currentKmsContextId: bigint;
  try {
    currentKmsContextId = await protocolConfig.getCurrentKmsContextId();
  } catch (err) {
    throw new Error(
      `Cannot deploy KMSVerifier: ProtocolConfig at ${protocolConfigAddress} is not initialized (reading current context reverted: ${formatError(
        err,
      )}).`,
    );
  }

  if (currentKmsContextId === 0n) {
    throw new Error(
      `Cannot deploy KMSVerifier: ProtocolConfig at ${protocolConfigAddress} has no active KMS context (currentKmsContextId=0).`,
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
    const readKmsStatusView = async <T>(viewLabel: string, read: () => Promise<T>): Promise<T> => {
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

task('task:deployProtocolConfig').setAction(async function (_taskArguments: TaskArguments, hre) {
  const { ethers, upgrades } = hre;
  const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);
  const currentImplementation = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
  const newImplem = await ethers.getContractFactory('ProtocolConfig', deployer);
  const parsedEnv = readHostEnv();
  const proxyAddress = parsedEnv.PROTOCOL_CONFIG_CONTRACT_ADDRESS;
  const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);
  const initialKmsNodes = buildKmsNodes();
  const thresholds = buildKmsThresholds();

  await upgrades.upgradeProxy(proxy, newImplem, {
    call: {
      fn: 'initializeFromEmptyProxy',
      args: [initialKmsNodes, thresholds],
    },
  });
  // On interval-mining networks, upgradeProxy can return before the tx is mined.
  await waitForTaskReady(hre, 'task:assertProtocolConfigReady');
  console.log('ProtocolConfig code set successfully at address:', proxyAddress);
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
