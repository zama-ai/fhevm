import {expect, test} from "bun:test";
import {mkdtempSync, writeFileSync, readFileSync, existsSync, rmSync} from "node:fs";
import {tmpdir} from "node:os";
import path from "node:path";
const scripts = path.resolve(import.meta.dir, "../../scripts");
const q = (s: string) => `'${s.replaceAll("'", `'\\''`)}'`;
for (const mode of ["exec", "migration"]) for (const scenario of ["timeout", "disconnect", "success"]) {
  const disconnect = scenario === "disconnect", success = scenario === "success";
  test.skipIf(process.env.GPU02_DOCKER_TESTS !== "1")(`GPU02 ${mode} quiesces before restore after ${scenario}`, () => {
    const dir = mkdtempSync(path.join(tmpdir(), "gpu02-transport-"));
    const harness = `review-${path.basename(dir).toLowerCase()}`;
    const database = `${harness}-pg`;
    const docker = Bun.spawnSync(["bash", "-c", "type -P docker"]).stdout.toString().trim();
    const call = (args: string[]) => Bun.spawnSync([docker, ...args], {timeout: 20000});
    try {
      if (mode === "migration") {
        const pg = call(["run", "-d", "--rm", "--name", database, "--network", "none", "--memory", "256m", "--cpus", "0.5", "-e", "POSTGRES_HOST_AUTH_METHOD=trust", "postgres:15.7"]);
        expect(pg.exitCode, pg.stderr.toString()).toBe(0);
        let ready = false;
        for (let attempt = 0; attempt < 100; attempt++) {
          if (call(["exec", database, "pg_isready", "-h", "127.0.0.1", "-U", "postgres"]).exitCode === 0) { ready = true; break; }
          Bun.spawnSync(["sleep", "0.1"]);
        }
        expect(ready).toBe(true);
      }
      writeFileSync(path.join(dir, "run-tests.sh"), success ? "#!/bin/bash\necho started >> /probe/evidence\necho working >> /probe/evidence\n" : "#!/bin/bash\necho started >> /probe/evidence\nwhile true; do echo working >> /probe/evidence; sleep 0.1; done\n", {mode: 0o755});
      const created = call(["run", "-d", "--rm", "--user", "0", "--name", harness, "--network", "none", "--memory", "256m", "--cpus", "0.2", "-v", `${dir}:/probe`, "-w", "/probe", "--entrypoint", "sleep", process.env.GPU02_TEST_NODE_IMAGE ?? "ghcr.io/zama-ai/fhevm/test-suite/e2e:fhevm-local", "120"]);
      expect(created.exitCode, created.stderr.toString()).toBe(0);
      const args = mode === "exec" ? ["docker", "exec", harness, "./run-tests.sh"] : ["docker", "run", "--rm", "--network", `container:${database}`, "--memory", "128m", "--cpus", "0.2", "-e", "DATABASE_URL=postgresql://postgres@127.0.0.1/postgres", "--entrypoint", "/bin/bash", "-v", `${dir}:/probe`, "-v", `${dir}/run-tests.sh:/revert_coprocessor_db_state.sh:ro`, "postgres:15.7", "/revert_coprocessor_db_state.sh"];
      writeFileSync(path.join(dir, "driver.ts"), `import {runWithHeartbeat} from ${JSON.stringify(path.resolve(scripts, "../src/utils/process"))};\ntry { await runWithHeartbeat(${JSON.stringify(args)}, 'GPU02 fixture'); } finally { await Bun.write(${JSON.stringify(path.join(dir, "cli-finally"))}, await Bun.file(${JSON.stringify(path.join(dir, "evidence"))}).text()); }\n`);
      writeFileSync(path.join(dir, "docker-proxy.py"), `#!/usr/bin/python3\nimport os,subprocess,sys,time\na=sys.argv[1:]\nif ${disconnect ? "True" : "False"} and ((a[:2]==['start','-a']) or (a and a[0]=='exec' and 'run' in a)):\n p=subprocess.Popen([${JSON.stringify(docker)}]+a)\n time.sleep(0.8)\n p.kill();p.wait();sys.exit(1)\nos.execv(${JSON.stringify(docker)},[${JSON.stringify(docker)}]+a)\n`, {mode: 0o755});
      const result = Bun.spawnSync(["bash", "-c", `set -uo pipefail
SCRIPT_DIR=${q(scripts)}; REPO_ROOT=${q(dir)}; FHEVM_STATE_DIR=${q(dir)}; SP_RUNTIME_DIR=${q(path.join(dir,"runtime"))}
source "$SCRIPT_DIR/lib/service-control.sh"; sc_init
source "$SCRIPT_DIR/lib/suite-process.sh"; sp_init
source "$SCRIPT_DIR/lib/gpu02-transport.sh"; gpu02_transport_init
GPU02_REAL_DOCKER=${q(path.join(dir, "docker-proxy.py"))}; export GPU02_REAL_DOCKER
# Sleep-only fixture: no DB/RPC journal exists.
sp_recover_suite_state() { return 0; }
case_deadline_start ${disconnect || success ? 15 : 3}
node -e "$(cat "$SCRIPT_DIR/lib/container-phase.cjs")" run gpu02_probe_$$ "$((CASE_DEADLINE_EPOCH * 1000))" env "PATH=$GPU02_SHIM_DIR:$PATH" bun ${q(path.join(dir,"driver.ts"))}
status=$?
[[ "$status" ${success ? "==" : "!="} 0 ]] || exit 2
hc_begin_cleanup
gpu02_stop_remote || exit 3
cp ${q(path.join(dir,"evidence"))} ${q(path.join(dir,"after-stop"))}
sleep 0.5
cmp ${q(path.join(dir,"evidence"))} ${q(path.join(dir,"after-stop"))} || exit 4
echo WRITERS_MAY_RESTORE
gpu02_dispose_remote || exit 5
`], {timeout: 35000});
      expect(result.exitCode, result.stderr.toString()).toBe(0);
      expect(result.stdout.toString()).toContain("WRITERS_MAY_RESTORE");
      expect(readFileSync(path.join(dir,"evidence"),"utf8")).toContain("working");
      expect(existsSync(path.join(dir,"cli-finally"))).toBe(disconnect || success);
      if (disconnect || success) expect(readFileSync(path.join(dir,"cli-finally"),"utf8")).toBe(readFileSync(path.join(dir,"evidence"),"utf8"));
    } finally {
      call(["rm", "-f", harness]);
      if (mode === "migration") call(["rm", "-f", database]);
      const names = call(["ps", "-a", "--filter", `label=fhevm.gpu02-owner=${path.join(dir,"runtime")}`, "--format", "{{.Names}}"]).stdout.toString().trim();
      for (const name of names.split("\n")) if (name) call(["rm", "-f", name]);
      rmSync(dir,{recursive:true,force:true});
    }
  }, 40000);
}

for (const ambiguous of ["create", "start"]) test(`GPU02 ambiguous migration ${ambiguous} fences and kills CLI before its finally can restore`, () => {
  const dir = mkdtempSync(path.join(tmpdir(), "gpu02-create-lost-"));
  try {
    writeFileSync(path.join(dir,"docker"), ambiguous === "create" ? '#!/bin/bash\nexit 1\n' : `#!/bin/bash
case "$1" in
 create|stop) exit 0;;
 start) exit 1;;
 inspect) if [[ "$*" == *Labels* ]]; then echo "$SP_RUNTIME_DIR"; else echo 'created false 0'; fi;;
 *) exit 1;;
esac
`, {mode:0o755});
    writeFileSync(path.join(dir,"driver.ts"), `try { const p=Bun.spawn([${JSON.stringify(path.join(scripts,"lib/gpu02-docker.cjs"))},'run','--rm','-e','DATABASE_URL=postgresql://test:dummy@localhost/test','image','/revert_coprocessor_db_state.sh'], {stdout:'inherit',stderr:'inherit'}); await p.exited; } finally { await Bun.write(${JSON.stringify(path.join(dir,"unsafe-finally"))},'unsafe'); }`);
    const result = Bun.spawnSync(["bash","-c", `set -uo pipefail
SCRIPT_DIR=${q(scripts)}; REPO_ROOT=${q(dir)}; FHEVM_STATE_DIR=${q(dir)}; SP_RUNTIME_DIR=${q(path.join(dir,"runtime"))}
source "$SCRIPT_DIR/lib/service-control.sh"; sc_init
source "$SCRIPT_DIR/lib/suite-process.sh"; sp_init
source "$SCRIPT_DIR/lib/gpu02-transport.sh"; gpu02_transport_init
GPU02_REAL_DOCKER=${q(path.join(dir,"docker"))}; export GPU02_REAL_DOCKER
case_deadline_start 10
bun ${q(path.join(dir,"driver.ts"))}; status=$?
[[ "$status" != 0 ]] || exit 2
[[ -s "$SP_CONTAMINATION" && -s "$GPU02_MIGRATION_REGISTRY" ]] || exit 3
hc_begin_cleanup
# The missing/unacknowledged container cannot be treated as clean absence.
gpu02_stop_remote && exit 4
exit 0
`], {timeout:5000});
    expect(result.exitCode,result.stderr.toString()).toBe(0);
    expect(existsSync(path.join(dir,"unsafe-finally"))).toBe(false);
  } finally {rmSync(dir,{recursive:true,force:true});}
});

test("GPU02 remote cleanup leaves GPU04 able to run its real host-supervised phase", () => {
  const dir = mkdtempSync(path.join(tmpdir(), "gpu02-next-phase-"));
  try {
    const source = readFileSync(path.join(scripts,"run-gpu-lifecycle-cases.sh"),"utf8");
    const cancel = source.slice(source.indexOf("gpu_cancel_command() {"),source.indexOf("cleanup_gpu_cases() {"));
    const phase = source.slice(source.indexOf("run_with_markers() {"),source.indexOf("record_from() {"));
    const run = Bun.spawnSync(["bash","-c",`set -uo pipefail
SCRIPT_DIR=${q(scripts)}; REPO_ROOT=${q(dir)}; FHEVM_STATE_DIR=${q(dir)}
source "$SCRIPT_DIR/lib/service-control.sh"; sc_init
source "$SCRIPT_DIR/lib/suite-process.sh"; sp_init
source "$SCRIPT_DIR/lib/gpu02-transport.sh"
GPU02_MIGRATION_REGISTRY="$SP_RUNTIME_DIR/migrations"; touch "$GPU02_MIGRATION_REGISTRY"
sp_recover_suite_state() { return 0; }
hc_begin_cleanup
gpu02_stop_remote || exit 2
[[ -f "$SP_RUNTIME_DIR/cancelling" ]] || exit 3
GPU_COMMAND_TOKEN=""; GPU_COMMAND_PID=""
${cancel}
${phase}
id=GPU-04-LOCK-LOG-VALIDITY; status=0
run_with_markers status NEXT_PHASE_RAN -- bash -c 'echo NEXT_PHASE_RAN' || exit 4
[[ "$status" == 0 ]] || exit 5
sp_dispose
`],{timeout:5000});
    expect(run.exitCode,run.stderr.toString()).toBe(0);
    expect(run.stdout.toString()).toContain("NEXT_PHASE_RAN");
  } finally {rmSync(dir,{recursive:true,force:true});}
});
