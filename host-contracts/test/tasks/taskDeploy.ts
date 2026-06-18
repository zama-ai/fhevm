import { expect } from 'chai';
import fs from 'fs';
import { ethers, run } from 'hardhat';

import { CRS_COUNTER_BASE, KEY_COUNTER_BASE, PREP_KEYGEN_COUNTER_BASE } from '../../tasks/utils/kmsGenerationConstants';
import { getRequiredEnvVar } from '../../tasks/utils/loadVariables';
import type { KMSGeneration } from '../../types';
import { HOST_ADDRESSES_SOL_FILE, HOST_ENV_FILE, deployFreshKMSGenerationProxy, readHostAddress } from './taskHelpers';

const KMS_CONTEXT_COUNTER_BASE = 7n << 248n;

describe('task:deployAllHostContracts', function () {
  const canonicalSnapshotEnv = {
    CANONICAL_KMS_CONTEXT_ID: (KMS_CONTEXT_COUNTER_BASE + 1n).toString(),
    CANONICAL_SOURCE_CHAIN_ID: '1',
    CANONICAL_SOURCE_BLOCK_NUMBER: '12345678',
    CANONICAL_PROTOCOL_CONFIG_ADDRESS: '0x0000000000000000000000000000000000C0FFEE',
    KMS_PCR_VALUES: '[]',
    KMS_SOFTWARE_VERSION: 'kms-v1',
  };
  let previousEnv: Partial<Record<keyof typeof canonicalSnapshotEnv, string | undefined>>;
  let previousSolidityCoverage: string | undefined;
  let originalEnvHost: string;
  let originalAddressesSol: string;

  beforeEach(function () {
    previousEnv = {};
    previousSolidityCoverage = process.env.SOLIDITY_COVERAGE;
    // Snapshot .env.host: the fresh-deploy test rewrites PROTOCOL_CONFIG_CONTRACT_ADDRESS.
    originalEnvHost = fs.readFileSync(HOST_ENV_FILE, 'utf-8');
    // Snapshot FHEVMHostAddresses.sol: the withKmsGeneration=false path regenerates this shared
    // file without kmsGenerationAdd, which would break the subsequent `forge test` compile of
    // contracts that unconditionally import that constant.
    originalAddressesSol = fs.readFileSync(HOST_ADDRESSES_SOL_FILE, 'utf-8');
    for (const [key, value] of Object.entries(canonicalSnapshotEnv)) {
      const envKey = key as keyof typeof canonicalSnapshotEnv;
      previousEnv[envKey] = process.env[envKey];
      process.env[envKey] = value;
    }
  });

  afterEach(function () {
    for (const key of Object.keys(canonicalSnapshotEnv) as (keyof typeof canonicalSnapshotEnv)[]) {
      const previousValue = previousEnv[key];
      if (previousValue === undefined) {
        delete process.env[key];
      } else {
        process.env[key] = previousValue;
      }
    }
    if (previousSolidityCoverage === undefined) {
      delete process.env.SOLIDITY_COVERAGE;
    } else {
      process.env.SOLIDITY_COVERAGE = previousSolidityCoverage;
    }
    fs.writeFileSync(HOST_ENV_FILE, originalEnvHost);
    fs.writeFileSync(HOST_ADDRESSES_SOL_FILE, originalAddressesSol);
  });

  it('requires the KMSGeneration deployment role to be explicit', async function () {
    await expect(run('task:deployAllHostContracts')).to.be.rejectedWith(/withKmsGeneration/);
  });

  it('rejects an invalid --protocol-config-source value before mutating state', async function () {
    await expect(
      run('task:deployAllHostContracts', { withKmsGeneration: false, protocolConfigSource: 'bogus' }),
    ).to.be.rejectedWith(/Invalid --protocol-config-source "bogus"\. Allowed values: fresh, migration\./);
  });

  it('rejects migration source for non-canonical multichain deployments', async function () {
    await expect(
      run('task:deployAllHostContracts', { withKmsGeneration: false, protocolConfigSource: 'migration' }),
    ).to.be.rejectedWith(
      /--protocol-config-source migration is canonical-host only\. Use fresh multichain deployment with canonical snapshot env vars\./,
    );
  });

  it('deploys a fresh non-canonical host without a KMSGeneration proxy', async function () {
    process.env.SOLIDITY_COVERAGE = 'true';
    await run('task:deployAllHostContracts', { withKmsGeneration: false, protocolConfigSource: 'fresh' });

    const protocolConfig = await ethers.getContractAt(
      'ProtocolConfigMultichain',
      readHostAddress('PROTOCOL_CONFIG_CONTRACT_ADDRESS'),
    );

    expect(await protocolConfig.getVersion()).to.equal('ProtocolConfigMultichain v0.2.0');
    expect(await protocolConfig.getCurrentKmsContextId()).to.equal(
      BigInt(canonicalSnapshotEnv.CANONICAL_KMS_CONTEXT_ID),
    );
  });
});

describe('task:deployEmptyUUPSProxies', function () {
  it('requires the KMSGeneration deployment role to be explicit', async function () {
    await expect(run('task:deployEmptyUUPSProxies')).to.be.rejectedWith(/withKmsGeneration/);
  });
});

describe('task:assertNoPendingKeyManagementRequest', function () {
  const deployerPrivateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(deployerPrivateKey).connect(ethers.provider);
  let kmsGeneration: KMSGeneration;
  let kmsGenerationAddress: string;

  beforeEach(async function () {
    kmsGeneration = await deployFreshKMSGenerationProxy(deployer);
    kmsGenerationAddress = await kmsGeneration.getAddress();
  });

  it('passes for a freshly initialized proxy', async function () {
    await run('task:assertNoPendingKeyManagementRequest', { address: kmsGenerationAddress });
  });

  it('rejects a wrong code-bearing address via the getVersion identity check', async function () {
    const protocolConfigAddress = readHostAddress('PROTOCOL_CONFIG_CONTRACT_ADDRESS');

    await expect(
      run('task:assertNoPendingKeyManagementRequest', { address: protocolConfigAddress }),
    ).to.be.rejectedWith(
      `Contract at ${protocolConfigAddress} reports version "ProtocolConfig v0.2.0"; expected "KMSGeneration v…".`,
    );
  });

  it('rejects when keygen is pending', async function () {
    await kmsGeneration.keygen(0);

    await expect(run('task:assertNoPendingKeyManagementRequest', { address: kmsGenerationAddress })).to.be.rejectedWith(
      `Keygen pending on ${kmsGenerationAddress}: keyCounter=${KEY_COUNTER_BASE + 1n} has not completed (isRequestDone=false). Complete or abort before proposing a new key management request.`,
    );
  });

  it('rejects when CRS generation is pending', async function () {
    await kmsGeneration.crsgenRequest(4096, 0);

    await expect(run('task:assertNoPendingKeyManagementRequest', { address: kmsGenerationAddress })).to.be.rejectedWith(
      `CRS generation pending on ${kmsGenerationAddress}: crsCounter=${CRS_COUNTER_BASE + 1n} has not completed (isRequestDone=false). Complete or abort before proposing a new key management request.`,
    );
  });

  it('passes again after aborting the pending key request', async function () {
    await kmsGeneration.keygen(0);
    await kmsGeneration.abortKeygen(PREP_KEYGEN_COUNTER_BASE + 1n);

    await run('task:assertNoPendingKeyManagementRequest', { address: kmsGenerationAddress });
  });

  it('passes again after aborting the pending CRS request', async function () {
    await kmsGeneration.crsgenRequest(4096, 0);
    await kmsGeneration.abortCrsgen(CRS_COUNTER_BASE + 1n);

    await run('task:assertNoPendingKeyManagementRequest', { address: kmsGenerationAddress });
  });
});
