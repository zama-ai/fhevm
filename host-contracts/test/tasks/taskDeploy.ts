import { expect } from 'chai';
import { ethers, run } from 'hardhat';

import { CRS_COUNTER_BASE, KEY_COUNTER_BASE, PREP_KEYGEN_COUNTER_BASE } from '../../tasks/utils/kmsGenerationConstants';
import { getRequiredEnvVar } from '../../tasks/utils/loadVariables';
import type { KMSGeneration } from '../../types';
import { deployFreshKMSGenerationProxy, readHostAddress } from './taskHelpers';

describe('task:deployAllHostContracts', function () {
  it('requires the KMSGeneration deployment role to be explicit', async function () {
    await expect(run('task:deployAllHostContracts')).to.be.rejectedWith(/withKmsGeneration/);
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
      `Contract at ${protocolConfigAddress} reports version "ProtocolConfig v0.1.0"; expected "KMSGeneration v…".`,
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
