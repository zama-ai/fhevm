import assert from 'node:assert/strict';
import { mkdirSync, mkdtempSync, readFileSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { describe, it } from 'node:test';

import { type CommandRunner, CompileError, GENERATOR_SCRIPT, compileContracts } from '../../../src/boot/compile.js';
import { loadProfile } from '../../../src/config/profiles/profile.js';
import { createRedactor } from '../../../src/infrastructure/secrets/redactor.js';
import { BUILD_STAMP_FILE } from '../../../src/runtime/modules/contracts/build-stamp.js';
import { FHE_PROFILES } from '../../helpers.js';

const profile = loadProfile('fx-fhe', {
  profilesDir: FHE_PROFILES,
  env: { TE_FHE_ACL: '0x00000000000000000000000000000000000000a1' },
});
const redactor = createRedactor([]);
const NOW = () => new Date('2026-11-11T10:00:00.000Z');

/** An e2e-like project: the generator and the Hardhat config exist (the fake runner never runs them). */
function project(): string {
  const dir = mkdtempSync(path.join(tmpdir(), 'te-e2e-'));
  mkdirSync(path.join(dir, 'scripts'));
  writeFileSync(path.join(dir, GENERATOR_SCRIPT), '');
  writeFileSync(path.join(dir, 'hardhat.config.ts'), '');
  return dir;
}

interface Call {
  command: string;
  args: string[];
  cwd: string;
  env: NodeJS.ProcessEnv;
}

/**
 * Records the calls; `hardhat compile` writes a compiled E2ECoprocessorConfig whose bytecode embeds
 * `embedded` (default: the addresses the generator was given).
 */
function fakeRunner(options: { exitCodes?: number[]; embedded?: string[] } = {}): {
  run: CommandRunner;
  calls: Call[];
} {
  const calls: Call[] = [];
  const run: CommandRunner = async (command, args, { cwd, env }) => {
    calls.push({ command, args, cwd, env });
    const code = options.exitCodes?.[calls.length - 1] ?? 0;
    if (code === 0 && args[0] === 'hardhat') {
      const embedded = options.embedded ?? [
        env.ACL_CONTRACT_ADDRESS!,
        env.FHEVM_EXECUTOR_CONTRACT_ADDRESS!,
        env.KMS_VERIFIER_CONTRACT_ADDRESS!,
      ];
      const dir = path.join(cwd, 'artifacts', 'contracts', 'E2ECoprocessorConfigLocal.sol');
      mkdirSync(dir, { recursive: true });
      const bytecode = `0x6080${embedded.map((address) => `73${address.slice(2).toLowerCase()}`).join('')}00`;
      writeFileSync(
        path.join(dir, 'E2ECoprocessorConfig.json'),
        JSON.stringify({ contractName: 'E2ECoprocessorConfig', abi: [], bytecode }),
      );
    }
    return code;
  };
  return { run, calls };
}

describe('compile (test-engine compile)', () => {
  it('runs the e2e generator with the profile addresses, compiles, verifies and stamps', async () => {
    const dir = project();
    const { run, calls } = fakeRunner();
    const result = await compileContracts(profile, { project: dir, redactor, run, env: { PATH: '/bin' }, now: NOW });

    assert.deepEqual(
      calls.map((call) => [call.command, ...call.args]),
      [
        ['npx', 'ts-node', '--transpile-only', GENERATOR_SCRIPT],
        ['npx', 'hardhat', 'compile'],
      ],
    );
    for (const call of calls) {
      assert.equal(call.cwd, dir);
      assert.equal(call.env.PATH, '/bin', 'the base environment is kept');
      assert.equal(call.env.E2E_COPROCESSOR_CONFIG_FROM_ENV, 'true');
      assert.equal(call.env.ACL_CONTRACT_ADDRESS, '0x00000000000000000000000000000000000000a1');
      assert.equal(call.env.FHEVM_EXECUTOR_CONTRACT_ADDRESS, '0x0000000000000000000000000000000000000007');
      assert.equal(call.env.KMS_VERIFIER_CONTRACT_ADDRESS, '0x0000000000000000000000000000000000000005');
    }
    assert.equal(result.artifactsDir, path.join(dir, 'artifacts'));
    assert.equal(result.stampFile, path.join(dir, 'artifacts', BUILD_STAMP_FILE));
    assert.deepEqual(JSON.parse(readFileSync(result.stampFile, 'utf8')), {
      profile: 'fx-fhe',
      addresses: {
        acl: '0x00000000000000000000000000000000000000a1',
        executor: '0x0000000000000000000000000000000000000007',
        kmsVerifier: '0x0000000000000000000000000000000000000005',
      },
      compiledAt: '2026-11-11T10:00:00.000Z',
    });
  });

  it('refuses a profile without contracts section and a folder that is not an e2e project', async () => {
    const { run, calls } = fakeRunner();
    const none = loadProfile('none', { profilesDir: FHE_PROFILES });
    await assert.rejects(
      compileContracts(none, { project: project(), redactor, run }),
      (error: unknown) =>
        error instanceof CompileError && /profile 'none' has no contracts section/.test(error.message),
    );
    await assert.rejects(
      compileContracts(profile, { project: mkdtempSync(path.join(tmpdir(), 'te-empty-')), redactor, run }),
      /is not an e2e project with the coprocessor config generator \(missing scripts\/generate-coprocessor-config\.ts, hardhat\.config\.ts\)/,
    );
    assert.equal(calls.length, 0, 'nothing ran');
  });

  it('stops on a failed generation or compilation, and on a runner that cannot start', async () => {
    const generation = fakeRunner({ exitCodes: [1] });
    await assert.rejects(
      compileContracts(profile, { project: project(), redactor, run: generation.run }),
      /generating the coprocessor config failed \(exit code 1\)/,
    );
    assert.equal(generation.calls.length, 1, 'no compilation after a failed generation');
    await assert.rejects(
      compileContracts(profile, { project: project(), redactor, run: fakeRunner({ exitCodes: [0, 2] }).run }),
      /compiling the contracts failed \(exit code 2\)/,
    );
    await assert.rejects(
      compileContracts(profile, {
        project: project(),
        redactor,
        run: async () => {
          throw new Error('spawn npx ENOENT');
        },
      }),
      /generating the coprocessor config: cannot start 'npx': spawn npx ENOENT/,
    );
  });

  it('fails when the compiled config does not embed the profile addresses, and writes no stamp', async () => {
    const dir = project();
    const stale = fakeRunner({ embedded: ['0x00000000000000000000000000000000000000a1'] });
    await assert.rejects(
      compileContracts(profile, { project: dir, redactor, run: stale.run }),
      /the compiled E2ECoprocessorConfig does not embed the profile's contracts\.executor, contracts\.kmsVerifier/,
    );
    assert.throws(() => readFileSync(path.join(dir, 'artifacts', BUILD_STAMP_FILE)), /ENOENT/);
    await assert.rejects(
      compileContracts(profile, { project: project(), redactor, run: async () => 0 }),
      /cannot verify the compiled coprocessor config: cannot read the contracts directory .*artifacts/,
    );
  });
});
