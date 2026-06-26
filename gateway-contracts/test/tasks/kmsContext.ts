import { loadFixture } from '@nomicfoundation/hardhat-network-helpers';
import { expect } from 'chai';
import { Interface } from 'ethers';
import hre from 'hardhat';

import { encodeUpdateKmsContext } from '../../tasks/kmsContext';
import { GatewayConfig } from '../../typechain-types';
import { createRandomAddress, loadTestVariablesFixture } from '../utils';

// KMS context ID format: [0x07 | counter]. The fixture deploys with context id |1, so |2 is the next.
const NEXT_KMS_CONTEXT_ID = (7n << 248n) | 2n;

interface KmsNode {
  txSenderAddress: string;
  signerAddress: string;
  ipAddress: string;
  storageUrl: string;
}

interface KmsThresholds {
  mpcThreshold: number;
  publicDecryptionThreshold: number;
  userDecryptionThreshold: number;
  kmsGenThreshold: number;
}

function makeNode(idx: number): KmsNode {
  return {
    txSenderAddress: createRandomAddress(),
    signerAddress: createRandomAddress(),
    ipAddress: `10.0.0.${idx + 1}`,
    storageUrl: `s3://kms-bucket-${idx}`,
  };
}

const DEFAULT_THRESHOLDS: KmsThresholds = {
  mpcThreshold: 1,
  publicDecryptionThreshold: 1,
  userDecryptionThreshold: 1,
  kmsGenThreshold: 1,
};

// Sets the KMS_* env vars the task reads (the same vars that define the node set at deployment).
function setKmsEnv(nodes: KmsNode[], thresholds: KmsThresholds, newContextId: bigint): void {
  process.env.NUM_KMS_NODES = String(nodes.length);
  nodes.forEach((node, i) => {
    process.env[`KMS_TX_SENDER_ADDRESS_${i}`] = node.txSenderAddress;
    process.env[`KMS_SIGNER_ADDRESS_${i}`] = node.signerAddress;
    process.env[`KMS_NODE_IP_ADDRESS_${i}`] = node.ipAddress;
    process.env[`KMS_NODE_STORAGE_URL_${i}`] = node.storageUrl;
  });
  process.env.MPC_THRESHOLD = String(thresholds.mpcThreshold);
  process.env.PUBLIC_DECRYPTION_THRESHOLD = String(thresholds.publicDecryptionThreshold);
  process.env.USER_DECRYPTION_THRESHOLD = String(thresholds.userDecryptionThreshold);
  process.env.KMS_GENERATION_THRESHOLD = String(thresholds.kmsGenThreshold);
  process.env.KMS_CONTEXT_ID = newContextId.toString();
}

describe('KMS context tasks', function () {
  let gatewayConfig: GatewayConfig;
  let gatewayConfigAddress: string;
  // Full env snapshot/restore: the task reads process-wide KMS_* vars, so each test must leave the
  // environment exactly as it found it for the rest of the suite.
  let envBackup: NodeJS.ProcessEnv;

  beforeEach(async function () {
    envBackup = { ...process.env };
    const fixture = await loadFixture(loadTestVariablesFixture);
    gatewayConfig = fixture.gatewayConfig;
    gatewayConfigAddress = await gatewayConfig.getAddress();
  });

  afterEach(function () {
    for (const key of Object.keys(process.env)) {
      if (!(key in envBackup)) {
        delete process.env[key];
      }
    }
    Object.assign(process.env, envBackup);
  });

  describe('updateKmsContext', function () {
    it('builds a governance-proposal-builder-compatible triple that decodes to the env node set (DAO path)', async function () {
      const nodes = [makeNode(0), makeNode(1)];
      setKmsEnv(nodes, DEFAULT_THRESHOLDS, NEXT_KMS_CONTEXT_ID);

      const encoded = await encodeUpdateKmsContext(hre);

      expect(encoded.functionSignature).to.equal(
        'updateKmsContext(uint256,(address,address,string,string)[],uint256,uint256,uint256,uint256)',
      );

      // `argsData` is selector-less; prepending the selector reproduces the full calldata.
      const iface = new Interface([`function ${encoded.functionSignature}`]);
      const selector = iface.getFunction('updateKmsContext')!.selector;
      expect(selector + encoded.argsData.slice(2)).to.equal(encoded.fullCalldata);

      // Decodes back to the env node set + newContextId.
      const decoded = iface.decodeFunctionData('updateKmsContext', encoded.fullCalldata);
      expect(decoded[0]).to.equal(NEXT_KMS_CONTEXT_ID);
      expect(decoded[1].length).to.equal(nodes.length);
      nodes.forEach((node, i) => {
        expect(decoded[1][i][0]).to.equal(node.txSenderAddress);
        expect(decoded[1][i][1]).to.equal(node.signerAddress);
        expect(decoded[1][i][2]).to.equal(node.ipAddress);
        expect(decoded[1][i][3]).to.equal(node.storageUrl);
      });
      // Threshold order in updateKmsContext: (mpc, publicDecryption, userDecryption, kmsGen).
      expect(decoded[2]).to.equal(BigInt(DEFAULT_THRESHOLDS.mpcThreshold));
      expect(decoded[3]).to.equal(BigInt(DEFAULT_THRESHOLDS.publicDecryptionThreshold));
    });

    it('uses the newContextId that the Ethereum proposal must also carry (KMS_CONTEXT_ID anchor)', async function () {
      setKmsEnv([makeNode(0), makeNode(1)], DEFAULT_THRESHOLDS, NEXT_KMS_CONTEXT_ID);
      const encoded = await encodeUpdateKmsContext(hre);
      // The host derives this same id on-chain (current + 1) and prints it as the value to set
      // here, so the two proposals stay aligned.
      expect(encoded.newContextId).to.equal(NEXT_KMS_CONTEXT_ID);
    });

    it('rejects a newContextId that is not greater than the live context id', async function () {
      // |1 == the current context id -> must be strictly greater.
      setKmsEnv([makeNode(0), makeNode(1)], DEFAULT_THRESHOLDS, (7n << 248n) | 1n);
      await expect(
        hre.run('task:buildUpdateKmsContextProposal', { gatewayConfigAddress, verifyContextId: true }),
      ).to.be.rejectedWith(/must be strictly greater/);
    });

    it('broadcasts the update with the deployer key (no-DAO path) advancing the context id', async function () {
      const nodes = [makeNode(0), makeNode(1)];
      setKmsEnv(nodes, DEFAULT_THRESHOLDS, NEXT_KMS_CONTEXT_ID);

      await hre.run('task:updateKmsContext', { gatewayConfigAddress });

      expect(await gatewayConfig.getCurrentKmsContextId()).to.equal(NEXT_KMS_CONTEXT_ID);
      expect(await gatewayConfig.isKmsSignerForContext(NEXT_KMS_CONTEXT_ID, nodes[0].signerAddress)).to.be.true;
      expect(await gatewayConfig.getKmsSignersForContext(NEXT_KMS_CONTEXT_ID)).to.deep.equal(
        nodes.map((n) => n.signerAddress),
      );
    });
  });
});
