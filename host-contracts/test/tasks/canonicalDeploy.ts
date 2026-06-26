import { expect } from 'chai';
import { Wallet } from 'ethers';
import fs from 'fs';
import hre, { ethers, run, upgrades } from 'hardhat';
import os from 'os';
import path from 'path';

import { buildSnapshotArtifact, readCanonicalSnapshot } from '../../tasks/protocolConfigMirror';
import { makeEnvHelpers } from '../../tasks/utils/envSnapshot';
import { getRequiredEnvVar } from '../../tasks/utils/loadVariables';
import { UPGRADE_TO_AND_CALL_INTERFACE } from '../../tasks/utils/upgradeProposal';
import { deployEmptyProxy } from '../utils/deploymentHelpers';
import {
  HOST_ENV_FILE,
  buildProtocolConfigNodes,
  buildProtocolConfigThresholds,
  deployFreshProtocolConfigProxy,
  deployFreshUninitializedProtocolConfigProxy,
  initializeProtocolConfigProxy,
  readHostAddress,
} from './taskHelpers';

async function deployEmptyUUPSProxy(deployer: Wallet): Promise<string> {
  const factory = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
  return deployEmptyProxy(factory);
}

/**
 * Replace a single key in the .env.host file.
 */
function patchHostEnv(key: string, value: string): void {
  const content = fs.readFileSync(HOST_ENV_FILE, 'utf-8');
  const updated = content.replace(new RegExp(`${key}=.*`), `${key}=${value}`);
  fs.writeFileSync(HOST_ENV_FILE, updated);
}

const HOST_ADDRESS_ENV_KEYS = [
  'PROTOCOL_CONFIG_CONTRACT_ADDRESS',
  'KMS_GENERATION_CONTRACT_ADDRESS',
  'KMS_VERIFIER_CONTRACT_ADDRESS',
] as const;

const { snapshot: snapshotHostAddressEnv, restore: restoreHostAddressEnv } = makeEnvHelpers(HOST_ADDRESS_ENV_KEYS);
type HostAddressEnvSnapshot = ReturnType<typeof snapshotHostAddressEnv>;

async function readImplementationSlot(proxyAddress: string): Promise<string> {
  return upgrades.erc1967.getImplementationAddress(proxyAddress);
}

describe('Canonical prepare tasks', function () {
  const deployerPrivateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(deployerPrivateKey).connect(ethers.provider);
  let originalEnvHost: string;
  let originalHostAddressEnv: HostAddressEnvSnapshot;

  before(function () {
    originalEnvHost = fs.readFileSync(HOST_ENV_FILE, 'utf-8');
  });

  beforeEach(function () {
    originalHostAddressEnv = snapshotHostAddressEnv();
  });

  afterEach(async function () {
    // Restore .env.host so other tests are unaffected.
    fs.writeFileSync(HOST_ENV_FILE, originalEnvHost);
    restoreHostAddressEnv(originalHostAddressEnv);
  });

  describe('ProtocolConfig from canonical artifact', function () {
    async function writeCanonicalArtifact(): Promise<{ file: string; canonicalAddress: string }> {
      const canonicalAddress = await deployFreshProtocolConfigProxy(
        deployer,
        buildProtocolConfigNodes(),
        buildProtocolConfigThresholds(),
      );
      const snapshot = await readCanonicalSnapshot(hre, {
        canonicalProvider: ethers.provider,
        canonicalProtocolConfigAddress: canonicalAddress,
      });
      const artifact = buildSnapshotArtifact(snapshot, canonicalAddress);
      const file = path.join(fs.mkdtempSync(path.join(os.tmpdir(), 'canonical-snapshot-')), 'snapshot.json');
      fs.writeFileSync(file, JSON.stringify(artifact, null, 2));
      return { file, canonicalAddress };
    }

    it('prepares DAO calldata from a reviewed artifact without mutating the proxy', async function () {
      const proxyAddress = await deployEmptyUUPSProxy(deployer);
      patchHostEnv('PROTOCOL_CONFIG_CONTRACT_ADDRESS', proxyAddress);
      const { file, canonicalAddress } = await writeCanonicalArtifact();
      const canonical = await ethers.getContractAt('ProtocolConfig', canonicalAddress, deployer);
      const [canonicalContextId, canonicalEpochId] = await canonical.getCurrentKmsContextAndEpoch();

      const implementationSlotBefore = await readImplementationSlot(proxyAddress);
      const preparedUpgrade = await run('task:prepareDeployProtocolConfigFromCanonical', {
        snapshot: file,
        verifyContract: false,
      });
      expect(await readImplementationSlot(proxyAddress)).to.equal(implementationSlotBefore);

      const { newImplementationAddress, innerFunctionSignature, innerCalldata, outerCalldata } = preparedUpgrade;
      expect(preparedUpgrade.proxyAddress).to.equal(proxyAddress);
      const iface = new ethers.Interface([`function ${innerFunctionSignature}`]);
      const decoded = iface.decodeFunctionData('initializeFromCanonical', innerCalldata);
      expect(decoded[0]).to.equal(canonicalContextId);
      expect(decoded[1]).to.equal(canonicalEpochId);
      // Round-trip every KmsNodeParams field (4 stored + 4 MPC-metadata) so a dropped or transposed
      // field is caught instead of silently passing on txSenderAddress alone. The canonical mirror
      // path preserves the four stored fields and fills the MPC metadata with deterministic
      // placeholders (partyId = index, mpcIdentity = '', caCert = '0x', storagePrefix = '').
      const expectedNodes = buildProtocolConfigNodes();
      expect(decoded[2].length).to.equal(expectedNodes.length);
      decoded[2].forEach((node: unknown[], index: number) => {
        const [txSenderAddress, signerAddress, ipAddress, storageUrl, partyId, mpcIdentity, caCert, storagePrefix] =
          node;
        expect(txSenderAddress).to.equal(expectedNodes[index].txSenderAddress);
        expect(signerAddress).to.equal(expectedNodes[index].signerAddress);
        expect(ipAddress).to.equal(expectedNodes[index].ipAddress);
        expect(storageUrl).to.equal(expectedNodes[index].storageUrl);
        expect(partyId).to.equal(BigInt(index));
        expect(mpcIdentity).to.equal('');
        expect(caCert).to.equal('0x');
        expect(storagePrefix).to.equal('');
      });
      expect(decoded[3][0]).to.equal(BigInt(buildProtocolConfigThresholds().publicDecryption));
      expect(outerCalldata).to.equal(
        UPGRADE_TO_AND_CALL_INTERFACE.encodeFunctionData('upgradeToAndCall', [newImplementationAddress, innerCalldata]),
      );
    });

    it('applies a reviewed artifact with the devnet deploy task, without canonical RPC access', async function () {
      const proxyAddress = await deployEmptyUUPSProxy(deployer);
      patchHostEnv('PROTOCOL_CONFIG_CONTRACT_ADDRESS', proxyAddress);
      const { file, canonicalAddress } = await writeCanonicalArtifact();
      const canonical = await ethers.getContractAt('ProtocolConfig', canonicalAddress, deployer);
      const [canonicalContextId, canonicalEpochId] = await canonical.getCurrentKmsContextAndEpoch();

      await run('task:deployProtocolConfigFromCanonical', { snapshot: file });

      const secondary = await ethers.getContractAt('ProtocolConfig', proxyAddress, deployer);
      expect(await secondary.getCurrentKmsContextId()).to.equal(canonicalContextId);
      const secondaryState = await secondary.getCurrentKmsContextAndEpoch();
      expect(secondaryState[0]).to.equal(canonicalContextId);
      expect(secondaryState[1]).to.equal(canonicalEpochId);
      expect((await secondary.getKmsNodesForContext(canonicalContextId)).length).to.equal(
        buildProtocolConfigNodes().length,
      );
      expect(await secondary.getPublicDecryptionThresholdForContext(canonicalContextId)).to.equal(
        BigInt(buildProtocolConfigThresholds().publicDecryption),
      );
    });

    it('rejects when neither a snapshot nor canonical RPC parameters are given', async function () {
      await expect(run('task:deployProtocolConfigFromCanonical')).to.be.rejectedWith(/either --snapshot/);
    });
  });

  describe('KMSVerifier deployment', function () {
    it('passes the standalone readiness check when ProtocolConfig is initialized', async function () {
      const protocolConfigProxyAddress = await deployEmptyUUPSProxy(deployer);

      patchHostEnv('PROTOCOL_CONFIG_CONTRACT_ADDRESS', protocolConfigProxyAddress);

      await initializeProtocolConfigProxy(
        protocolConfigProxyAddress,
        deployer,
        buildProtocolConfigNodes(),
        buildProtocolConfigThresholds(),
      );

      await run('task:assertProtocolConfigReady');
    });

    it('rejects the standalone readiness check when ProtocolConfig is not initialized', async function () {
      const protocolConfigProxyAddress = await deployEmptyUUPSProxy(deployer);

      patchHostEnv('PROTOCOL_CONFIG_CONTRACT_ADDRESS', protocolConfigProxyAddress);

      await expect(run('task:assertProtocolConfigReady')).to.be.rejectedWith(
        `Cannot deploy KMSVerifier: Contract at ${protocolConfigProxyAddress} does not expose getVersion(); it is not a ProtocolConfig proxy.`,
      );
    });

    it('rejects the standalone readiness check when ProtocolConfig has no active KMS context', async function () {
      // getVersion() passes the identity check, but currentKmsContextId is still 0 (never initialized),
      // so the readiness task must reject before KMSVerifier deployment.
      const protocolConfigProxyAddress = await deployFreshUninitializedProtocolConfigProxy(deployer);

      patchHostEnv('PROTOCOL_CONFIG_CONTRACT_ADDRESS', protocolConfigProxyAddress);

      await expect(run('task:assertProtocolConfigReady')).to.be.rejectedWith(
        `Cannot deploy KMSVerifier: ProtocolConfig at ${protocolConfigProxyAddress} has no active KMS context (currentKmsContextId=0).`,
      );
    });

    it('rejects the standalone readiness check when pointed at a KMSVerifier address', async function () {
      const kmsVerifierAddress = readHostAddress('KMS_VERIFIER_CONTRACT_ADDRESS');

      patchHostEnv('PROTOCOL_CONFIG_CONTRACT_ADDRESS', kmsVerifierAddress);

      await expect(run('task:assertProtocolConfigReady')).to.be.rejectedWith(
        new RegExp(`Cannot deploy KMSVerifier: Contract at ${kmsVerifierAddress} reports version "KMSVerifier`),
      );
    });

    it('rejects deployment when ProtocolConfig is not initialized', async function () {
      const protocolConfigProxyAddress = await deployEmptyUUPSProxy(deployer);
      const kmsVerifierProxyAddress = await deployEmptyUUPSProxy(deployer);

      patchHostEnv('PROTOCOL_CONFIG_CONTRACT_ADDRESS', protocolConfigProxyAddress);
      patchHostEnv('KMS_VERIFIER_CONTRACT_ADDRESS', kmsVerifierProxyAddress);

      const kmsVerifierImplBefore = await upgrades.erc1967.getImplementationAddress(kmsVerifierProxyAddress);

      await expect(run('task:deployKMSVerifier')).to.be.rejectedWith(
        `Cannot deploy KMSVerifier: Contract at ${protocolConfigProxyAddress} does not expose getVersion(); it is not a ProtocolConfig proxy.`,
      );

      expect(await upgrades.erc1967.getImplementationAddress(kmsVerifierProxyAddress)).to.equal(kmsVerifierImplBefore);
    });

    it('prepares KMSVerifier upgrade calldata without mutating the proxy implementation', async function () {
      const kmsVerifierProxyAddress = readHostAddress('KMS_VERIFIER_CONTRACT_ADDRESS');
      const kmsVerifierImplBefore = await upgrades.erc1967.getImplementationAddress(kmsVerifierProxyAddress);

      await run('task:prepareUpgradeKMSVerifier', {
        currentImplementation: 'contracts/KMSVerifier.sol:KMSVerifier',
        newImplementation: 'contracts/KMSVerifier.sol:KMSVerifier',
        useInternalProxyAddress: true,
        verifyContract: false,
      });

      expect(await upgrades.erc1967.getImplementationAddress(kmsVerifierProxyAddress)).to.equal(kmsVerifierImplBefore);
    });
  });
});
