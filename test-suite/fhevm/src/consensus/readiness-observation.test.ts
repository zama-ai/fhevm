import { expect, test } from 'bun:test';
import { mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';

test('every readiness entrypoint rejects restart budget consumed before its first poll', () => {
  const dir = mkdtempSync(path.join(tmpdir(), 'readiness-first-poll-'));
  try {
    writeFileSync(path.join(dir, 'docker'), `#!/bin/sh
if [ "$1" = inspect ]; then
 echo '[{"RestartCount":10,"State":{"Status":"running","ExitCode":0,"Health":{"Status":"healthy"}}}]'
else echo 'worker ready'; fi
`, { mode: 0o755 });
    const module = path.resolve(import.meta.dir, '../flow/readiness.ts');
    for (const expression of ['waitForContainer("worker","healthy")', 'waitForContainer("worker","running")',
      'waitForLog("worker",/worker ready/)', 'postBootHealthGate(["worker"],0)']) {
      const result = Bun.spawnSync([process.execPath, '-e', `import {waitForContainer,waitForLog,postBootHealthGate} from '${module}';
        try { await ${expression}; process.exitCode=9; } catch(e) { if(e.constructor.name !== 'ContainerCrashed') throw e; }`],
      { env: { ...process.env, PATH: `${dir}:${process.env.PATH}` } });
      expect(result.exitCode, result.stderr.toString()).toBe(0);
    }
  } finally { rmSync(dir, { recursive: true, force: true }); }
});

test('key ingestion wait accepts CPU legacy material but requires compressed material for GPU callers', () => {
  const dir = mkdtempSync(path.join(tmpdir(), 'readiness-key-capability-'));
  try {
    writeFileSync(path.join(dir, 'docker'), '#!/bin/sh\necho "$KEY_ROWS"\n', { mode: 0o755 });
    const module = path.resolve(import.meta.dir, '../flow/readiness.ts');
    for (const [compressed, rows, expected] of [[false, '1|0|1', 0], [true, '1|0|1', 1], [true, '1|1|0', 0]] as const) {
      const result = Bun.spawnSync([process.execPath, '-e', `import {waitForCoprocessorKeyMaterial} from '${module}';
        Bun.sleep=async()=>{};
        await waitForCoprocessorKeyMaterial({versions:{env:{COPROCESSOR_DB_MIGRATION_VERSION:'v0.14.0'}},
          scenario:{kind:'coprocessor-consensus',instances:[{index:0,env:{}}],topology:{count:1,threshold:1}}},1,{requireCompressed:${compressed}});`],
      { env: { ...process.env, PATH: `${dir}:${process.env.PATH}`, KEY_ROWS: rows } });
      expect(result.exitCode, result.stderr.toString()).toBe(expected);
    }
  } finally { rmSync(dir, { recursive: true, force: true }); }
});
