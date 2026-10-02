import { expect, test } from 'bun:test';
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';

const cli = path.resolve(import.meta.dir, '../..');
const runner = readFileSync(path.join(cli, 'scripts/run-crash-retry-consensus.sh'), 'utf8');
const cleanup = runner.slice(runner.indexOf('cleanup_crash() {'), runner.indexOf('\ntrap cleanup_crash EXIT'));

for (const failure of ['none', 'disable', 'audit']) {
  test(`crash cleanup commits release before heal and retains failed ${failure} ownership`, () => {
    const dir = mkdtempSync(path.join(tmpdir(), 'crash-controls-'));
    try {
      writeFileSync(path.join(dir, 'docker'), `#!/usr/bin/env bash
sql="\${*: -1}"
if [[ "$sql" == *'DELETE FROM'* ]]; then stage=disable; else stage=audit; fi
echo "$stage" >> "$TRACE"
[[ "$stage" != "$FAILURE" ]]
`, { mode: 0o755 });
      const script = `set -uo pipefail
source '${cli}/scripts/lib/host-command.sh'
source '${cli}/scripts/lib/crash-controls.sh'
${cleanup}
crash_record_abort() { :; }
sp_cancel_all() { echo cancel >> "$TRACE"; }
sp_recover_suite_state() { echo recover >> "$TRACE"; }
sc_run_restores() { echo heal >> "$TRACE"; }
sp_dispose() { echo dispose >> "$TRACE"; }
victim_database() { echo coprocessor_1; }
SP_RUNTIME_DIR='$dir'
SP_CONTAMINATION='$dir/contamination'
SP_PHASE_REGISTRY='$dir/phases'
SC_RESTORE_LOG='$dir/restores'
SP_FORCED_STOP=0
DB_CONTAINER=isolated-db
CRASH_CONTROLS_ARMED=1
cleanup_crash
`.replaceAll('$dir', dir);
      const result = Bun.spawnSync(['bash', '-c', script], {
        env: { ...process.env, PATH: `${dir}:${process.env.PATH}`, TRACE: path.join(dir, 'trace'), FAILURE: failure },
      });
      expect(result.exitCode, result.stderr.toString()).toBe(failure === 'none' ? 0 : 1);
      const steps = readFileSync(path.join(dir, 'trace'), 'utf8').trim().split('\n');
      expect(steps).toEqual(failure === 'disable' ? ['cancel', 'recover', 'disable']
        : failure === 'audit' ? ['cancel', 'recover', 'disable', 'heal', 'audit']
          : ['cancel', 'recover', 'disable', 'heal', 'audit', 'dispose']);
      if (failure !== 'none') {
        expect(readFileSync(path.join(dir, 'contamination'), 'utf8')).toContain('crash_database=coprocessor_1');
      }
    } finally { rmSync(dir, { recursive: true, force: true }); }
  });
}
