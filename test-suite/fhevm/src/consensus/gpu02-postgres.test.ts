import {expect, test} from "bun:test";
import {mkdtempSync, mkdirSync, writeFileSync, readFileSync, existsSync, rmSync} from "node:fs";
import {tmpdir} from "node:os";
import path from "node:path";
const scripts = path.resolve(import.meta.dir, "../../scripts");
const q = (value: string) => `'${value.replaceAll("'", `'\\''`)}'`;

for (const queryFails of [false, true]) test.skipIf(process.env.GPU02_DOCKER_TESTS !== "1")(`GPU02 holds writers until the interrupted PostgreSQL backend is gone (query failure=${queryFails})`, () => {
  const dir = mkdtempSync(path.join(tmpdir(), "gpu02-pg-backend-"));
  const runtime = path.join(dir, "runtime");
  mkdirSync(runtime);
  const database = `review-${path.basename(dir).toLowerCase()}`;
  const migration = `gpu02-revert-${process.pid}-${Date.now()}`;
  const docker = Bun.spawnSync(["bash", "-c", "type -P docker"]).stdout.toString().trim();
  const call = (args: string[]) => Bun.spawnSync([docker, ...args], {timeout: 20000});
  const sql = (query: string) => {
    const result = call(["exec", database, "psql", "-U", "postgres", "-A", "-t", "-v", "ON_ERROR_STOP=1", "-c", query]);
    expect(result.exitCode, result.stderr.toString()).toBe(0);
    return result.stdout.toString().trim();
  };
  try {
    const started = call(["run", "-d", "--rm", "--name", database, "--network", "none", "--memory", "256m", "--cpus", "0.5", "-e", "POSTGRES_HOST_AUTH_METHOD=trust", "postgres:15.7"]);
    expect(started.exitCode, started.stderr.toString()).toBe(0);
    let ready = false;
    for (let i = 0; i < 100; i++) {
      if (call(["exec", database, "pg_isready", "-h", "127.0.0.1", "-U", "postgres"]).exitCode === 0) { ready = true; break; }
      Bun.spawnSync(["sleep", "0.1"]);
    }
    expect(ready).toBe(true);
    sql("CREATE TABLE evidence(id int primary key); INSERT INTO evidence VALUES(1); CREATE FUNCTION slow_delete() RETURNS trigger LANGUAGE plpgsql AS $$BEGIN PERFORM pg_sleep(30); RETURN OLD; END$$; CREATE TRIGGER slow_delete BEFORE DELETE ON evidence FOR EACH ROW EXECUTE FUNCTION slow_delete();");
    writeFileSync(path.join(dir, "revert.sql"), "BEGIN;\nDELETE FROM evidence;\nCOMMIT;\n");
    const created = call(["run", "-d", "--name", migration, "--label", `fhevm.gpu02-owner=${runtime}`, "--network", `container:${database}`, "--memory", "128m", "--cpus", "0.2", "--entrypoint", "/bin/sh",
      "-e", `PGAPPNAME=${migration}`, "-e", `DATABASE_URL=postgresql://postgres:sentinel-private@127.0.0.1/postgres?application_name=${migration}`,
      "-v", `${dir}/revert.sql:/revert.sql:ro`, "postgres:15.7", "-c", 'psql "$DATABASE_URL" -v ON_ERROR_STOP=1 -f /revert.sql; echo finished']);
    expect(created.exitCode, created.stderr.toString()).toBe(0);
    const unrelated = call(["exec", "-d", "-e", "PGAPPNAME=unrelated_writer", database, "psql", "-h", "127.0.0.1", "-U", "postgres", "-c", "SELECT pg_sleep(30)"]);
    expect(unrelated.exitCode).toBe(0);
    const active = `SELECT count(*) FROM pg_stat_activity WHERE application_name='${migration}' AND state='active' AND query='DELETE FROM evidence;' AND wait_event='PgSleep'`;
    let observed = false;
    for (let i = 0; i < 100; i++) {
      if (sql(active) === "1") { observed = true; break; }
      Bun.spawnSync(["sleep", "0.05"]);
    }
    expect(observed).toBe(true);
    expect(call(["kill", "--signal", "KILL", migration]).exitCode).toBe(0);
    // Negative control for the old Docker-only proof: client terminal, SQL active.
    expect(call(["inspect", "-f", "{{.State.Status}} {{.State.Running}} {{.State.Pid}}", migration]).stdout.toString().trim()).toBe("exited false 0");
    expect(sql(active)).toBe("1");
    writeFileSync(path.join(dir, "docker-proxy"), `#!/bin/bash\nif [[ '${queryFails}' == true && "$1" == run ]]; then exit 1; fi\nexec ${q(docker)} "$@"\n`, {mode: 0o755});
    const result = Bun.spawnSync(["bash", "-c", `set -uo pipefail
SCRIPT_DIR=${q(scripts)}; REPO_ROOT=${q(dir)}; FHEVM_STATE_DIR=${q(dir)}; SP_RUNTIME_DIR=${q(runtime)}
source "$SCRIPT_DIR/lib/service-control.sh"; sc_init
source "$SCRIPT_DIR/lib/suite-process.sh"; sp_init
source "$SCRIPT_DIR/lib/gpu02-transport.sh"
GPU02_REAL_DOCKER=${q(path.join(dir, "docker-proxy"))}; export GPU02_REAL_DOCKER SP_RUNTIME_DIR
GPU02_MIGRATION_REGISTRY="$SP_RUNTIME_DIR/migrations"
printf '%s\\n' ${q(migration)} > "$GPU02_MIGRATION_REGISTRY"
touch "$GPU02_MIGRATION_REGISTRY.${migration}.start-requested"
sp_recover_suite_state() { return 0; }
hc_begin_cleanup
if gpu02_stop_remote; then echo WRITERS_MAY_RESTORE; else echo WRITERS_REMAIN_HELD; fi
`], {timeout: 20000});
    expect(result.exitCode, result.stderr.toString()).toBe(0);
    expect(result.stdout.toString().trim()).toBe(queryFails ? "WRITERS_REMAIN_HELD" : "WRITERS_MAY_RESTORE");
    expect(result.stderr.toString()).not.toContain("sentinel-private");
    expect(sql(`SELECT count(*) FROM pg_stat_activity WHERE application_name='${migration}'`)).toBe(queryFails ? "1" : "0");
    expect(sql("SELECT count(*) FROM pg_stat_activity WHERE application_name='unrelated_writer'")).toBe("1");
    if (!queryFails) expect(sql("SELECT count(*) FROM evidence")).toBe("1"); // killed DELETE rolled back
    expect(existsSync(path.join(dir, "runtime/failure-matrix/uncancelled-phase"))).toBe(queryFails);
    if (queryFails) expect(readFileSync(path.join(runtime, "migrations"), "utf8")).toContain(migration);
  } finally {
    call(["rm", "-f", migration]);
    call(["rm", "-f", database]);
    rmSync(dir, {recursive: true, force: true});
  }
}, 40000);
