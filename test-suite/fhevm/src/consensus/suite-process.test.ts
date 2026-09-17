import { expect, test } from 'bun:test';
import { existsSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';

const cli = path.resolve(import.meta.dir, '../..');
const live = (pid: number) => {
  try {
    return !['Z', 'X'].includes(readFileSync(`/proc/${pid}/stat`, 'utf8').split(') ').at(-1)!.split(' ')[0]);
  } catch {
    return false;
  }
};
async function ready(file: string) {
  const end = Date.now() + 5000;
  while (!existsSync(file)) {
    if (Date.now() > end) throw new Error('fixture not ready');
    await Bun.sleep(20);
  }
}
const dockerExecPrefix = `#!/usr/bin/env bash
[[ "$1" == exec ]] || exit 1
shift
while [[ "$1" == -e || "$1" == --env ]]; do
  [[ "$#" -ge 2 ]] || exit 2
  export "$2" || exit 2
  shift 2
done
[[ "$#" -ge 2 ]] || exit 2
shift
`;
function fixture(body: string) {
  const runId = process.env.CONSENSUS_RUN_ID ?? 'suite-process-fixture';
  const dir = mkdtempSync(path.join(tmpdir(), 'suite-process-'));
  writeFileSync(path.join(dir, 'docker'), dockerExecPrefix + 'exec "$@"\n', {
    mode: 0o755,
  });
  const file = path.join(dir, 'runner.sh');
  writeFileSync(
    file,
    `#!/usr/bin/env bash
set -uo pipefail
SCRIPT_DIR='${cli}/scripts'
REPO_ROOT='${cli}/../..'
source "$SCRIPT_DIR/lib/suite-process.sh"
source "$SCRIPT_DIR/lib/case-result.sh"
export CONSENSUS_SCENARIO=none
sp_init
sp_recover_suite_state() { return 0; }
${body.replaceAll('@DIR@', dir)}
`,
  );
  return { dir, file, env: { ...process.env, PATH: `${dir}:${process.env.PATH}`, FHEVM_STATE_DIR: dir, CONSENSUS_RUN_ID: runId, CR_RUN_ID: runId, EXPECTED_RUN_ID: runId } };
}
test.skipIf(process.platform !== 'linux')(
  'shared deadline removes TERM-resistant descendants before returning',
  async () => {
    const f = fixture(`SP_PHASE_TIMEOUT_SECONDS=2
sp_exec target node -e 'if(process.env.CONSENSUS_RUN_ID!==process.env.EXPECTED_RUN_ID)process.exit(91); const fs=require("fs"),{spawn}=require("child_process"); process.on("SIGTERM",()=>{}); const child=spawn(process.execPath,["-e","setInterval(()=>{},1000)"],{stdio:"ignore"}); fs.writeFileSync("@DIR@/pids",JSON.stringify([process.pid,child.pid])); setInterval(()=>{},1000);'
status=$?
sp_cancel_all || exit 1
sp_dispose || exit 1
exit "$status"`);
    const child = Bun.spawn(['bash', f.file], { env: f.env, stdout: 'pipe', stderr: 'pipe' });
    try {
      await ready(path.join(f.dir, 'pids'));
      const pids: number[] = JSON.parse(readFileSync(path.join(f.dir, 'pids'), 'utf8'));
      expect(await child.exited, await new Response(child.stderr).text()).toBe(124);
      expect(pids.some(live)).toBe(false);
    } finally {
      child.kill();
      rmSync(f.dir, { recursive: true, force: true });
    }
  },
  10000,
);
test.skipIf(process.platform !== 'linux')(
  'TERM interrupts captured output and joins actual work before healing',
  async () => {
    const f = fixture(`cleanup(){
 local status=$?
 trap - EXIT INT TERM
 sp_cancel_all || exit 1
 node -e 'const fs=require("fs");const pid=Number(fs.readFileSync("@DIR@/pid","utf8"));try{process.kill(pid,0);process.exit(1);}catch{fs.writeFileSync("@DIR@/healed","yes");}' || exit 1
 sp_dispose
 exit "$status"
}
trap cleanup EXIT
trap 'exit 143' TERM
out=""
cr_run_suite out '' sp_exec target node -e 'require("fs").writeFileSync("@DIR@/pid",String(process.pid));setInterval(()=>{},1000);'`);
    const child = Bun.spawn(['bash', f.file], { env: f.env, stdout: 'pipe', stderr: 'pipe' });
    try {
      await ready(path.join(f.dir, 'pid'));
      child.kill('SIGTERM');
      expect(await child.exited, await new Response(child.stderr).text()).toBe(143);
      expect(existsSync(path.join(f.dir, 'healed'))).toBe(true);
      expect(live(Number(readFileSync(path.join(f.dir, 'pid'), 'utf8')))).toBe(false);
    } finally {
      child.kill();
      rmSync(f.dir, { recursive: true, force: true });
    }
  },
  10000,
);
test('child init and disposal preserve caller registry and inherited deadline', () => {
  const f = fixture(`sp_cancel_all; sp_dispose
SP_RUNTIME_DIR='@DIR@/parent-runtime'
SP_PHASE_REGISTRY='@DIR@/parent-registry'
mkdir -p "$SP_RUNTIME_DIR"
echo 'prior record' > "$SP_PHASE_REGISTRY"
CASE_DEADLINE_EPOCH=$(( $(date +%s)+2 ))
sp_init
sp_case_start MAT-01-BOUNDARY-FANOUT || exit 1
[[ "$(case_seconds_left)" -le 2 ]] || exit 1
[[ "$(cat "$SP_PHASE_REGISTRY")" == 'prior record' ]] || exit 1
touch "$SP_RUNTIME_DIR/cancelling"
sp_dispose
[[ -f "$SP_PHASE_REGISTRY" && -d "$SP_RUNTIME_DIR" ]]`);
  try {
    const r = Bun.spawnSync(['bash', f.file], { env: f.env });
    expect(r.exitCode, r.stderr.toString()).toBe(0);
  } finally {
    rmSync(f.dir, { recursive: true, force: true });
  }
});
test('unverified fallback stop retains recovery records and closes phase admission', () => {
  const f = fixture(`printf 'target|known_token\\n' >> "$SP_PHASE_REGISTRY"
sp_cancel_all; result=$?
[[ "$result" == 1 && -f "$SP_CONTAMINATION" && -f "$SP_PHASE_REGISTRY" ]] || exit 1
sp_exec target node -e 'throw new Error("must not run")'; status=$?
[[ "$status" == 143 ]]`);
  writeFileSync(
    path.join(f.dir, 'docker'),
    '#!/usr/bin/env bash\ncase "$1" in stop) exit 0;; inspect) echo "true 123";; *) exit 1;; esac\n',
    { mode: 0o755 },
  );
  try {
    const r = Bun.spawnSync(['bash', f.file], { env: f.env });
    expect(r.exitCode, r.stderr.toString()).toBe(0);
  } finally {
    rmSync(f.dir, { recursive: true, force: true });
  }
});

test('verified fallback restart preserves the harness filesystem and recovers state before disposal', () => {
  const f = fixture(`printf 'target|known_token\\n' >> "$SP_PHASE_REGISTRY"
sp_cancel_all || exit 1
[[ "$SP_FORCED_STOP" == 1 && -f "$SP_RUNTIME_DIR/known_token.verified" ]] || exit 1
# Reload the actual recovery function; the fixture's process-only tests stub it.
source /dev/stdin <<'FUNCTION'
@RECOVERY@
FUNCTION
sp_recover_suite_state || exit 1
sp_dispose || exit 1`);
  const helper = readFileSync(path.join(cli, 'scripts/lib/suite-process.sh'), 'utf8');
  const start = helper.indexOf('sp_recover_suite_state() {');
  const end = helper.indexOf('\n# Only after cancellation', start);
  writeFileSync(f.file, readFileSync(f.file, 'utf8').replace('@RECOVERY@', helper.slice(start, end)));
  writeFileSync(
    path.join(f.dir, 'docker'),
    `#!/usr/bin/env bash
case "$1" in
 stop) echo stop >> '${f.dir}/trace'; exit 0;;
 inspect) echo 'false 0'; exit 0;;
 start) echo start >> '${f.dir}/trace'; exit 0;;
 exec) if [[ "$*" == *'recoverAbortedSuite()'* ]]; then echo recover >> '${f.dir}/trace'; exit 0; fi; exit 1;;
 *) exit 1;;
esac
`,
    { mode: 0o755 },
  );
  try {
    const r = Bun.spawnSync(['bash', f.file], { env: f.env });
    expect(r.exitCode, r.stderr.toString()).toBe(0);
    expect(readFileSync(path.join(f.dir, 'trace'), 'utf8').trim().split('\n')).toEqual(['stop', 'start', 'recover']);
  } finally {
    rmSync(f.dir, { recursive: true, force: true });
  }
});

test('delegated runners inherit the generated parent run ID', () => {
  const f = fixture(`unset CONSENSUS_RUN_ID
cr_init >/dev/null
before="$CR_RUN_ID"
child="$(bash -c 'source "$SCRIPT_DIR/lib/case-result.sh"; cr_init >/dev/null; printf "%s" "$CR_RUN_ID"')"
[[ "$child" == "$before" ]]`);
  f.env = { ...f.env, SCRIPT_DIR: `${cli}/scripts`, REPO_ROOT: path.resolve(cli, '../..') } as typeof f.env;
  try {
    const r = Bun.spawnSync(['bash', f.file], { env: f.env });
    expect(r.exitCode, r.stderr.toString()).toBe(0);
  } finally {
    rmSync(f.dir, { recursive: true, force: true });
  }
});

test.skipIf(process.platform !== 'linux')(
  'a failed phase cancels other active phases before shared journal recovery',
  async () => {
    const f = fixture(`
sp_recover_suite_state() {
 node -e 'const fs=require("fs");const pid=Number(fs.readFileSync("@DIR@/other.pid","utf8"));try{process.kill(pid,0);process.exit(1);}catch{fs.writeFileSync("@DIR@/recovered","yes");}'
}
sp_exec target node -e 'require("fs").writeFileSync("@DIR@/other.pid",String(process.pid));setInterval(()=>{},1000)' &
other=$!
while [[ ! -f '@DIR@/other.pid' ]]; do sleep 0.01; done
sp_exec target node -e 'process.exit(42)'; status=$?
wait "$other" 2>/dev/null || true
[[ "$status" == 42 && -f '@DIR@/recovered' && -f "$SP_RUNTIME_DIR/cancelling" ]] || exit 1
sp_dispose
`);
    const child = Bun.spawn(['bash', f.file], { env: f.env, stdout: 'pipe', stderr: 'pipe' });
    try {
      expect(await child.exited, await new Response(child.stderr).text()).toBe(0);
      expect(live(Number(readFileSync(path.join(f.dir, 'other.pid'), 'utf8')))).toBe(false);
    } finally {
      child.kill();
      rmSync(f.dir, { recursive: true, force: true });
    }
  },
  10000,
);

test("missing or unreadable initialized phase ledger blocks restoration admission", () => {
  const directory = mkdtempSync(path.join(tmpdir(), "phase-registry-io-"));
  try {
    for (const registry of [path.join(directory, "missing"), "/proc/self/mem"]) {
      const run = Bun.spawnSync(["bash", "-c", `set -uo pipefail
SCRIPT_DIR='${cli}/scripts'; REPO_ROOT='${directory}'; FHEVM_STATE_DIR='${directory}'
source "$SCRIPT_DIR/lib/suite-process.sh"
sp_init
SP_PHASE_REGISTRY='$REGISTRY'
sp_cancel_all; status=$?
[[ "$status" != 0 && -f "$SP_CONTAMINATION" ]]
`.replace("$REGISTRY", registry)], {timeout: 5000});
      expect(run.exitCode, run.stderr.toString()).toBe(0);
    }
  } finally { rmSync(directory, {recursive: true, force: true}); }
});

test('cancellation removes a hung transport even though the hc_run Bash PID has no token argument', async () => {
  const f = fixture(`
sp_exec target node -e 'process.exit(0)' &
CR_SUITE_PID=$!
while [[ ! -f '@DIR@/transport.pid' ]]; do sleep .02; done
sp_cancel_all || exit 1
sp_dispose || exit 1
`);
  writeFileSync(path.join(f.dir, 'docker'), `${dockerExecPrefix}
if [[ "$4" == run ]]; then
  "$@" || exit "$?"
  echo "$BASHPID" > '${f.dir}/transport.pid'
  trap '' TERM
  sleep 30
else exec "$@"; fi
`, {mode: 0o755});
  const child = Bun.spawn(['bash', f.file], {env:f.env,stdout:'pipe',stderr:'pipe'});
  try {
    await ready(path.join(f.dir,'transport.pid'));
    const pid=Number(readFileSync(path.join(f.dir,'transport.pid'),'utf8'));
    expect(await child.exited, await new Response(child.stderr).text()).toBe(0);
    expect(live(pid)).toBe(false);
  } finally {child.kill();rmSync(f.dir,{recursive:true,force:true});}
}, 10000);
