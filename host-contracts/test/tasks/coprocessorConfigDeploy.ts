import { expect } from 'chai';
import fs from 'fs';
import hre, { ethers, run } from 'hardhat';

import { readCanonicalCoprocessorSnapshot } from '../../tasks/coprocessorConfigMirror';
import {
  applyCoprocessorConfigCanonicalEnv,
  buildCanonicalCoprocessorSnapshotEnv,
  readCanonicalCoprocessorSnapshotFromEnv,
  restoreCoprocessorConfigCanonicalEnv,
  snapshotCoprocessorConfigCanonicalEnv,
} from '../../tasks/utils/coprocessorConfigCanonicalEnv';
import { COPROCESSOR_CONTEXT_COUNTER_BASE } from '../../tasks/utils/kmsGenerationConstants';
import { getRequiredEnvVar } from '../../tasks/utils/loadVariables';
import { UPGRADE_TO_AND_CALL_INTERFACE } from '../../tasks/utils/upgradeProposal';
import type { CoprocessorConfig, CoprocessorConfigReplica } from '../../types';
import {
  COPROCESSOR_MAJORITY_THRESHOLD,
  HOST_ENV_FILE,
  buildCoprocessors,
  deployFreshCoprocessorConfigProxy,
  deployFreshEmptyUUPSProxy,
} from './taskHelpers';

const FIRST_COPROCESSOR_CONTEXT_ID = COPROCESSOR_CONTEXT_COUNTER_BASE + 1n;

// Sets a key in .env.host, appending it when absent.
function setHostEnv(key: string, value: string): void {
  const content = fs.readFileSync(HOST_ENV_FILE, 'utf-8');
  const pattern = new RegExp(`^${key}=.*$`, 'm');
  const updated = pattern.test(content)
    ? content.replace(pattern, `${key}=${value}`)
    : `${content.replace(/\n?$/, '\n')}${key}=${value}\n`;
  fs.writeFileSync(HOST_ENV_FILE, updated);
}

function expectCoprocessors(actual: unknown[][], expected: ReturnType<typeof buildCoprocessors>): void {
  expect(actual.map((coprocessor) => [...coprocessor])).to.deep.equal(
    expected.map((coprocessor) => [
      ethers.getAddress(coprocessor.txSenderAddress),
      ethers.getAddress(coprocessor.signerAddress),
      coprocessor.storageUrl,
      coprocessor.prfEndpointUrl,
    ]),
  );
}

describe('CoprocessorConfig deploy tasks', function () {
  const deployer = new ethers.Wallet(getRequiredEnvVar('DEPLOYER_PRIVATE_KEY')).connect(ethers.provider);
  let originalEnvHost: string;
  let originalCanonicalEnv: ReturnType<typeof snapshotCoprocessorConfigCanonicalEnv>;

  before(function () {
    originalEnvHost = fs.readFileSync(HOST_ENV_FILE, 'utf-8');
  });

  beforeEach(function () {
    originalCanonicalEnv = snapshotCoprocessorConfigCanonicalEnv();
    restoreCoprocessorConfigCanonicalEnv({});
  });

  afterEach(function () {
    fs.writeFileSync(HOST_ENV_FILE, originalEnvHost);
    restoreCoprocessorConfigCanonicalEnv(originalCanonicalEnv);
  });

  async function deployCanonical(): Promise<CoprocessorConfig> {
    const address = await deployFreshCoprocessorConfigProxy(
      deployer,
      buildCoprocessors(),
      COPROCESSOR_MAJORITY_THRESHOLD,
    );
    return (await ethers.getContractAt('CoprocessorConfig', address, deployer)) as unknown as CoprocessorConfig;
  }

  // Exports the canonical coprocessor context and loads it into the CANONICAL_COPROCESSOR_* env
  // variables, as a deployment platform injects the export task's `export` map.
  async function exportToEnv(canonical: CoprocessorConfig): Promise<void> {
    const snapshot = await readCanonicalCoprocessorSnapshot(hre, {
      canonicalProvider: ethers.provider,
      canonicalCoprocessorConfigAddress: await canonical.getAddress(),
      blockNumber: await ethers.provider.getBlockNumber(),
    });
    applyCoprocessorConfigCanonicalEnv(buildCanonicalCoprocessorSnapshotEnv(snapshot));
  }

  describe('task:deployCoprocessorConfig', function () {
    it('initializes the empty proxy with the COPROCESSOR_* env set as the first context', async function () {
      const proxyAddress = await deployFreshEmptyUUPSProxy(deployer);
      setHostEnv('COPROCESSOR_CONFIG_CONTRACT_ADDRESS', proxyAddress);

      await run('task:deployCoprocessorConfig');

      const coprocessorConfig = (await ethers.getContractAt(
        'CoprocessorConfig',
        proxyAddress,
      )) as unknown as CoprocessorConfig;
      expect(await coprocessorConfig.getVersion()).to.match(/^CoprocessorConfig v/);
      expect(await coprocessorConfig.getCurrentCoprocessorContextId()).to.equal(FIRST_COPROCESSOR_CONTEXT_ID);
      const numCoprocessors = Number(getRequiredEnvVar('NUM_COPROCESSORS'));
      const stored = await coprocessorConfig.getCoprocessorsForContext(FIRST_COPROCESSOR_CONTEXT_ID);
      expect(stored.length).to.equal(numCoprocessors);
      expect(stored[0].txSenderAddress).to.equal(getRequiredEnvVar('COPROCESSOR_TX_SENDER_ADDRESS_0'));
      expect(stored[0].storageUrl).to.equal(getRequiredEnvVar('COPROCESSOR_STORAGE_URL_0'));
      expect(await coprocessorConfig.getCoprocessorMajorityThresholdForContext(FIRST_COPROCESSOR_CONTEXT_ID)).to.equal(
        BigInt(getRequiredEnvVar('COPROCESSOR_THRESHOLD')),
      );
    });

    it('task:assertCoprocessorConfigReady rejects a proxy that is not a CoprocessorConfig', async function () {
      setHostEnv('COPROCESSOR_CONFIG_CONTRACT_ADDRESS', await deployFreshEmptyUUPSProxy(deployer));
      await expect(run('task:assertCoprocessorConfigReady')).to.be.rejectedWith(/does not expose getVersion/);
    });
  });

  describe('canonical snapshot', function () {
    it('reads the current context of the canonical contract and round-trips through the env', async function () {
      const canonical = await deployCanonical();
      const snapshot = await readCanonicalCoprocessorSnapshot(hre, {
        canonicalProvider: ethers.provider,
        canonicalCoprocessorConfigAddress: await canonical.getAddress(),
        blockNumber: await ethers.provider.getBlockNumber(),
      });

      expect(snapshot.coprocessorConfigAddress).to.equal(await canonical.getAddress());
      expect(snapshot.currentCoprocessorContextId).to.equal(FIRST_COPROCESSOR_CONTEXT_ID);
      expect(snapshot.majorityThreshold).to.equal(BigInt(COPROCESSOR_MAJORITY_THRESHOLD));
      expect(snapshot.coprocessors).to.deep.equal(
        buildCoprocessors().map((coprocessor) => ({
          ...coprocessor,
          txSenderAddress: ethers.getAddress(coprocessor.txSenderAddress),
          signerAddress: ethers.getAddress(coprocessor.signerAddress),
        })),
      );

      applyCoprocessorConfigCanonicalEnv(buildCanonicalCoprocessorSnapshotEnv(snapshot));
      expect(readCanonicalCoprocessorSnapshotFromEnv()).to.deep.equal(snapshot);
    });

    it('rejects a snapshot read from a contract that is not a CoprocessorConfig', async function () {
      await expect(
        readCanonicalCoprocessorSnapshot(hre, {
          canonicalProvider: ethers.provider,
          canonicalCoprocessorConfigAddress: await deployFreshEmptyUUPSProxy(deployer),
        }),
      ).to.be.rejectedWith(/is not a CoprocessorConfig proxy/);
    });

    it('rejects a malformed coprocessor entry, naming the variable', async function () {
      await exportToEnv(await deployCanonical());
      const coprocessors = JSON.parse(process.env.CANONICAL_COPROCESSORS!);
      coprocessors[0].txSenderAddress = 'not-an-address';
      process.env.CANONICAL_COPROCESSORS = JSON.stringify(coprocessors);
      expect(() => readCanonicalCoprocessorSnapshotFromEnv()).to.throw(
        /"CANONICAL_COPROCESSORS" env variable entry 0 field "txSenderAddress" must be a valid address/,
      );

      delete coprocessors[0].prfEndpointUrl;
      coprocessors[0].txSenderAddress = coprocessors[0].signerAddress;
      process.env.CANONICAL_COPROCESSORS = JSON.stringify(coprocessors);
      expect(() => readCanonicalCoprocessorSnapshotFromEnv()).to.throw(
        /"CANONICAL_COPROCESSORS" env variable entry 0 field "prfEndpointUrl" is missing/,
      );

      process.env.CANONICAL_COPROCESSORS = '[]';
      expect(() => readCanonicalCoprocessorSnapshotFromEnv()).to.throw(
        /"CANONICAL_COPROCESSORS" env variable must hold a non-empty JSON array/,
      );
    });

    it('names the first missing env variable', function () {
      expect(() => readCanonicalCoprocessorSnapshotFromEnv()).to.throw(/CANONICAL_COPROCESSOR_CHAIN_ID/);
    });
  });

  describe('replica from the canonical snapshot', function () {
    it('task:prepareDeployCoprocessorConfigFromCanonical prints initializeFromCanonical calldata without upgrading', async function () {
      const canonical = await deployCanonical();
      await exportToEnv(canonical);
      const proxyAddress = await deployFreshEmptyUUPSProxy(deployer);
      setHostEnv('COPROCESSOR_CONFIG_CONTRACT_ADDRESS', proxyAddress);

      const prepared = await run('task:prepareDeployCoprocessorConfigFromCanonical', { verifyContract: false });

      expect(prepared.proxyAddress).to.equal(proxyAddress);
      const iface = new ethers.Interface([`function ${prepared.innerFunctionSignature}`]);
      const decoded = iface.decodeFunctionData('initializeFromCanonical', prepared.innerCalldata);
      expect(decoded[0]).to.equal(await canonical.getCurrentCoprocessorContextId());
      expectCoprocessors(decoded[1], buildCoprocessors());
      expect(decoded[2]).to.equal(BigInt(COPROCESSOR_MAJORITY_THRESHOLD));
      expect(prepared.outerCalldata).to.equal(
        UPGRADE_TO_AND_CALL_INTERFACE.encodeFunctionData('upgradeToAndCall', [
          prepared.newImplementationAddress,
          prepared.innerCalldata,
        ]),
      );
      // The proxy is still the empty one: preparing does not upgrade.
      await expect(run('task:assertCoprocessorConfigReady')).to.be.rejectedWith(/does not expose getVersion/);
    });

    it('task:deployCoprocessorConfigFromCanonical lands the replica on the canonical context', async function () {
      const canonical = await deployCanonical();
      await (await canonical.defineNewCoprocessorContext(buildCoprocessors().slice(0, 1), 1)).wait();
      await exportToEnv(canonical);
      const proxyAddress = await deployFreshEmptyUUPSProxy(deployer);
      setHostEnv('COPROCESSOR_CONFIG_CONTRACT_ADDRESS', proxyAddress);

      await run('task:deployCoprocessorConfigFromCanonical');

      const replica = (await ethers.getContractAt(
        'CoprocessorConfigReplica',
        proxyAddress,
      )) as unknown as CoprocessorConfigReplica;
      expect(await replica.getVersion()).to.match(/^CoprocessorConfigReplica v/);
      const contextId = FIRST_COPROCESSOR_CONTEXT_ID + 1n;
      expect(await replica.getCurrentCoprocessorContextId()).to.equal(contextId);
      expectCoprocessors(
        (await replica.getCoprocessorsForContext(contextId)) as unknown as unknown[][],
        buildCoprocessors().slice(0, 1),
      );
      expect(await replica.getCoprocessorMajorityThresholdForContext(contextId)).to.equal(1n);
      // The canonical context the replica started after was never mirrored.
      expect(await replica.isValidCoprocessorContext(FIRST_COPROCESSOR_CONTEXT_ID)).to.equal(false);
    });
  });
});
