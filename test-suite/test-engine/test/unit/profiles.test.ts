import assert from 'node:assert/strict';
import path from 'node:path';
import { describe, it } from 'node:test';

import { DEFAULT_PROFILES_DIR } from '../../src/paths.js';
import { parseEnvFile } from '../../src/profiles/env-file.js';
import {
  listProfileNames,
  listProfiles,
  loadProfile,
  readProfileFile,
  redactedChain,
  redactedSettings,
  secretsOf,
} from '../../src/profiles/profile.js';
import { ProfileError } from '../../src/profiles/types.js';
import {
  ANVIL_MNEMONIC,
  CHAIN_PROFILES,
  FIXTURE_PROFILES,
  FIXTURE_SECRET,
  INVALID_PROFILES,
  RESERVED_PROFILES,
} from '../helpers.js';

function problemsOf(action: () => unknown): string {
  try {
    action();
  } catch (error) {
    assert.ok(error instanceof ProfileError, String(error));
    return error.problems.join('; ');
  }
  assert.fail('expected a ProfileError');
}

const SECRET_ENV = { TE_FIXTURE_SECRET: FIXTURE_SECRET };

describe('profile files', () => {
  it('ships valid profiles: local, devnet, testnet, mainnet (mainnet requires confirmation)', () => {
    const { profiles, errors } = listProfiles(DEFAULT_PROFILES_DIR);
    assert.deepEqual(errors, []);
    assert.deepEqual(
      profiles.map((profile) => profile.metadata.name),
      ['devnet', 'local', 'mainnet', 'testnet'],
    );
    const mainnet = profiles.find((profile) => profile.metadata.name === 'mainnet')!;
    assert.equal(mainnet.spec.requiresConfirmation, true);
    assert.deepEqual(listProfileNames(DEFAULT_PROFILES_DIR), ['none', 'devnet', 'local', 'mainnet', 'testnet']);
  });

  it('reports every invalid profile with a clear problem', () => {
    const { errors } = listProfiles(INVALID_PROFILES);
    const byFile = new Map(errors.map((error) => [path.basename(error.source, '.yaml'), error.problems.join('; ')]));
    assert.match(byFile.get('unknown-field')!, /spec: unknown field 'rpc'/);
    assert.match(byFile.get('name-mismatch')!, /must match the file name 'name-mismatch'/);
    assert.match(byFile.get('multi-problem')!, /spec: unknown field 'rpc'/);
    assert.match(byFile.get('multi-problem')!, /must match the file name 'multi-problem'/);
    assert.match(byFile.get('multi-problem')!, /a secret cannot have a literal 'value'/);
    assert.match(byFile.get('secret-value')!, /a secret cannot have a literal 'value'/);
    assert.match(byFile.get('secret-default')!, /a secret cannot have a 'default'/);
    assert.match(byFile.get('value-and-env')!, /settings\.url: exactly one of 'value' or 'env'/);
    assert.match(byFile.get('value-and-env')!, /settings\.empty: exactly one of 'value' or 'env'/);
    assert.match(byFile.get('chain-invalid')!, /spec\.chain: missing required field 'chainId'/);
    assert.match(byFile.get('chain-invalid')!, /spec\.chain\.mnemonic: must declare 'secret: true'/);
    assert.match(byFile.get('chain-invalid')!, /spec\.chain\.hdPath: must match pattern/);
    assert.match(byFile.get('chain-invalid')!, /spec\.chain\.gas: unknown field 'multiplier'/);
    assert.match(byFile.get('chain-invalid')!, /spec\.chain\.wallets: 'alice' and 'bob' use the same index 0/);
    // A valid file whose resolution would fail is still a valid definition.
    assert.equal(byFile.has('missing-env-file'), false);
    assert.match(byFile.get('chain-secret-literal')!, /spec\.chain\.mnemonic: a secret cannot have a literal 'value'/);
    assert.equal(byFile.has('chain-types'), false);
  });

  it("treats a 'none.yaml' file as a configuration error everywhere", () => {
    assert.throws(() => listProfileNames(RESERVED_PROFILES), /'none' is reserved for the built-in profile/);
    assert.match(listProfiles(RESERVED_PROFILES).errors[0]!.problems.join(), /'none' is reserved/);
  });

  it('applies defaults', () => {
    const basic = readProfileFile(path.join(FIXTURE_PROFILES, 'fx-basic.yaml'));
    assert.equal(basic.spec.requiresConfirmation, false);
    assert.deepEqual(basic.spec.envFiles, []);
    assert.deepEqual(basic.spec.settings, {});
  });
});

describe('loadProfile', () => {
  it('returns the built-in none profile', () => {
    const none = loadProfile('none', { profilesDir: FIXTURE_PROFILES });
    assert.equal(none.requiresConfirmation, false);
    assert.deepEqual(none.settings, {});
  });

  it('resolves every kind of value source and records where each value came from', () => {
    const profile = loadProfile('fx-secret', { profilesDir: FIXTURE_PROFILES, env: SECRET_ENV });
    const { settings } = profile;
    assert.deepEqual(settings['fixture.literal'], { value: 'hello-literal', secret: false, source: 'literal' });
    assert.deepEqual(settings['fixture.secret'], {
      value: FIXTURE_SECRET,
      secret: true,
      source: 'env:TE_FIXTURE_SECRET',
    });
    assert.equal(settings['fixture.from-file']!.value, 'value-from-file');
    assert.match(settings['fixture.from-file']!.source, /^env-file:.*fx-secret\.env:TE_FIXTURE_FROM_FILE$/);
    assert.deepEqual(settings['fixture.with-default'], { value: 'fallback-value', secret: false, source: 'default' });
  });

  it('gives the process environment precedence over env files', () => {
    const profile = loadProfile('fx-secret', {
      profilesDir: FIXTURE_PROFILES,
      env: { ...SECRET_ENV, TE_FIXTURE_FROM_FILE: 'from-env' },
    });
    assert.deepEqual(profile.settings['fixture.from-file'], {
      value: 'from-env',
      secret: false,
      source: 'env:TE_FIXTURE_FROM_FILE',
    });
  });

  it('never merges env files into process.env', () => {
    loadProfile('fx-secret', { profilesDir: FIXTURE_PROFILES, env: SECRET_ENV });
    assert.equal(process.env.TE_FIXTURE_FROM_FILE, undefined);
    assert.equal(process.env.TE_FIXTURE_OTHER, undefined);
  });

  it('fails on a missing environment variable', () => {
    assert.match(
      problemsOf(() => loadProfile('fx-secret', { profilesDir: FIXTURE_PROFILES, env: {} })),
      /fixture\.secret: environment variable 'TE_FIXTURE_SECRET' is not set/,
    );
  });

  it('fails on a secret shorter than 8 characters', () => {
    assert.match(
      problemsOf(() =>
        loadProfile('fx-secret', { profilesDir: FIXTURE_PROFILES, env: { TE_FIXTURE_SECRET: 'short' } }),
      ),
      /secret value must be at least 8 characters/,
    );
  });

  it('fails on a missing non-optional env file (and lists every problem)', () => {
    const problems = problemsOf(() => loadProfile('missing-env-file', { profilesDir: INVALID_PROFILES, env: {} }));
    assert.match(problems, /spec\.envFiles: file not found: \.\/not-there\.env/);
    assert.match(problems, /SOME_VARIABLE_THAT_IS_NOT_SET' is not set/);
  });

  it('fails on an unknown profile, listing the available ones', () => {
    assert.match(
      problemsOf(() => loadProfile('staging', { profilesDir: FIXTURE_PROFILES })),
      /unknown profile 'staging'\. Available: none, fx-basic, fx-confirm, fx-secret/,
    );
  });

  it('exposes secrets for redaction and redacts them in printable settings', () => {
    const profile = loadProfile('fx-secret', { profilesDir: FIXTURE_PROFILES, env: SECRET_ENV });
    assert.deepEqual(secretsOf(profile), [{ key: 'fixture.secret', value: FIXTURE_SECRET }]);
    const printable = redactedSettings(profile);
    assert.equal(printable['fixture.secret']!.value, '[REDACTED:fixture.secret]');
    assert.equal(printable['fixture.literal']!.value, 'hello-literal');
    assert.ok(!JSON.stringify(printable).includes(FIXTURE_SECRET));
  });

  it('fails when the profiles directory does not exist', () => {
    assert.throws(() => listProfileNames(path.join(FIXTURE_PROFILES, 'nope')), /Profiles directory not found/);
  });
});

describe('chain section', () => {
  const ANVIL_ENV = {};

  it('is declared by every shipped profile, and resolves with the mnemonic as a secret', () => {
    const { profiles } = listProfiles(DEFAULT_PROFILES_DIR);
    for (const profile of profiles) assert.ok(profile.spec.chain, `${profile.metadata.name} has a chain section`);
    const env = { SEPOLIA_ETH_RPC_URL: 'https://sepolia.example/key-123456', MNEMONIC: ANVIL_MNEMONIC };
    const testnet = loadProfile('testnet', { profilesDir: DEFAULT_PROFILES_DIR, env });
    assert.equal(testnet.chain!.chainId.value, 11155111);
    assert.deepEqual(
      secretsOf(testnet).map((secret) => secret.key),
      [
        'chain.rpcUrl',
        'chain.mnemonic',
        'chain.wallets.alice',
        'chain.wallets.bob',
        'chain.wallets.carol',
        'chain.wallets.dave',
        'chain.wallets.eve',
      ],
    );
  });

  it('resolves values, applies the e2e defaults and records the sources', () => {
    const profile = loadProfile('fx-anvil', { profilesDir: CHAIN_PROFILES, env: ANVIL_ENV });
    const chain = profile.chain!;
    assert.deepEqual(chain.rpcUrl, { value: 'http://127.0.0.1:8545', secret: false, source: 'default' });
    assert.deepEqual(chain.chainId, { value: 31337, secret: false, source: 'literal' });
    assert.equal(chain.mnemonic.value, ANVIL_MNEMONIC);
    assert.match(chain.mnemonic.source, /^env-file:.*fx-anvil\.env:TE_ANVIL_MNEMONIC$/);
    assert.equal(chain.hdPath, "m/44'/60'/0'/0");
    assert.deepEqual(chain.wallets, { alice: 0, bob: 1, carol: 2, dave: 3, eve: 4 });
    assert.deepEqual(chain.gas, { bufferPercent: 50, limitCap: 10_000_000 });
  });

  it('converts a chain ID read from the environment to a number', () => {
    const profile = loadProfile('fx-anvil', {
      profilesDir: CHAIN_PROFILES,
      env: { TE_ANVIL_RPC_URL: 'http://anvil:8545' },
    });
    assert.equal(profile.chain!.rpcUrl.source, 'env:TE_ANVIL_RPC_URL');
    const typed = loadProfile('chain-types', {
      profilesDir: INVALID_PROFILES,
      env: { TE_CHAIN_RPC_URL: 'http://x:1', TE_CHAIN_ID: '12345', TE_CHAIN_MNEMONIC: ANVIL_MNEMONIC },
    });
    assert.equal(typed.chain!.chainId.value, 12345);
  });

  it('checks the types of resolved values, reporting every problem without quoting the mnemonic', () => {
    const problems = problemsOf(() =>
      loadProfile('chain-types', {
        profilesDir: INVALID_PROFILES,
        env: { TE_CHAIN_RPC_URL: 'ftp://x', TE_CHAIN_ID: '12a', TE_CHAIN_MNEMONIC: 'not a valid mnemonic phrase' },
      }),
    );
    assert.match(problems, /spec\.chain\.rpcUrl: must be an http\(s\) URL/);
    assert.match(problems, /spec\.chain\.chainId: must be a positive integer, got '12a'/);
    assert.match(problems, /spec\.chain\.mnemonic: not a valid BIP-39 mnemonic/);
    assert.ok(!problems.includes('not a valid mnemonic phrase'));
    assert.match(
      problemsOf(() => loadProfile('chain-types', { profilesDir: INVALID_PROFILES, env: {} })),
      /spec\.chain\.rpcUrl: environment variable 'TE_CHAIN_RPC_URL' is not set.*spec\.chain\.mnemonic: environment variable/,
    );
  });

  it('gives the process environment precedence over env files for chain values', () => {
    const other = 'legal winner thank year wave sausage worth useful legal winner thank yellow';
    const profile = loadProfile('fx-anvil', { profilesDir: CHAIN_PROFILES, env: { TE_ANVIL_MNEMONIC: other } });
    assert.deepEqual(profile.chain!.mnemonic, { value: other, secret: true, source: 'env:TE_ANVIL_MNEMONIC' });
  });

  it('exposes the mnemonic and the derived private keys for redaction, and redacts the printable section', () => {
    const profile = loadProfile('fx-anvil', { profilesDir: CHAIN_PROFILES, env: ANVIL_ENV });
    const secrets = secretsOf(profile);
    assert.deepEqual(secrets[0], { key: 'chain.mnemonic', value: ANVIL_MNEMONIC });
    // Account 0 of the Anvil mnemonic, without the 0x prefix.
    assert.deepEqual(secrets[1], {
      key: 'chain.wallets.alice',
      value: 'ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80',
    });
    assert.equal(secrets.length, 6);
    const printable = redactedChain(profile)!;
    assert.equal(printable.mnemonic.value, '[REDACTED:chain.mnemonic]');
    assert.equal(printable.rpcUrl.value, 'http://127.0.0.1:8545');
    assert.ok(!JSON.stringify(printable).includes(ANVIL_MNEMONIC));
    assert.equal(profile.chain!.mnemonic.value, ANVIL_MNEMONIC, 'the resolved profile is not modified');
  });

  it('is absent from the built-in none profile', () => {
    assert.equal(loadProfile('none', { profilesDir: CHAIN_PROFILES }).chain, undefined);
  });
});

describe('parseEnvFile', () => {
  it('parses comments, export, quotes, escapes and trailing comments', () => {
    const values = parseEnvFile(
      [
        '# comment',
        '',
        'export A=1',
        "B='single # not a comment'",
        'C="double \\"quoted\\"\\nline"',
        'D=plain value # trailing comment',
        'E=',
      ].join('\n'),
    );
    assert.deepEqual(Object.fromEntries(values), {
      A: '1',
      B: 'single # not a comment',
      C: 'double "quoted"\nline',
      D: 'plain value',
      E: '',
    });
  });

  it('rejects malformed lines', () => {
    assert.throws(() => parseEnvFile('NOT A LINE', 'x.env'), /x\.env:1: expected KEY=VALUE/);
    assert.throws(() => parseEnvFile('A="unterminated', 'x.env'), /unterminated double-quoted/);
  });
});
