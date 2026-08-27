// `precomputeCreate2Addresses` against what the deploy coordinator actually seals.
//
// ## Why this test is the whole justification for the function
//
// The TypeScript predicts an address by hashing init code it assembles from the payload's TEMPLATES, with
// placeholders patched in. The coordinator predicts it by hashing init code `forge` produced by COMPILING
// `pkg/src` against a generated `addresses.sol`. Two different pipelines, and nothing forces their output
// to be byte-identical — but an address is `keccak256` of that output, so one divergent byte anywhere
// (metadata, a placeholder offset, an initializer selector) yields a different address that looks
// perfectly plausible and matches nothing.
//
// Reasoning cannot settle that; only running both and comparing can. So this runs the real coordinator's
// `compute` stage, reads the manifest it seals, and requires all thirteen roles to agree.
//
// It earned its place immediately. The first version of the function encoded the ACL proxy's init data with
// `ACL`'s own ABI — but a proxy is constructed over the EMPTY implementation, so the initializer belongs to
// `EmptyUUPSProxyACL`. `ACL.initialize` does not exist at all (its initializer is `initializeFromEmptyProxy`,
// called later during materialization), so that particular slip threw. Had the name happened to match, it
// would have silently produced a wrong address set.
//
// ## Why it lives here rather than in test/e2e/
//
// It imports from the PUBLISHED package, so it also proves `precomputeCreate2Addresses` is actually
// exported and usable by a consumer — and it drives `createEthersEthereumUtils`, which is where the three
// new `AbstractEthereumUtils` members are implemented. A test that imported the source would check neither.
//
// ## It deploys for real, and that is the point
//
// Comparing against the manifest alone would only prove the two PREDICTIONS agree. They could both be
// wrong in the same way — a salt preimage that omits a field, a factory that behaves differently — and a
// manifest is a prediction, not an observation. So this runs the full deploy and then checks the chain.
//
// The strongest available assertion is to feed the PREDICTED addresses into `verify()`: if the prediction
// is correct then a `Deployed` assembled purely from predictions describes the real stack, and every
// version, every baked-in address and the whole ownership chain must check out against it. If any single
// address were wrong, that call collapses — the reads would hit the wrong contract, or nothing at all.

import {
  CREATE2_ROLES,
  precomputeCreate2Addresses,
  verify,
  type AbstractEthereumHistory,
  type Deployed,
} from '@fhevm/host-contracts-cleartext/ts';
import { Interface, JsonRpcProvider } from 'ethers';
import { spawn } from 'node:child_process';
import { existsSync, readFileSync, rmSync } from 'node:fs';
import { join } from 'node:path';
import { expect, test } from 'vitest';
import { startAnvil, stopAnvil, waitForAnvil } from './utils/anvil.ts';
import { createEthersEthereumProvider, createEthersEthereumUtils } from './utils/ethersEthereumLib.ts';

/** The harness root, two levels up from `test/ts`. The coordinator must be spawned from there. */
const PACKAGE_ROOT = join(import.meta.dirname, '..', '..');

/** Resolved by the coordinator against `create2-deploy/`, the only directory forge may write to. */
const OUT_DIR_ARG = '.out-create2-precompute';
const OUT_DIR_ABS = join(PACKAGE_ROOT, 'create2-deploy', OUT_DIR_ARG);

/**
 * Clears the SEAL but keeps `build/`, which is forge's `--out` — i.e. its compilation cache.
 *
 * Wiping the whole out-dir is the obvious thing and it triples the runtime: a full run compiles twelve
 * times (three compute passes over the whole payload, then one per broadcasting stage), and with a cold
 * cache that is ~90s instead of ~30s. forge keys its cache on source hashes, so keeping it cannot serve
 * stale artifacts — while the manifest and the journal MUST go, or the run resumes a previous seal
 * instead of starting clean.
 */
function clearSealKeepingBuildCache(): void {
  for (const name of ['manifest.json', 'pass2.json', 'journal.jsonl', 'broadcast']) {
    rmSync(join(OUT_DIR_ABS, name), { recursive: true, force: true });
  }
}

type Manifest = {
  readonly address: Record<string, string>;
  readonly admin: string;
  readonly deployer: string;
  readonly deploymentId: string;
  readonly version: string;
  readonly factory: string;
};

/**
 * `AbstractEthereumHistory` over ethers.
 *
 * Supplied rather than omitted because the check that matters most here is the ERC-1967 one: a proxy's code
 * is identical before and after it is pointed at an implementation, so without `getStorageAt` "the address
 * is right" and "the address is right AND materialized" are indistinguishable. `getLogs` is only reached by
 * `mode: 'upgrade'`, but it is implemented properly anyway — a throwing stub in a shipped adapter is a
 * landmine for whoever copies it.
 */
function createEthersHistory(rpcUrl: string): AbstractEthereumHistory {
  const provider = new JsonRpcProvider(rpcUrl, undefined, { staticNetwork: true });
  return {
    getBlockNumber: async () => BigInt(await provider.getBlockNumber()),
    getStorageAt: async ({ address, slot }) => await provider.getStorage(address, slot),
    getLogs: async ({ address, abi, eventNames, fromBlock, toBlock }) => {
      const itf = new Interface(abi as never);
      const wanted = new Map(
        eventNames
          .map((name) => itf.getEvent(name))
          .filter((e): e is NonNullable<typeof e> => e !== null)
          .map((e) => [e.topicHash, e.name]),
      );
      if (wanted.size === 0) return [];
      const logs = await provider.getLogs({
        address,
        topics: [[...wanted.keys()]],
        fromBlock: Number(fromBlock),
        toBlock: toBlock === 'latest' ? 'latest' : Number(toBlock),
      });
      return logs.map((log) => ({ eventName: wanted.get(log.topics[0] ?? '') ?? '' }));
    },
  };
}

/**
 * Runs the coordinator, reporting progress on a timer and keeping the whole output for a failure message.
 *
 * A heartbeat rather than a tee, and the reason is measured: the coordinator emits ~500 lines, and
 * forwarding each one with its own `process.stdout.write` made the SAME command take 93s inside vitest
 * against 37s standalone. Vitest forwards a worker's stdout to the main process per write, so the writes,
 * not the work, were the cost. One line every few seconds carries the same reassurance for ~1% of the
 * writes; the full output is still accumulated, so a failure is quoted rather than lost.
 *
 * What the run is actually spending time on: twelve `solc` invocations — three compute passes over the
 * whole payload, then one per broadcasting stage. The transactions are the cheap part.
 */
async function runCoordinator(args: readonly string[], started: number): Promise<{ status: number; output: string }> {
  const child = spawn('node', [...args], { cwd: PACKAGE_ROOT, stdio: ['ignore', 'pipe', 'pipe'] });
  let output = '';
  let last = 'starting';
  const keep = (chunk: Buffer): void => {
    const text = chunk.toString('utf8');
    output += text;
    // Remember the most recent line worth showing, so the heartbeat says WHERE the run is.
    for (const line of text.split('\n')) {
      const t = line.trim();
      if (t === '' || t.startsWith('==') || t.startsWith('##')) continue;
      if (
        /^(Compiling|Solc|Script ran|Sensitive values|ONCHAIN|Traces|Gas used|Total|Estimated|Chain |Setting|Sequence|Transactions|Saved|Waiting)/.test(
          t,
        ) ||
        t.length < 90
      ) {
        last = t.slice(0, 88);
      }
    }
  };
  child.stdout.on('data', keep);
  child.stderr.on('data', keep);

  const beat = setInterval(() => {
    step(started, `still running — ${last}`);
  }, 5000);
  try {
    return await new Promise<{ status: number; output: string }>((resolve) => {
      child.on('close', (code) => {
        resolve({ status: code ?? 1, output });
      });
    });
  } finally {
    clearInterval(beat);
  }
}

/** Wall-clock label for the progress lines, so a slow phase is visibly slow rather than silent. */
function step(started: number, message: string): void {
  process.stdout.write(`   [${String(Math.round((Date.now() - started) / 1000)).padStart(3)}s] ${message}\n`);
}

test('precomputeCreate2Addresses predicts where a real create2 deploy actually lands', async () => {
  const t0 = Date.now();
  // A port no other test in this suite uses. `startAnvil` does NOT fail when the port is taken — anvil
  // exits, `waitForAnvil` then connects to whatever else is listening, and the run fails much later with
  // something unrelated (a deployer with no balance, if the squatter was funded from another mnemonic).
  // `test/anvil-ports.test.ts` is what keeps this unique.
  const anvil = startAnvil({ port: 8650 });
  try {
    await waitForAnvil(anvil.rpcUrl);
    clearSealKeepingBuildCache();

    step(t0, 'anvil up — running the create2 coordinator: 12 solc runs (3 compute passes + 9 stages), ~40s');
    const run = await runCoordinator(
      [
        'create2-deploy/deploy-testnet.ts',
        '--config',
        'create2-deploy/anvil-config.json',
        '--rpc-url',
        anvil.rpcUrl,
        '--out-dir',
        OUT_DIR_ARG,
        '--deployment-id',
        'precompute-parity',
        '--stage',
        'all',
      ],
      t0,
    );
    expect(run.status, `create2 deploy failed:\n${run.output.slice(-4000)}`).toBe(0);
    step(t0, 'deploy done — comparing the sealed manifest against precomputeCreate2Addresses');

    const manifestPath = join(OUT_DIR_ABS, 'manifest.json');
    expect(existsSync(manifestPath), `no manifest sealed at ${manifestPath}`).toBe(true);
    const manifest = JSON.parse(readFileSync(manifestPath, 'utf8')) as Manifest;

    // Every salt input is taken from the manifest rather than written here. Hardcoding the version or the
    // deployer would let this pass while the FUNCTION disagreed with a real run — the test would be
    // comparing the same inputs against themselves.
    const ethUtils = createEthersEthereumUtils();
    const predicted = await precomputeCreate2Addresses({
      ethUtils,
      version: manifest.version,
      deploymentId: manifest.deploymentId,
      deployer: manifest.deployer,
      factory: manifest.factory,
    });

    const pairs: ReadonlyArray<readonly [string, string]> = [
      [CREATE2_ROLES.implEmptyProxyAcl, predicted.emptyImplementations.acl],
      [CREATE2_ROLES.implEmptyProxy, predicted.emptyImplementations.shared],
      [CREATE2_ROLES.acl, predicted.fhevmAddresses.aclAddress],
      [CREATE2_ROLES.fhevmExecutor, predicted.fhevmAddresses.fhevmExecutorAddress],
      [CREATE2_ROLES.kmsVerifier, predicted.fhevmAddresses.kmsVerifierAddress],
      [CREATE2_ROLES.inputVerifier, predicted.fhevmAddresses.inputVerifierAddress],
      [CREATE2_ROLES.hcuLimit, predicted.fhevmAddresses.hcuLimitAddress],
      [CREATE2_ROLES.protocolConfig, predicted.fhevmAddresses.protocolConfigAddress],
      [CREATE2_ROLES.kmsGeneration, predicted.fhevmAddresses.kmsGenerationAddress],
      [CREATE2_ROLES.cleartextArithmetic, predicted.cleartextAddresses.cleartextArithmeticAddress],
      [CREATE2_ROLES.cleartextDb, predicted.cleartextAddresses.cleartextDbAddress],
      [CREATE2_ROLES.pauserSet, predicted.pauserSetAddress],
      [CREATE2_ROLES.aclOwner, predicted.aclOwnerAddress],
    ];

    const mismatches = pairs
      .map(([role, ts]) => {
        const sealed = manifest.address[role];
        if (sealed === undefined) return `${role}: absent from the manifest`;
        return ts.toLowerCase() === sealed.toLowerCase() ? null : `${role}: ts=${ts} sealed=${sealed}`;
      })
      .filter((m): m is string => m !== null);

    expect(
      mismatches,
      'the TypeScript prediction and the coordinator disagree. Both hash init code, so a mismatch means\n' +
        'the two pipelines built different bytes: check the template placeholders, the constructor-argument\n' +
        'encoding, and the initializer selectors.',
    ).toEqual([]);

    // Otherwise a role could be added to the deploy and simply escape the comparison above. The nine
    // `IMPL_` entries are excluded deliberately: their init code bakes in the whole address set, so no pure
    // function can predict them — see `precomputeCreate2Addresses`' own doc comment.
    const uncovered = Object.keys(manifest.address)
      .filter((role) => !role.startsWith('IMPL_'))
      .filter((role) => !pairs.some(([covered]) => covered === role));
    expect(uncovered, 'these roles are in the seal but not returned by precomputeCreate2Addresses').toEqual([]);

    step(t0, 'manifest matches — verifying the live chain through the predicted addresses');

    // --- the chain, not the manifest ---
    //
    // Every predicted address must hold code. Checked per role before `verify` runs, so a single wrong
    // prediction is reported as "nothing at PAUSER_SET_ADDRESS" rather than as whichever downstream read
    // happened to revert first.
    const history = createEthersHistory(anvil.rpcUrl);
    const provider = new JsonRpcProvider(anvil.rpcUrl, undefined, { staticNetwork: true });
    try {
      const empty: string[] = [];
      for (const [role, address] of pairs) {
        if ((await provider.getCode(address)) === '0x') empty.push(`${role} (${address})`);
      }
      expect(empty, 'these addresses were predicted but hold no code after a full deploy').toEqual([]);

      // The whole prediction, used as if it were the truth. Note `deployed` is built ONLY from predicted
      // values — nothing here is read back out of the manifest — so a clean report means the predicted set
      // and the deployed set are the same set.
      const deployed: Deployed = {
        fhevmAddresses: predicted.fhevmAddresses,
        cleartextAddresses: predicted.cleartextAddresses,
        pauserSetAddress: predicted.pauserSetAddress,
        aclOwnerAddress: predicted.aclOwnerAddress,
      };
      const report = await verify({
        mode: 'deploy',
        ethProvider: createEthersEthereumProvider(provider),
        history,
        deployed,
        expected: { admin: manifest.admin },
      });
      expect(
        report.failures.map((f) => `${f.name}: ${f.detail ?? ''}`),
        'a stack described entirely by predicted addresses must verify clean',
      ).toEqual([]);
      expect(
        report.skipped.map((sk) => sk.name),
        'nothing should need skipping here',
      ).toEqual([]);
    } finally {
      provider.destroy();
    }

    step(t0, 'verify clean — checking that a different deployment id is disjoint');

    // The property an operator relies on when rehearsing: a fresh deployment id cannot collide with a live
    // stack. Asserted rather than assumed, because it is a property of the SALT PREIMAGE — drop the id from
    // it and every check above still passes while `--deployment-id` silently does nothing.
    const other = await precomputeCreate2Addresses({
      ethUtils,
      version: manifest.version,
      deploymentId: `${manifest.deploymentId}-other`,
      deployer: manifest.deployer,
      factory: manifest.factory,
    });
    const otherAddresses = [
      other.emptyImplementations.acl,
      other.emptyImplementations.shared,
      other.fhevmAddresses.aclAddress,
      other.fhevmAddresses.fhevmExecutorAddress,
      other.fhevmAddresses.kmsVerifierAddress,
      other.fhevmAddresses.inputVerifierAddress,
      other.fhevmAddresses.hcuLimitAddress,
      other.fhevmAddresses.protocolConfigAddress,
      other.fhevmAddresses.kmsGenerationAddress,
      other.cleartextAddresses.cleartextArithmeticAddress,
      other.cleartextAddresses.cleartextDbAddress,
      other.pauserSetAddress,
      other.aclOwnerAddress,
    ].map((a) => a.toLowerCase());

    expect(otherAddresses.length, 'both sets must cover the same roles').toBe(pairs.length);
    const before = new Set(pairs.map(([, address]) => address.toLowerCase()));
    expect(
      otherAddresses.filter((a) => before.has(a)),
      'a different deploymentId reused an address',
    ).toEqual([]);
    // And within one set, the eight shared proxies share ONE init code — so if the role name were ever
    // dropped from the salt they would all collapse onto a single address.
    expect(new Set(otherAddresses).size, 'two roles in one set landed on the same address').toBe(otherAddresses.length);
    step(t0, 'done');
  } finally {
    await stopAnvil(anvil.process);
    // The seal goes; `build/` stays, so the next run is warm.
    clearSealKeepingBuildCache();
  }
}, 180_000);
