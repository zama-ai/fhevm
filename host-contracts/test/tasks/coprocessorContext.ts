import { expect } from 'chai';
import hre, { ethers, run } from 'hardhat';

import {
  encodeDefineNewCoprocessorContext,
  encodeDestroyCoprocessorContext,
  predictNewCoprocessorContextId,
} from '../../tasks/coprocessorContext';
import {
  COPROCESSOR_CONFIG_ADDRESS_ENV_VAR,
  getCoprocessorConfigInterface,
} from '../../tasks/utils/coprocessorConfigTasks';
import { COPROCESSOR_CONTEXT_COUNTER_BASE } from '../../tasks/utils/kmsGenerationConstants';
import { getRequiredEnvVar } from '../../tasks/utils/loadVariables';
import type { CoprocessorConfig } from '../../types';
import { COPROCESSOR_MAJORITY_THRESHOLD, buildCoprocessors, deployFreshCoprocessorConfigProxy } from './taskHelpers';

interface CoprocessorEntry {
  txSenderAddress: string;
  signerAddress: string;
  storageUrl: string;
  prfEndpointUrl: string;
}

const NEW_COPROCESSORS: CoprocessorEntry[] = [
  {
    txSenderAddress: '0x00000000000000000000000000000000000C1111',
    signerAddress: '0x00000000000000000000000000000000000C2222',
    storageUrl: 'https://new-coprocessor0.example.com',
    prfEndpointUrl: 'https://new-prf0.example.com',
  },
  {
    txSenderAddress: '0x00000000000000000000000000000000000C3333',
    signerAddress: '0x00000000000000000000000000000000000C4444',
    storageUrl: 'https://new-coprocessor1.example.com',
    prfEndpointUrl: '',
  },
];
const NEW_MAJORITY_THRESHOLD = 2;

// Sets the COPROCESSOR_* env vars the tasks read (the same vars that define the set at deployment).
function setCoprocessorEnv(coprocessors: CoprocessorEntry[], majorityThreshold: number): void {
  process.env.NUM_COPROCESSORS = String(coprocessors.length);
  coprocessors.forEach((coprocessor, i) => {
    process.env[`COPROCESSOR_TX_SENDER_ADDRESS_${i}`] = coprocessor.txSenderAddress;
    process.env[`COPROCESSOR_SIGNER_ADDRESS_${i}`] = coprocessor.signerAddress;
    process.env[`COPROCESSOR_STORAGE_URL_${i}`] = coprocessor.storageUrl;
    process.env[`COPROCESSOR_PRF_ENDPOINT_URL_${i}`] = coprocessor.prfEndpointUrl;
  });
  process.env.COPROCESSOR_THRESHOLD = String(majorityThreshold);
}

describe('Coprocessor context tasks', function () {
  const deployer = new ethers.Wallet(getRequiredEnvVar('DEPLOYER_PRIVATE_KEY')).connect(ethers.provider);
  // Full env snapshot/restore: the tasks read process-wide COPROCESSOR_* vars, so each test must
  // leave the environment exactly as it found it for the rest of the suite.
  let envBackup: NodeJS.ProcessEnv;

  beforeEach(function () {
    envBackup = { ...process.env };
  });

  afterEach(function () {
    for (const key of Object.keys(process.env)) {
      if (!(key in envBackup)) {
        delete process.env[key];
      }
    }
    Object.assign(process.env, envBackup);
  });

  async function deployCanonical(): Promise<CoprocessorConfig> {
    const address = await deployFreshCoprocessorConfigProxy(
      deployer,
      buildCoprocessors(),
      COPROCESSOR_MAJORITY_THRESHOLD,
    );
    process.env[COPROCESSOR_CONFIG_ADDRESS_ENV_VAR] = address;
    return (await ethers.getContractAt('CoprocessorConfig', address)) as unknown as CoprocessorConfig;
  }

  describe('defineNewCoprocessorContext', function () {
    it('builds calldata that decodes back to the env coprocessor set (DAO path)', async function () {
      setCoprocessorEnv(NEW_COPROCESSORS, NEW_MAJORITY_THRESHOLD);
      const iface = await getCoprocessorConfigInterface(hre);
      const encoded = encodeDefineNewCoprocessorContext(iface);

      expect(encoded.functionSignature).to.equal(
        'defineNewCoprocessorContext((address,address,string,string)[],uint256)',
      );
      expect(encoded.calldata.slice(0, 10)).to.equal(iface.getFunction('defineNewCoprocessorContext')!.selector);
      const [coprocessors, majorityThreshold] = encoded.decodedArgs as [CoprocessorEntry[], bigint];
      expect(coprocessors).to.deep.equal(NEW_COPROCESSORS);
      expect(majorityThreshold).to.equal(BigInt(NEW_MAJORITY_THRESHOLD));
    });

    it('predicts the new context ID as the current one + 1', async function () {
      const canonical = await deployCanonical();
      expect(await predictNewCoprocessorContextId(hre, await canonical.getAddress())).to.equal(
        COPROCESSOR_CONTEXT_COUNTER_BASE + 2n,
      );
    });

    it('broadcasts the new context with the deployer key (no-DAO path), making it current', async function () {
      const canonical = await deployCanonical();
      setCoprocessorEnv(NEW_COPROCESSORS, NEW_MAJORITY_THRESHOLD);

      await run('task:defineNewCoprocessorContext', {});

      const newContextId = COPROCESSOR_CONTEXT_COUNTER_BASE + 2n;
      expect(await canonical.getCurrentCoprocessorContextId()).to.equal(newContextId);
      expect(await canonical.getCoprocessorMajorityThresholdForContext(newContextId)).to.equal(
        BigInt(NEW_MAJORITY_THRESHOLD),
      );
      const stored = await canonical.getCoprocessorsForContext(newContextId);
      expect(stored.map((coprocessor) => coprocessor.signerAddress)).to.deep.equal(
        NEW_COPROCESSORS.map((coprocessor) => coprocessor.signerAddress),
      );
    });
  });

  describe('destroyCoprocessorContext', function () {
    it('builds calldata that decodes back to the context ID (DAO path)', async function () {
      const iface = await getCoprocessorConfigInterface(hre);
      const encoded = encodeDestroyCoprocessorContext(iface, COPROCESSOR_CONTEXT_COUNTER_BASE + 1n);
      const decoded = iface.decodeFunctionData('destroyCoprocessorContext', encoded.calldata);
      expect(decoded[0]).to.equal(COPROCESSOR_CONTEXT_COUNTER_BASE + 1n);
      expect(encoded.calldata.slice(0, 10)).to.equal(iface.getFunction('destroyCoprocessorContext')!.selector);
    });

    it('broadcasts the destruction of a past context (no-DAO path)', async function () {
      const canonical = await deployCanonical();
      setCoprocessorEnv(NEW_COPROCESSORS, NEW_MAJORITY_THRESHOLD);
      await run('task:defineNewCoprocessorContext', {});

      await run('task:destroyCoprocessorContext', {
        coprocessorContextId: (COPROCESSOR_CONTEXT_COUNTER_BASE + 1n).toString(),
      });

      expect(await canonical.isValidCoprocessorContext(COPROCESSOR_CONTEXT_COUNTER_BASE + 1n)).to.equal(false);
      expect(await canonical.isValidCoprocessorContext(COPROCESSOR_CONTEXT_COUNTER_BASE + 2n)).to.equal(true);
    });
  });
});
