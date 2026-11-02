import { expect } from 'chai';
import { Interface, type JsonRpcProvider } from 'ethers';

import {
  type BlockReader,
  type BlockWindowResult,
  bufferLeadSeconds,
  bufferSatisfied,
  computeBlockWindow,
  parseIsoTimestampToEpochSeconds,
  readChainTip,
} from '../../tasks/utils/blockWindow';
import {
  type CoprocessorUpgradeInputs,
  type GatewayReport,
  type HostReport,
  type NetworkReader,
  PROPOSE_UPGRADE_ABI,
  assertRpcChainId,
  assertWindowCoversFinality,
  bufferViolations,
  encodeProposeCoprocessorUpgrade,
  isReleaseTag,
  parseCoprocessorUpgradeInputs,
  redactRpcUrl,
  resolveEnvironment,
} from '../../tasks/utils/coprocessorUpgradeProposal';

// Window computation hits real RPC (covered by the workflow); these tests cover the RPC-free
// surface: env resolution, input validation, the calldata encoder (decode round-trip), and the buffer gate.

function makeResult(startBlock: number, endBlock: number): BlockWindowResult {
  return {
    currentTipBlock: 100,
    currentTipTimestamp: 1_700_000_000,
    computedAtEpochSeconds: 1_700_000_000,
    tipAgeSeconds: 0,
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
    label: `chain-${chainId}`,
    rpcUrl: 'https://example.invalid',
    result: makeResult(startBlock, endBlock),
    bufferOk,
    driftWarn: false,
  };
}

function makeGatewayReport(tipBlock: number): GatewayReport {
  return { rpcUrl: 'https://gw.invalid', tipBlock, tipTimestamp: 1_700_000_000, startBlock: tipBlock };
}

// A chain whose tip is `tipBlock` at `tipTimestamp`, with `blockTime` seconds per block before it.
// `sampleFails` makes the historical read reject, as a flaky RPC would.
function stubChain(tipBlock: number, tipTimestamp: number, blockTime: number, sampleFails = false): JsonRpcProvider {
  return {
    async getBlockNumber() {
      return tipBlock;
    },
    async getBlock(block: number) {
      if (block === tipBlock) return { timestamp: tipTimestamp };
      if (sampleFails) throw new Error('rpc hiccup');
      return { timestamp: tipTimestamp - (tipBlock - block) * blockTime };
    },
  } as unknown as JsonRpcProvider;
}

async function expectRejection(promise: Promise<unknown>, pattern: RegExp): Promise<void> {
  let error: unknown;
  try {
    await promise;
  } catch (e) {
    error = e;
  }
  expect(String(error)).to.match(pattern);
}

function stubNetworkReader(chainId: bigint | Error): NetworkReader {
  return {
    async getNetwork() {
      if (chainId instanceof Error) throw chainId;
      return { chainId };
    },
  } as unknown as NetworkReader;
}

const TESTNET_PARAMS = {
  environment: 'testnet',
  startTime: '2026-07-01T12:00:00Z',
  duration: '30m',
  buffer: '1h',
  proposalId: '5',
  softwareVersion: 'v0.14.0',
};

function stubBlockReader(tipBlock: number, tipTimestamp: number | null): BlockReader {
  return {
    async getBlockNumber() {
      return tipBlock;
    },
    async getBlock(block: number | string) {
      return block === tipBlock && tipTimestamp !== null ? { timestamp: tipTimestamp } : null;
    },
  } as unknown as BlockReader;
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
    it('is empty when every chain satisfies the buffer', function () {
      const proposal = {
        inputs: {} as CoprocessorUpgradeInputs,
        host: [makeHostReport(11155111, 1000, 1150, true)],
        gateway: makeGatewayReport(9000),
        calldata: '0x',
      };
      expect(bufferViolations(proposal)).to.deep.equal([]);
    });

    it('names each violating chain and ignores the gateway', function () {
      const proposal = {
        inputs: {} as CoprocessorUpgradeInputs,
        host: [makeHostReport(11155111, 1000, 1150, false), makeHostReport(80002, 5000, 6200, true)],
        gateway: makeGatewayReport(9000),
        calldata: '0x',
      };
      expect(bufferViolations(proposal)).to.deep.equal(['chain 11155111']);
    });
  });

  describe('readChainTip', function () {
    it('returns the tip as-is even when its timestamp is hours stale (idle on-demand Gateway)', async function () {
      const tenHoursAgo = Math.floor(Date.now() / 1000) - 10 * 3600;
      const tip = await readChainTip(stubBlockReader(22889, tenHoursAgo));
      expect(tip).to.deep.equal({ tipBlock: 22889, tipTimestamp: tenHoursAgo });
    });

    it('throws when the tip block cannot be fetched', async function () {
      let error: unknown;
      try {
        await readChainTip(stubBlockReader(42, null));
      } catch (e) {
        error = e;
      }
      expect(String(error)).to.match(/Failed to fetch tip block #42/);
    });
  });

  describe('parseCoprocessorUpgradeInputs', function () {
    it('parses valid params, resolving the environment RPC URLs from env vars', function () {
      process.env.GATEWAY_TESTNET_RPC_URL = 'https://gw.testnet.invalid';
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
      expect(inputs.hostChains.map((c) => c.chainId)).to.deep.equal([11155111, 80002, 560048, 97]);
      expect(inputs.hostChains[0].label).to.equal('sepolia');
      expect(inputs.gateway.rpcUrl).to.equal('https://gw.testnet.invalid');
      expect(inputs.softwareVersion).to.equal('v0.14.0');
    });

    it('resolves the ProtocolConfig chain and the gateway identity from the registry', function () {
      process.env.GATEWAY_TESTNET_RPC_URL = 'https://gw.testnet.invalid';
      const inputs = parseCoprocessorUpgradeInputs(TESTNET_PARAMS);
      expect(inputs.canonicalChainId).to.equal(11155111);
      expect(inputs.gateway.label).to.equal('gateway-testnet');
      expect(inputs.gateway.chainId).to.equal(10901);
    });

    it('takes the first scenario chain as the ProtocolConfig chain and labels unnamed chains (local)', function () {
      process.env.LOCAL_GATEWAY_RPC_URL = 'http://gw.local.invalid';
      process.env.LOCAL_HOST_CHAINS = JSON.stringify([
        { chainId: 777, rpcUrl: 'http://a.invalid', fallbackBlockTimeSeconds: 1 },
        { chainId: 778, label: 'polygon-preview', rpcUrl: 'http://b.invalid', fallbackBlockTimeSeconds: 1 },
      ]);
      const inputs = parseCoprocessorUpgradeInputs({ ...TESTNET_PARAMS, environment: 'local' });
      expect(inputs.canonicalChainId).to.equal(777);
      expect(inputs.hostChains.map((c) => c.label)).to.deep.equal(['chain-777', 'polygon-preview']);
      expect(inputs.gateway.chainId).to.equal(undefined);
    });

    it('trims the software version and rejects one that is not a release tag', function () {
      process.env.GATEWAY_TESTNET_RPC_URL = 'https://gw.testnet.invalid';
      expect(
        parseCoprocessorUpgradeInputs({ ...TESTNET_PARAMS, softwareVersion: ' v0.15.0 ' }).softwareVersion,
      ).to.equal('v0.15.0');
      expect(() => parseCoprocessorUpgradeInputs({ ...TESTNET_PARAMS, softwareVersion: 'v0-15-0' })).to.throw(
        /not a release tag/,
      );
    });

    it('rejects a start time without a time zone', function () {
      process.env.GATEWAY_TESTNET_RPC_URL = 'https://gw.testnet.invalid';
      expect(() => parseCoprocessorUpgradeInputs({ ...TESTNET_PARAMS, startTime: '2026-07-01T12:00:00' })).to.throw(
        /no time zone/,
      );
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
      process.env.GATEWAY_TESTNET_RPC_URL = 'https://gw.testnet.invalid';
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
      delete process.env.GATEWAY_DEVNET_RPC_URL;
      expect(() => resolveEnvironment('devnet')).to.throw(/GATEWAY_DEVNET_RPC_URL/);
    });
  });

  describe('computeBlockWindow', function () {
    const tip = 10_000;
    const tipTs = 1_700_000_000;

    it('measures the DAO lead from the wall clock, not from the tip', async function () {
      const now = tipTs + 30;
      const result = await computeBlockWindow({
        provider: stubChain(tip, tipTs, 12),
        startEpochSeconds: now + 3600,
        durationSeconds: 1800,
        bufferSeconds: 3000,
        fallbackBlockTimeSeconds: 12,
        nowEpochSeconds: now,
      });
      expect(result.observedBlockTimeSeconds).to.equal(12);
      expect(result.tipAgeSeconds).to.equal(30);
      expect(result.computedAtEpochSeconds).to.equal(now);
      // The lead is start minus now, within one block of rounding; the tip lag is not added to it.
      expect(Math.abs(bufferLeadSeconds(result) - 3600)).to.be.at.most(12);
      expect(bufferSatisfied(result, 3000)).to.equal(true);
      expect(bufferSatisfied(result, 3700)).to.equal(false);
    });

    it('measures the DAO lead from the tip when the local clock runs behind it', async function () {
      const result = await computeBlockWindow({
        provider: stubChain(tip, tipTs, 12),
        startEpochSeconds: tipTs + 3600,
        durationSeconds: 1800,
        bufferSeconds: 0,
        fallbackBlockTimeSeconds: 12,
        nowEpochSeconds: tipTs - 600,
      });
      expect(result.tipAgeSeconds).to.equal(-600);
      // Measured from the clock, the lead would read 4200s; the tip caps it at the real 3600s.
      expect(Math.abs(bufferLeadSeconds(result) - 3600)).to.be.at.most(12);
      expect(bufferSatisfied(result, 3700)).to.equal(false);
    });

    it('rejects a tip the RPC left behind', async function () {
      await expectRejection(
        computeBlockWindow({
          provider: stubChain(tip, tipTs, 12),
          startEpochSeconds: tipTs + 7200,
          durationSeconds: 1800,
          bufferSeconds: 0,
          fallbackBlockTimeSeconds: 12,
          nowEpochSeconds: tipTs + 600,
        }),
        /chain tip is 600s old .* allowed 120s/,
      );
    });

    it('falls back to the configured block time when the historical read fails', async function () {
      const result = await computeBlockWindow({
        provider: stubChain(tip, tipTs, 12, true),
        startEpochSeconds: tipTs + 3600,
        durationSeconds: 1800,
        bufferSeconds: 0,
        fallbackBlockTimeSeconds: 15,
        nowEpochSeconds: tipTs,
      });
      expect(result.usedFallback).to.equal(true);
      expect(result.effectiveBlockTimeSeconds).to.equal(15);
      expect(result.endBlock - result.startBlock).to.equal(120);
    });
  });

  describe('assertWindowCoversFinality', function () {
    const chain = (chainId: number, minWindowSeconds = 0) => ({
      chainId,
      label: `chain-${chainId}`,
      rpcUrl: 'https://x.invalid',
      fallbackBlockTimeSeconds: 12,
      minWindowSeconds,
    });

    it('names the chain whose floor the duration does not reach', function () {
      expect(() => assertWindowCoversFinality(600, [chain(56, 300), chain(1, 1800)])).to.throw(
        /below the minimum window of chain 1 \(1800s\)/,
      );
    });

    it('passes at the floor and when a chain has none', function () {
      assertWindowCoversFinality(1800, [chain(56, 300), chain(1, 1800)]);
      assertWindowCoversFinality(60, [chain(12345), chain(777, 0)]);
    });

    it('is applied from the registry floors when parsing inputs', function () {
      process.env.GATEWAY_TESTNET_RPC_URL = 'https://gw.testnet.invalid';
      const params = {
        environment: 'testnet',
        startTime: '2026-07-01T12:00:00Z',
        duration: '10m',
        buffer: '1h',
        proposalId: '5',
        softwareVersion: 'v0.15.0',
      };
      expect(() => parseCoprocessorUpgradeInputs(params)).to.throw(/below the minimum window of chain 11155111/);
      expect(parseCoprocessorUpgradeInputs({ ...params, duration: '30m' }).durationSeconds).to.equal(1800);
    });
  });

  describe('parseIsoTimestampToEpochSeconds', function () {
    it('accepts Z and explicit offsets', function () {
      const expected = Date.UTC(2026, 6, 1, 12, 0, 0) / 1000;
      expect(parseIsoTimestampToEpochSeconds('2026-07-01T12:00:00Z')).to.equal(expected);
      expect(parseIsoTimestampToEpochSeconds('2026-07-01T14:00:00+02:00')).to.equal(expected);
    });

    it('rejects a timestamp without a time zone (it would be read as local time)', function () {
      expect(() => parseIsoTimestampToEpochSeconds('2026-07-01T12:00:00')).to.throw(/no time zone/);
    });
  });

  describe('isReleaseTag', function () {
    it('accepts release tags with or without the v prefix and a pre-release suffix', function () {
      for (const tag of ['v0.15.0', '0.15.0', 'v0.15.0-rc.1']) {
        expect(isReleaseTag(tag), tag).to.equal(true);
      }
    });

    it('rejects shapes the coprocessor would parse as zeros', function () {
      for (const tag of ['v0-15-0', 'v0.15', '0.15.0.1', 'latest', '']) {
        expect(isReleaseTag(tag), tag).to.equal(false);
      }
    });
  });

  describe('assertRpcChainId', function () {
    it('passes when the RPC reports the registry chain id', async function () {
      await assertRpcChainId(stubNetworkReader(1n), 1, 'ethereum');
    });

    it('rejects an RPC that serves another chain', async function () {
      await expectRejection(
        assertRpcChainId(stubNetworkReader(137n), 1, 'ethereum'),
        /reports chain id 137, expected 1/,
      );
    });

    it('reports a dead RPC as an RPC problem', async function () {
      await expectRejection(
        assertRpcChainId(stubNetworkReader(new Error('ECONNREFUSED')), 1, 'ethereum'),
        /did not answer eth_chainId/,
      );
    });
  });

  describe('redactRpcUrl', function () {
    it('keeps the scheme and host only', function () {
      expect(redactRpcUrl('https://eth-mainnet.g.alchemy.com/v2/SECRET-KEY?x=1')).to.equal(
        'https://eth-mainnet.g.alchemy.com',
      );
      expect(redactRpcUrl('https://user:pw@rpc.invalid:8545/path')).to.equal('https://rpc.invalid:8545');
      expect(redactRpcUrl('not a url')).to.equal('<unparsable url>');
    });
  });
});
