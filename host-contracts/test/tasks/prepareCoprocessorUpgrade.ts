import { expect } from 'chai';
import { Interface, type JsonRpcProvider } from 'ethers';
import { mkdtempSync, readFileSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';

import {
  type BlockReader,
  type BlockWindowResult,
  bufferLeadSeconds,
  bufferSatisfied,
  computeBlockWindow,
  parseIsoTimestampToEpochSeconds,
  readChainTip,
} from '../../tasks/utils/blockWindow';
import { parseChainsJson } from '../../tasks/utils/chainsFile';
import {
  type CoprocessorUpgradeInputs,
  EXPERIMENTAL_CHAIN_SET_WARNING,
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
  printCoprocessorUpgradeProposal,
  redactRpcUrl,
  resolveEnvironment,
} from '../../tasks/utils/coprocessorUpgradeProposal';
import { ENVIRONMENTS } from '../../tasks/utils/environments';

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
      expect(inputs.hostChains.map((c) => c.chainId)).to.deep.equal([11155111, 80002, 97]);
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

    it('keeps the environment default when no override is supplied', function () {
      process.env.GATEWAY_TESTNET_RPC_URL = 'https://gw.testnet.invalid';
      const resolved = resolveEnvironment('testnet');
      expect(resolved.chains.map((c) => c.chainId)).to.deep.equal([11155111, 80002, 97]);
      expect(resolved.chainSetSource).to.match(/testnet default/);
    });

    it('replaces the chain set with the override, including a chain the table never listed', function () {
      process.env.GATEWAY_TESTNET_RPC_URL = 'https://gw.testnet.invalid';
      process.env.HOODI_RPC_URL = 'https://hoodi.invalid';
      const resolved = resolveEnvironment('testnet', {
        source: '/etc/fhevm/chains.json (sha256:abc)',
        chains: [
          {
            chainId: 11155111,
            label: 'sepolia',
            minWindowSeconds: 1800,
            fallbackBlockTimeSeconds: 12,
            rpcUrlEnv: 'SEPOLIA_RPC_URL',
            defaultRpcUrl: 'https://sepolia.invalid',
          },
          {
            chainId: 560048,
            label: 'hoodi',
            minWindowSeconds: 1800,
            fallbackBlockTimeSeconds: 12,
            rpcUrlEnv: 'HOODI_RPC_URL',
          },
        ],
      });
      expect(resolved.chains.map((c) => c.chainId)).to.deep.equal([11155111, 560048]);
      expect(resolved.chains[1].rpcUrl).to.equal('https://hoodi.invalid');
      expect(resolved.chainSetSource).to.equal('/etc/fhevm/chains.json (sha256:abc)');
    });

    it('refuses an override that drops the canonical chain', function () {
      process.env.GATEWAY_TESTNET_RPC_URL = 'https://gw.testnet.invalid';
      expect(() =>
        resolveEnvironment('testnet', {
          source: 'test',
          chains: [
            {
              chainId: 97,
              label: 'bnb-testnet',
              minWindowSeconds: 300,
              fallbackBlockTimeSeconds: 0.45,
              rpcUrlEnv: 'BNB_TESTNET_RPC_URL',
              defaultRpcUrl: 'https://bnb.invalid',
            },
          ],
        }),
      ).to.throw(/canonical chain 11155111/);
    });

    it('names the missing env var for an override chain with no RPC', function () {
      process.env.GATEWAY_TESTNET_RPC_URL = 'https://gw.testnet.invalid';
      delete process.env.HOODI_RPC_URL;
      expect(() =>
        resolveEnvironment('testnet', {
          source: 'test',
          chains: [
            {
              chainId: 11155111,
              label: 'sepolia',
              minWindowSeconds: 1800,
              fallbackBlockTimeSeconds: 12,
              rpcUrlEnv: 'SEPOLIA_RPC_URL',
              defaultRpcUrl: 'https://sepolia.invalid',
            },
            {
              chainId: 560048,
              label: 'hoodi',
              minWindowSeconds: 1800,
              fallbackBlockTimeSeconds: 12,
              rpcUrlEnv: 'HOODI_RPC_URL',
            },
          ],
        }),
      ).to.throw(/HOODI_RPC_URL/);
    });

    it('lets the override win over LOCAL_HOST_CHAINS', function () {
      process.env.LOCAL_GATEWAY_RPC_URL = 'http://gw.local.invalid';
      process.env.LOCAL_HOST_RPC_URL = 'http://host.local.invalid';
      process.env.LOCAL_HOST_CHAINS = JSON.stringify([
        { chainId: 999, rpcUrl: 'http://scenario.invalid', fallbackBlockTimeSeconds: 1 },
      ]);
      const resolved = resolveEnvironment('local', {
        source: 'test',
        chains: [
          {
            chainId: 12345,
            label: 'host-local',
            minWindowSeconds: 0,
            fallbackBlockTimeSeconds: 1,
            rpcUrlEnv: 'LOCAL_HOST_RPC_URL',
          },
        ],
      });
      expect(resolved.chains.map((c) => c.chainId)).to.deep.equal([12345]);
      expect(resolved.chains[0].rpcUrl).to.equal('http://host.local.invalid');
    });
  });

  describe('parseChainsJson', function () {
    const ENTRY = {
      chainId: 11155111,
      label: 'sepolia',
      minWindowSeconds: 1800,
      fallbackBlockTimeSeconds: 12,
      rpcUrlEnv: 'SEPOLIA_RPC_URL',
    };
    const parse = (value: unknown) => () => parseChainsJson(JSON.stringify(value), 'chains.json');

    it('accepts a well-formed array and keeps its order', function () {
      const { chains } = parseChainsJson(
        JSON.stringify([ENTRY, { ...ENTRY, chainId: 97, label: 'bnb' }]),
        'chains.json',
      );
      expect(chains.map((c) => c.chainId)).to.deep.equal([11155111, 97]);
      expect(chains[0].minWindowSeconds).to.equal(1800);
    });

    it('accepts an optional defaultRpcUrl off mainnet', function () {
      const { chains } = parseChainsJson(JSON.stringify([{ ...ENTRY, defaultRpcUrl: 'https://public.invalid' }]), 'x');
      expect(chains[0].defaultRpcUrl).to.equal('https://public.invalid');
    });

    it('refuses a defaultRpcUrl on mainnet, where the operator must supply the secret', function () {
      expect(() =>
        parseChainsJson(JSON.stringify([{ ...ENTRY, defaultRpcUrl: 'https://keyed.invalid' }]), 'x', 'mainnet'),
      ).to.throw(/defaultRpcUrl is not allowed on mainnet/);
      // Same file is fine for any other environment.
      expect(() =>
        parseChainsJson(JSON.stringify([{ ...ENTRY, defaultRpcUrl: 'https://public.invalid' }]), 'x', 'testnet'),
      ).to.not.throw();
    });

    it('reads the canonical chain from a marked entry', function () {
      const marked = parseChainsJson(JSON.stringify([ENTRY, { ...ENTRY, chainId: 97, canonical: true }]), 'x');
      expect(marked.canonicalChainId).to.equal(97);
      expect(parseChainsJson(JSON.stringify([ENTRY]), 'x').canonicalChainId).to.equal(undefined);
    });

    it('refuses two canonical marks and a non-boolean mark', function () {
      expect(
        parse([
          { ...ENTRY, canonical: true },
          { ...ENTRY, chainId: 97, canonical: true },
        ]),
      ).to.throw(/marks both chain 11155111 and chain 97 as canonical/);
      expect(parse([{ ...ENTRY, canonical: 'yes' }])).to.throw(/canonical must be true or false/);
    });

    it('rejects malformed JSON, a non-array and an empty array', function () {
      expect(() => parseChainsJson('{', 'chains.json')).to.throw(/is not valid JSON/);
      expect(parse({ chainId: 1 })).to.throw(/must be a JSON array/);
      expect(parse([])).to.throw(/empty array/);
    });

    it('rejects a missing or malformed field, naming the entry', function () {
      expect(parse([{ ...ENTRY, chainId: 0 }])).to.throw(/chains\[0\]\.chainId must be an integer greater than 0/);
      expect(parse([{ ...ENTRY, chainId: 1.5 }])).to.throw(/chainId/);
      expect(parse([ENTRY, { ...ENTRY, chainId: 97, label: '' }])).to.throw(/chains\[1\]\.label/);
      expect(parse([{ ...ENTRY, minWindowSeconds: -1 }])).to.throw(/minWindowSeconds/);
      expect(parse([{ ...ENTRY, fallbackBlockTimeSeconds: 0 }])).to.throw(/fallbackBlockTimeSeconds/);
      expect(parse([{ ...ENTRY, rpcUrlEnv: undefined }])).to.throw(/rpcUrlEnv/);
    });

    it('rejects an unknown key, which would otherwise be dropped in silence', function () {
      expect(parse([{ ...ENTRY, minWindowSecond: 1800 }])).to.throw(/unknown key "minWindowSecond"/);
    });

    it('rejects a duplicate chain id', function () {
      expect(parse([ENTRY, ENTRY])).to.throw(/lists chain 11155111 twice/);
    });
  });

  describe('chains.example.json', function () {
    const EXAMPLE_PATH = join(__dirname, '../../chains.example.json');
    const HOODI_CHAIN_ID = 560048;
    const { chains: example } = parseChainsJson(readFileSync(EXAMPLE_PATH, 'utf8'), 'chains.example.json');

    it('parses, so the template an operator copies is never malformed', function () {
      expect(example.length).to.be.greaterThan(0);
    });

    // Update chains.example.json whenever the testnet chains change.
    it('opens with the testnet default set, so the sample stays current', function () {
      const testnet = ENVIRONMENTS.testnet.chains;
      expect(example.slice(0, testnet.length)).to.deep.equal(testnet);
      // A dropped testnet chain leaves a stale entry behind, so pin the count too.
      expect(example.length).to.equal(testnet.length + 1);
    });

    // The extra entry shows the one case for --chains-file: a chain outside every environment.
    it('adds hoodi, which no environment defines', function () {
      expect(example.map((c) => c.chainId)).to.include(HOODI_CHAIN_ID);
      for (const [name, def] of Object.entries(ENVIRONMENTS)) {
        expect(
          def.chains.map((c) => c.chainId),
          name,
        ).to.not.include(HOODI_CHAIN_ID);
      }
    });

    it('loads with no env var beyond the gateway, since every entry carries a default', function () {
      process.env.GATEWAY_TESTNET_RPC_URL = 'https://gw.testnet.invalid';
      const inputs = parseCoprocessorUpgradeInputs({ ...TESTNET_PARAMS, chainsFile: EXAMPLE_PATH });
      expect(inputs.hostChains.map((c) => c.chainId)).to.deep.equal(example.map((c) => c.chainId));
    });

    it('covers every environment the table defines', function () {
      // A required field added to EnvChainEntry has to reach the parser, or real data fails to load.
      for (const [name, def] of Object.entries(ENVIRONMENTS)) {
        expect(() => parseChainsJson(JSON.stringify(def.chains), name, name), name).to.not.throw();
      }
    });
  });

  describe('chain set override plumbing', function () {
    const chainsJson = JSON.stringify([
      {
        chainId: 11155111,
        label: 'sepolia',
        minWindowSeconds: 1800,
        fallbackBlockTimeSeconds: 12,
        rpcUrlEnv: 'SEPOLIA_RPC_URL',
        defaultRpcUrl: 'https://sepolia.invalid',
      },
    ]);

    function writeChainsFile(contents = chainsJson): string {
      const path = join(mkdtempSync(join(tmpdir(), 'chains-')), 'chains.json');
      writeFileSync(path, contents);
      return path;
    }

    beforeEach(function () {
      process.env.GATEWAY_TESTNET_RPC_URL = 'https://gw.testnet.invalid';
    });

    it('reads the set from a file outside the repo and records its path and digest', function () {
      const path = writeChainsFile();
      const inputs = parseCoprocessorUpgradeInputs({ ...TESTNET_PARAMS, chainsFile: path });
      expect(inputs.hostChains.map((c) => c.chainId)).to.deep.equal([11155111]);
      expect(inputs.chainSetSource).to.match(new RegExp(`^${path} \\(sha256:[0-9a-f]{12}\\)$`));
    });

    it('reads the set from a named env var with env:NAME', function () {
      process.env.HOST_CHAINS = chainsJson;
      const inputs = parseCoprocessorUpgradeInputs({ ...TESTNET_PARAMS, chainsFile: 'env:HOST_CHAINS' });
      expect(inputs.hostChains.map((c) => c.chainId)).to.deep.equal([11155111]);
      expect(inputs.chainSetSource).to.match(/^HOST_CHAINS env var \(sha256:[0-9a-f]{12}\)$/);
    });

    it('ignores an env var the caller did not name', function () {
      // A stray variable must never replace the chain set on its own.
      process.env.HOST_CHAINS = chainsJson;
      const inputs = parseCoprocessorUpgradeInputs(TESTNET_PARAMS);
      expect(inputs.hostChains.map((c) => c.chainId)).to.deep.equal([11155111, 80002, 97]);
      expect(inputs.chainSetSource).to.match(/testnet default/);
    });

    it('reports an env:NAME that is unset, empty or nameless', function () {
      delete process.env.HOST_CHAINS;
      const run = (arg: string) => () => parseCoprocessorUpgradeInputs({ ...TESTNET_PARAMS, chainsFile: arg });
      expect(run('env:HOST_CHAINS')).to.throw(/names env var HOST_CHAINS, which is unset or empty/);
      process.env.HOST_CHAINS = '   ';
      expect(run('env:HOST_CHAINS')).to.throw(/unset or empty/);
      expect(run('env:')).to.throw(/needs an env var name/);
    });

    it('keeps the environment default when no override is supplied', function () {
      const inputs = parseCoprocessorUpgradeInputs(TESTNET_PARAMS);
      expect(inputs.hostChains.map((c) => c.chainId)).to.deep.equal([11155111, 80002, 97]);
      expect(inputs.chainSetSource).to.match(/testnet default/);
    });

    it('reports an unreadable path as a file problem', function () {
      expect(() =>
        parseCoprocessorUpgradeInputs({ ...TESTNET_PARAMS, chainsFile: '/nonexistent/chains.json' }),
      ).to.throw(/--chains-file .*nonexistent.* could not be read/);
    });

    it('still applies the finality floor the file states', function () {
      const path = writeChainsFile();
      expect(() => parseCoprocessorUpgradeInputs({ ...TESTNET_PARAMS, chainsFile: path, duration: '29m' })).to.throw(
        /below the minimum window of chain 11155111/,
      );
    });

    it('marks a file-supplied set experimental, and the default set not', function () {
      const path = writeChainsFile();
      expect(parseCoprocessorUpgradeInputs({ ...TESTNET_PARAMS, chainsFile: path }).chainSetFromFile).to.be.true;
      expect(parseCoprocessorUpgradeInputs(TESTNET_PARAMS).chainSetFromFile).to.be.false;
    });

    it('prints the experimental warning beside the calldata, and only for a file', function () {
      const lines: string[] = [];
      const original = console.log;
      console.log = (...args: unknown[]) => void lines.push(args.join(' '));
      try {
        for (const fromFile of [true, false]) {
          lines.length = 0;
          const inputs = { ...parseCoprocessorUpgradeInputs(TESTNET_PARAMS), chainSetFromFile: fromFile };
          printCoprocessorUpgradeProposal(
            {
              inputs,
              host: [makeHostReport(11155111, 1000, 1150)],
              gateway: makeGatewayReport(42),
              calldata: '0x',
            },
            '0xtarget',
          );
          expect(
            lines.some((l) => l.includes(EXPERIMENTAL_CHAIN_SET_WARNING)),
            `fromFile=${fromFile}`,
          ).to.equal(fromFile);
        }
      } finally {
        console.log = original;
      }
    });

    it('lets a file name its own canonical chain, which unblocks a non-default set', function () {
      process.env.LOCAL_GATEWAY_RPC_URL = 'http://gw.local.invalid';
      process.env.HOST_RPC_A = 'http://a.invalid';
      const entry = (chainId: number, canonical?: boolean) => ({
        chainId,
        label: `chain-${chainId}`,
        minWindowSeconds: 0,
        fallbackBlockTimeSeconds: 1,
        rpcUrlEnv: 'HOST_RPC_A',
        ...(canonical ? { canonical: true } : {}),
      });
      const path = writeChainsFile(JSON.stringify([entry(31337, true), entry(31338)]));
      const inputs = parseCoprocessorUpgradeInputs({
        ...TESTNET_PARAMS,
        environment: 'local',
        chainsFile: path,
      });
      expect(inputs.canonicalChainId).to.equal(31337);
      // Without the marker the same set fails, since local's canonical chain is 12345.
      const unmarked = writeChainsFile(JSON.stringify([entry(31337), entry(31338)]));
      expect(() =>
        parseCoprocessorUpgradeInputs({ ...TESTNET_PARAMS, environment: 'local', chainsFile: unmarked }),
      ).to.throw(/must include the canonical chain 12345/);
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
