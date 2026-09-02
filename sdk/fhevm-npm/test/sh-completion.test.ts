import assert from 'node:assert/strict';
import test from 'node:test';

import { renderCompletionScript } from '../base/sh-completion.ts';
import { commandNames, parseCliOptions } from '../cli-options.ts';

function completionCommands(): readonly Parameters<typeof renderCompletionScript>[1][number][] {
  const options = parseCliOptions(['sh-completion', 'zsh']);
  assert.equal(options.command, 'sh-completion');
  return options.command === 'sh-completion' ? options.commands : [];
}

test('the completion table is introspected from the live registry and covers every command', () => {
  const names = completionCommands().map((cmd) => cmd.name);
  for (const name of [
    ...commandNames,
    'generate-cleartext-config',
    'sync-vendored',
    'test-consumer',
    'sh-completion',
  ]) {
    assert.ok(names.includes(name), `completion table is missing '${name}'`);
  }
});

test('the zsh script names every command, escapes descriptions, and offers selector values', () => {
  const script = renderCompletionScript('zsh', completionCommands(), ['./host-contracts-cleartext/v12']);
  for (const name of commandNames) assert.ok(script.includes(`'${name}:`), `zsh script is missing '${name}'`);
  // 'package.json hygiene' survives the sentence-splitter; a raw description colon would break _describe.
  assert.match(script, /'check-package-json:Check package\.json hygiene'/);
  assert.doesNotMatch(script, /truth: the shared/);
  assert.match(script, /generate-exports\) _opts\+=\( --check \)/);
  assert.match(script, /_pkgs=\( \.\/host-contracts-cleartext\/v12 \)/);
  assert.match(script, /compdef _fhevm_npm_cli fhevm-npm-cli fhevm-npm/);
});

test('the bash script registers the completer and carries per-command flags', () => {
  const script = renderCompletionScript('bash', completionCommands(), ['./common']);
  assert.match(script, /complete -F _fhevm_npm_cli fhevm-npm-cli fhevm-npm/);
  assert.match(script, /test-consumer\) opts\+=" -l --list/);
  assert.match(script, /\.\/common/);
});

test('an unsupported shell is refused', () => {
  assert.throws(() => parseCliOptions(['sh-completion', 'fish']), /unsupported shell 'fish'/);
});
