import { expect } from 'chai';
import { Interface } from 'ethers';

import type { BlockWindowResult } from '../../tasks/utils/blockWindow';
import {
  type CoprocessorUpgradeInputs,
  type GatewayReport,
  type HostReport,
  PROPOSE_UPGRADE_ABI,
  bufferViolations,
  encodeProposeCoprocessorUpgrade,
  parseCoprocessorUpgradeInputs,
  resolveEnvironment,
} from '../../tasks/utils/coprocessorUpgradeProposal';

// Window computation hits real RPC (covered by the workflow); these tests cover the RPC-free
// surface: env resolution, input validation, the calldata encoder (decode round-trip), and the buffer gate.

function makeResult(startBlock: number, endBlock: number): BlockWindowResult {
  return {
    currentTipBlock: 100,
    currentTipTimestamp: 1_700_000_000,
    observedBlockTimeSeconds: 12,
    effectiveBlockTimeSeconds: 12,
    startBlock,
    endBlock,
    startBlockEstimatedTimestamp: 1_700_000_000,
    endBlockEstimatedTimestamp: 1_700_001_800,
    startSkewSeconds: 0,
    usedFallback: false,
  };
}

function makeHostReport(chainId: number, startBlock: number, endBlock: number, bufferOk = true): HostReport {
  return {
    chainId,
    rpcUrl: 'https://example.invalid',
    result: makeResult(startBlock, endBlock),
    bufferOk,
    driftWarn: false,
  };
}

function makeGatewayReport(startBlock: number, bufferOk = true): GatewayReport {
  return { rpcUrl: 'https://gw.invalid', result: makeResult(startBlock, startBlock + 150), bufferOk, driftWarn: false };
}

describe('prepareCoprocessorUpgrade task utils', function () {
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

  describe('encodeProposeCoprocessorUpgrade', function () {
    it('encodes calldata that decodes back to the supplied windows (DAO path)', function () {
      const inputs = {
        proposalId: 7n,
        softwareVersion: 'v0.14.0',
      } as CoprocessorUpgradeInputs;
      const host = [makeHostReport(11155111, 1000, 1150), makeHostReport(80002, 5000, 6200)];
      const gateway = makeGatewayReport(9000);

      const calldata = encodeProposeCoprocessorUpgrade(inputs, { host, gateway });

      const iface = new Interface(PROPOSE_UPGRADE_ABI);
      const decoded = iface.decodeFunctionData('proposeCoprocessorUpgrade', calldata);
      expect(decoded[0]).to.equal(7n);
      expect(decoded[1]).to.equal('v0.14.0');
      expect(decoded[2].length).to.equal(2);
      expect(decoded[2][0][0]).to.equal(11155111n);
      expect(decoded[2][0][1]).to.equal(1000n);
      expect(decoded[2][0][2]).to.equal(1150n);
      expect(decoded[2][1][0]).to.equal(80002n);
      expect(decoded[2][1][1]).to.equal(5000n);
      expect(decoded[2][1][2]).to.equal(6200n);
      expect(decoded[3]).to.equal(9000n);
    });
  });

  describe('bufferViolations', function () {
    it('is empty when every chain and the gateway satisfy the buffer', function () {
      const proposal = {
        inputs: {} as CoprocessorUpgradeInputs,
        host: [makeHostReport(11155111, 1000, 1150, true)],
        gateway: makeGatewayReport(9000, true),
        calldata: '0x',
      };
      expect(bufferViolations(proposal)).to.deep.equal([]);
    });

    it('names each violating chain and the gateway', function () {
      const proposal = {
        inputs: {} as CoprocessorUpgradeInputs,
        host: [makeHostReport(11155111, 1000, 1150, false), makeHostReport(80002, 5000, 6200, true)],
        gateway: makeGatewayReport(9000, false),
        calldata: '0x',
      };
      expect(bufferViolations(proposal)).to.deep.equal(['chain 11155111', 'gateway']);
    });
  });

  describe('parseCoprocessorUpgradeInputs', function () {
    it('parses valid params, resolving the environment RPC URLs from env vars', function () {
      process.env.RPC_URL_GATEWAY_TESTNET = 'https://gw.testnet.invalid';
      const inputs = parseCoprocessorUpgradeInputs({
        environment: 'testnet',
        startTime: '2026-07-01T12:00:00Z',
        duration: '30m',
        buffer: '1h',
        proposalId: '5',
        softwareVersion: 'v0.14.0',
      });
      expect(inputs.environment).to.equal('testnet');
      expect(inputs.durationSeconds).to.equal(1800);
      expect(inputs.bufferSeconds).to.equal(3600);
      expect(inputs.proposalId).to.equal(5n);
      expect(inputs.hostChains.map((c) => c.chainId)).to.deep.equal([11155111, 80002]);
      expect(inputs.gateway.rpcUrl).to.equal('https://gw.testnet.invalid');
    });

    it('rejects an unknown environment', function () {
      expect(() =>
        parseCoprocessorUpgradeInputs({
          environment: 'nope',
          startTime: '2026-07-01T12:00:00Z',
          duration: '30m',
          buffer: '1h',
          proposalId: '1',
          softwareVersion: 'v0.14.0',
        }),
      ).to.throw(/--environment must be one of/);
    });

    it('rejects a non-positive proposal id', function () {
      process.env.RPC_URL_GATEWAY_TESTNET = 'https://gw.testnet.invalid';
      expect(() =>
        parseCoprocessorUpgradeInputs({
          environment: 'testnet',
          startTime: '2026-07-01T12:00:00Z',
          duration: '30m',
          buffer: '1h',
          proposalId: '0',
          softwareVersion: 'v0.14.0',
        }),
      ).to.throw(/must be > 0/);
    });
  });

  describe('resolveEnvironment', function () {
    it('requires the gateway RPC env var when no default exists (devnet)', function () {
      delete process.env.RPC_URL_GATEWAY_DEVNET;
      expect(() => resolveEnvironment('devnet')).to.throw(/RPC_URL_GATEWAY_DEVNET/);
    });
  });
});
