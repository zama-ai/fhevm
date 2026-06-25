import { expect } from 'chai';
import { Signer, Wallet } from 'ethers';
import hre, { ethers, run } from 'hardhat';

import {
  encodeDefineNewEpochForCurrentKmsContext,
  encodeDefineNewKmsContextAndEpoch,
  getProtocolConfigInterface,
  inspectKmsContextSwitch,
} from '../../tasks/kmsContext';
import { getRequiredEnvVar } from '../../tasks/utils/loadVariables';
import type { ProtocolConfig } from '../../types';
import { deployFreshProtocolConfigProxy } from './taskHelpers';

const PROTOCOL_CONFIG_ENV_VAR = 'PROTOCOL_CONFIG_CONTRACT_ADDRESS';

interface NodeConfig {
  txSenderAddress: string;
  signerAddress: string;
  ipAddress: string;
  storageUrl: string;
  partyId: number;
  mpcIdentity: string;
  caCert: string;
  storagePrefix: string;
}

interface Thresholds {
  publicDecryption: number;
  userDecryption: number;
  kmsGen: number;
  mpc: number;
}

function makeNode(txSenderAddress: string, signerAddress: string, idx: number): NodeConfig {
  return {
    txSenderAddress,
    signerAddress,
    ipAddress: `10.0.0.${idx + 1}`,
    storageUrl: `https://s${idx}.example.com`,
    partyId: idx,
    mpcIdentity: `node-${idx}`,
    caCert: '0x',
    storagePrefix: '',
  };
}

function defaultNodes(): NodeConfig[] {
  return [
    makeNode('0x0000000000000000000000000000000000001111', '0x0000000000000000000000000000000000002222', 0),
    makeNode('0x0000000000000000000000000000000000003333', '0x0000000000000000000000000000000000004444', 1),
    makeNode('0x0000000000000000000000000000000000005555', '0x0000000000000000000000000000000000006666', 2),
  ];
}

const DEFAULT_THRESHOLDS: Thresholds = { publicDecryption: 1, userDecryption: 2, kmsGen: 2, mpc: 1 };

// Sets the KMS_* env vars the tasks read (the same vars that define the node set at deployment).
function setKmsEnv(
  nodes: NodeConfig[],
  thresholds: Thresholds,
  opts: {
    softwareVersion?: string;
    pcrValues?: { pcr0: string; pcr1: string; pcr2: string }[];
    newContextId?: string;
  } = {},
): void {
  process.env.NUM_KMS_NODES = String(nodes.length);
  nodes.forEach((node, i) => {
    process.env[`KMS_TX_SENDER_ADDRESS_${i}`] = node.txSenderAddress;
    process.env[`KMS_SIGNER_ADDRESS_${i}`] = node.signerAddress;
    process.env[`KMS_NODE_IP_${i}`] = node.ipAddress;
    process.env[`KMS_NODE_STORAGE_URL_${i}`] = node.storageUrl;
    process.env[`KMS_NODE_PARTY_ID_${i}`] = String(node.partyId);
    process.env[`KMS_NODE_MPC_IDENTITY_${i}`] = node.mpcIdentity;
    process.env[`KMS_NODE_CA_CERT_${i}`] = node.caCert;
    process.env[`KMS_NODE_STORAGE_PREFIX_${i}`] = node.storagePrefix;
  });
  process.env.PUBLIC_DECRYPTION_THRESHOLD = String(thresholds.publicDecryption);
  process.env.USER_DECRYPTION_THRESHOLD = String(thresholds.userDecryption);
  process.env.KMS_GEN_THRESHOLD = String(thresholds.kmsGen);
  process.env.MPC_THRESHOLD = String(thresholds.mpc);
  process.env.KMS_SOFTWARE_VERSION = opts.softwareVersion ?? 'v0.14.0';
  process.env.KMS_PCR_VALUES = JSON.stringify(opts.pcrValues ?? [{ pcr0: '0xaa', pcr1: '0xbb', pcr2: '0xcc' }]);
  if (opts.newContextId !== undefined) {
    process.env.KMS_NEW_CONTEXT_ID = opts.newContextId;
  }
}

describe('KMS context tasks', function () {
  const deployer = new ethers.Wallet(getRequiredEnvVar('DEPLOYER_PRIVATE_KEY')).connect(ethers.provider);
  // Full env snapshot/restore: the tasks read process-wide KMS_* vars, so each test must leave the
  // environment exactly as it found it for the rest of the suite.
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

  // ---------------------------------------------------------------------------
  // defineNewKmsContextAndEpoch — DAO calldata (build) + direct (execute)
  // ---------------------------------------------------------------------------

  describe('defineNewKmsContextAndEpoch', function () {
    it('builds calldata that decodes back to the env node set (DAO path)', async function () {
      const nodes = defaultNodes();
      setKmsEnv(nodes, DEFAULT_THRESHOLDS);

      const iface = await getProtocolConfigInterface(hre);
      const encoded = encodeDefineNewKmsContextAndEpoch(iface);
      const decoded = iface.decodeFunctionData('defineNewKmsContextAndEpoch', encoded.calldata);

      const decodedNodes = decoded[0];
      expect(decodedNodes.length).to.equal(nodes.length);
      nodes.forEach((node, i) => {
        expect(decodedNodes[i][0]).to.equal(node.txSenderAddress);
        expect(decodedNodes[i][1]).to.equal(node.signerAddress);
        expect(decodedNodes[i][2]).to.equal(node.ipAddress);
        expect(decodedNodes[i][3]).to.equal(node.storageUrl);
        expect(decodedNodes[i][4]).to.equal(BigInt(node.partyId));
        expect(decodedNodes[i][5]).to.equal(node.mpcIdentity);
        expect(decodedNodes[i][6]).to.equal(node.caCert);
        expect(decodedNodes[i][7]).to.equal(node.storagePrefix);
      });
      expect(decoded[1][0]).to.equal(BigInt(DEFAULT_THRESHOLDS.publicDecryption));
      expect(decoded[1][3]).to.equal(BigInt(DEFAULT_THRESHOLDS.mpc));
      expect(decoded[2]).to.equal('v0.14.0');
      expect(decoded[3][0][0]).to.equal('0xaa');
    });

    it('verifies KMS_NEW_CONTEXT_ID against a live ProtocolConfig and rejects a mismatch', async function () {
      const proxyAddress = await deployFreshProtocolConfigProxy(deployer, defaultNodes(), DEFAULT_THRESHOLDS);
      const protocolConfig = (await ethers.getContractAt('ProtocolConfig', proxyAddress)) as unknown as ProtocolConfig;
      const currentContextId = await protocolConfig.getCurrentKmsContextId();

      setKmsEnv(defaultNodes(), DEFAULT_THRESHOLDS, { newContextId: (currentContextId + 1n).toString() });
      process.env[PROTOCOL_CONFIG_ENV_VAR] = proxyAddress;
      await expect(run('task:buildDefineNewKmsContextAndEpochCalldata', {})).to.not.be.rejected;

      process.env.KMS_NEW_CONTEXT_ID = (currentContextId + 2n).toString();
      await expect(run('task:buildDefineNewKmsContextAndEpochCalldata', {})).to.be.rejectedWith(
        /does not match the id ProtocolConfig/,
      );
    });

    it('broadcasts the switch with the deployer key (no-DAO path) leaving a PENDING context', async function () {
      const initialNodes = defaultNodes();
      const proxyAddress = await deployFreshProtocolConfigProxy(deployer, initialNodes, DEFAULT_THRESHOLDS);

      // A distinct new committee for the switch.
      const newNodes = [
        makeNode('0x00000000000000000000000000000000000A1111', '0x00000000000000000000000000000000000A2222', 0),
        makeNode('0x00000000000000000000000000000000000A3333', '0x00000000000000000000000000000000000A4444', 1),
      ];
      setKmsEnv(newNodes, { publicDecryption: 1, userDecryption: 1, kmsGen: 1, mpc: 1 });
      process.env[PROTOCOL_CONFIG_ENV_VAR] = proxyAddress;

      await run('task:defineNewKmsContextAndEpoch', {});

      const status = await inspectKmsContextSwitch(hre, proxyAddress, 0);
      expect(status.flow).to.equal('context-switch');
      expect(status.contextState).to.equal('PENDING');
      expect(status.newSigners).to.deep.equal(newNodes.map((n) => n.signerAddress));
    });
  });

  // ---------------------------------------------------------------------------
  // defineNewEpochForCurrentKmsContext — DAO calldata (build) + direct (execute)
  // ---------------------------------------------------------------------------

  describe('defineNewEpochForCurrentKmsContext', function () {
    it('builds the no-arg selector calldata (DAO path)', async function () {
      const iface = await getProtocolConfigInterface(hre);
      const encoded = encodeDefineNewEpochForCurrentKmsContext(iface);
      expect(encoded.calldata).to.equal(iface.getFunction('defineNewEpochForCurrentKmsContext')!.selector);
      expect(encoded.calldata).to.have.lengthOf(10);
    });

    it('broadcasts the rotation with the deployer key (no-DAO path) leaving a PENDING epoch', async function () {
      const proxyAddress = await deployFreshProtocolConfigProxy(deployer, defaultNodes(), DEFAULT_THRESHOLDS);
      process.env[PROTOCOL_CONFIG_ENV_VAR] = proxyAddress;

      await run('task:defineNewEpochForCurrentKmsContext', {});

      const status = await inspectKmsContextSwitch(hre, proxyAddress, 0);
      expect(status.flow).to.equal('same-set-rotation');
      expect(status.epochState).to.equal('PENDING');
    });
  });

  // ---------------------------------------------------------------------------
  // Status task (event-indexing monitor)
  // ---------------------------------------------------------------------------

  describe('context-switch status', function () {
    // Old context: 3 nodes, mpc=2 -> previous-signer creation target (n - t + 1) = 2.
    // New context: 2 nodes, mpc=1. Old/new signer sets are disjoint so confirmations partition cleanly.
    let proxyAddress: string;
    let protocolConfig: ProtocolConfig;
    let oldSigners: Signer[];
    let newSigners: Signer[];
    let newTxSenders: Signer[];
    let newNodes: NodeConfig[];
    const newThresholds: Thresholds = { publicDecryption: 1, userDecryption: 1, kmsGen: 1, mpc: 1 };

    async function asOwner(): Promise<ProtocolConfig> {
      return (await ethers.getContractAt('ProtocolConfig', proxyAddress, deployer)) as unknown as ProtocolConfig;
    }

    function parseEventArg(
      receipt: { logs: readonly { topics: string[]; data: string }[] },
      name: string,
      arg: string,
    ): bigint | undefined {
      for (const log of receipt.logs) {
        let parsed;
        try {
          parsed = protocolConfig.interface.parseLog({ topics: [...log.topics], data: log.data });
        } catch {
          continue;
        }
        if (parsed?.name === name) {
          return parsed.args[arg] as bigint;
        }
      }
      return undefined;
    }

    async function defineSwitch(): Promise<bigint> {
      const receipt = await (
        await (await asOwner()).defineNewKmsContextAndEpoch(newNodes, newThresholds, '', [])
      ).wait();
      const contextId = parseEventArg(receipt!, 'NewKmsContext', 'contextId');
      if (contextId === undefined) {
        throw new Error('NewKmsContext not emitted');
      }
      return contextId;
    }

    async function confirmCreation(contextId: bigint, signers: Signer[]): Promise<bigint | undefined> {
      let epochId: bigint | undefined;
      for (const signer of signers) {
        const asSigner = (await ethers.getContractAt(
          'ProtocolConfig',
          proxyAddress,
          signer,
        )) as unknown as ProtocolConfig;
        const receipt = await (await asSigner.confirmKmsContextCreation(contextId)).wait();
        epochId ??= parseEventArg(receipt!, 'NewKmsEpoch', 'epochId');
      }
      return epochId;
    }

    async function confirmActivation(epochId: bigint, txSenders: Signer[]): Promise<void> {
      for (const txSender of txSenders) {
        const asTxSender = (await ethers.getContractAt(
          'ProtocolConfig',
          proxyAddress,
          txSender,
        )) as unknown as ProtocolConfig;
        await (await asTxSender.confirmEpochActivation(epochId, [], [])).wait();
      }
    }

    beforeEach(async function () {
      const accounts = await ethers.getSigners();
      oldSigners = accounts.slice(1, 4);
      newSigners = accounts.slice(4, 6);
      newTxSenders = accounts.slice(6, 8);

      const oldNodes = await Promise.all(
        oldSigners.map(async (s, i) => makeNode(Wallet.createRandom().address, await s.getAddress(), i)),
      );
      newNodes = await Promise.all(
        newSigners.map(async (s, i) => makeNode(await newTxSenders[i].getAddress(), await s.getAddress(), i)),
      );

      proxyAddress = await deployFreshProtocolConfigProxy(deployer, oldNodes, {
        publicDecryption: 1,
        userDecryption: 1,
        kmsGen: 1,
        mpc: 2, // -> previousSignerThreshold = 3 - 2 + 1 = 2
      });
      protocolConfig = (await ethers.getContractAt('ProtocolConfig', proxyAddress)) as unknown as ProtocolConfig;
    });

    it('reports idle when no switch is in progress', async function () {
      const result = await inspectKmsContextSwitch(hre, proxyAddress, 0);
      expect(result.flow).to.equal('idle');
      expect(result.fullyLive).to.equal(true);
    });

    it('reports PENDING with outstanding new signers part-way through creation', async function () {
      const contextId = await defineSwitch();
      await confirmCreation(contextId, [newSigners[0]]);

      const result = await inspectKmsContextSwitch(hre, proxyAddress, 0);
      expect(result.flow).to.equal('context-switch');
      expect(result.pendingContextId).to.equal(contextId);
      expect(result.contextState).to.equal('PENDING');
      expect(result.newSignersConfirmed).to.have.lengthOf(1);
      expect(result.newSignersOutstanding).to.deep.equal([await newSigners[1].getAddress()]);
      expect(result.contextCreationQuorumReached).to.equal(false);
    });

    it('surfaces the (n - t + 1) old-side target and flags being stuck below it', async function () {
      const contextId = await defineSwitch();
      await confirmCreation(contextId, [...newSigners, oldSigners[0]]);

      const result = await inspectKmsContextSwitch(hre, proxyAddress, 0);
      expect(result.contextState).to.equal('PENDING');
      expect(result.newSignersOutstanding).to.have.lengthOf(0);
      expect(result.previousSignerThreshold).to.equal(2);
      expect(result.previousConfirmationCount).to.equal(1);
      expect(result.stuckBelowPreviousThreshold).to.equal(true);
      expect(result.contextCreationQuorumReached).to.equal(false);
    });

    it('reports CREATED once the creation quorum is reached, with the epoch still PENDING', async function () {
      const contextId = await defineSwitch();
      const epochId = await confirmCreation(contextId, [...newSigners, oldSigners[0], oldSigners[1]]);
      expect(epochId, 'creation quorum should emit NewKmsEpoch').to.not.be.undefined;

      const result = await inspectKmsContextSwitch(hre, proxyAddress, 0);
      expect(result.contextState).to.equal('CREATED');
      expect(result.contextCreationQuorumReached).to.equal(true);
      expect(result.pendingEpochId).to.equal(epochId);
      expect(result.epochState).to.equal('PENDING');
      expect(result.epochSignersOutstanding).to.have.lengthOf(newSigners.length);
      expect(result.fullyLive).to.equal(false);
    });

    it('reports fully live once the epoch is activated', async function () {
      const contextId = await defineSwitch();
      const epochId = await confirmCreation(contextId, [...newSigners, oldSigners[0], oldSigners[1]]);
      await confirmActivation(epochId!, newTxSenders);

      const result = await inspectKmsContextSwitch(hre, proxyAddress, 0);
      expect(result.flow).to.equal('idle');
      expect(result.fullyLive).to.equal(true);
      expect(result.activeContextId).to.equal(contextId);
      const [activeContextId, activeEpochId] = await protocolConfig.getCurrentKmsContextAndEpoch();
      expect(activeContextId).to.equal(contextId);
      expect(activeEpochId).to.equal(epochId);
    });

    it('distinguishes an aborted switch from one still in progress', async function () {
      const contextId = await defineSwitch();
      await confirmCreation(contextId, [newSigners[0]]);

      const inProgress = await inspectKmsContextSwitch(hre, proxyAddress, 0);
      expect(inProgress.aborted).to.equal(false);
      expect(inProgress.contextState).to.equal('PENDING');

      await (await (await asOwner()).abortPendingContext(contextId)).wait();

      const aborted = await inspectKmsContextSwitch(hre, proxyAddress, 0);
      expect(aborted.flow).to.equal('context-switch');
      expect(aborted.aborted).to.equal(true);
      expect(aborted.abortReason).to.equal('context-aborted');
    });
  });
});
