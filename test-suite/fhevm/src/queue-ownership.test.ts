import { expect, test } from "bun:test";
import { mkdtempSync, mkdirSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

const kinds = ["tfhe", "sns", "zkproof"];
const container = (name: string, pid: number, kind = "tfhe", network = "local") => ({
  Name: `/${name}`, State: {Pid: pid, Status: "running"},
  Config: {Cmd: [`/${kind}_worker`], Env: ["DATABASE_URL=postgres://db/coprocessor"],
    ExposedPorts: kind === "postgres" ? {"5432/tcp":{}} : {}},
  NetworkSettings: {Networks: {stack: {NetworkID: network, Aliases: kind === "postgres" ? ["db"] : [], IPAddress: kind === "postgres" ? "172.29.0.2" : ""}}},
});
const fleet = () => kinds.map((kind, index) => container(`coprocessor-${kind}-worker`, 100 + index, kind));
type Fixture = {containers: ReturnType<typeof container>[]; hosts?: Record<string, number[]>; gpu?: boolean; blueGreen?: boolean; gpuMissing?: string; child?: number; pgrepError?: boolean; inspectGone?: boolean; inspectError?: boolean; managed?: string[]; quiescent?: boolean; topGone?: string};
const exercise = (fixture: Fixture) => {
  const dir = mkdtempSync(path.join(tmpdir(), "queue-owners-"));
  try {
    const bin = path.join(dir, "bin"); mkdirSync(bin);
    if (fixture.gpu) {mkdirSync(path.join(dir, "runtime/gpu-consensus-workers"), {recursive: true}); writeFileSync(path.join(dir, "runtime/gpu-consensus-workers/node-config.env"), "");}
    writeFileSync(path.join(dir, "fixture.json"), JSON.stringify(fixture));
    const tool = `#!${process.execPath}
const fs = await import('node:fs');
const f = JSON.parse(fs.readFileSync(process.env.FAKE_ROOT + '/fixture.json', 'utf8'));
if (f.topGone && fs.existsSync(process.env.FAKE_ROOT+'/top-gone')) f.containers=f.containers.filter(c=>c.Name!=='/'+f.topGone);
const tool = process.argv[1].split('/').pop(); const a = process.argv.slice(2);
if (tool === 'docker') {
 if (a[0] === 'ps') console.log(f.containers.map((_,i) => i).join(' '));
 else if (a[0] === 'inspect') {
  if (f.inspectError) { console.error('Cannot connect to Docker daemon'); process.exit(1); }
  if (f.inspectGone && !fs.existsSync(process.env.FAKE_ROOT+'/gone')) {
   fs.writeFileSync(process.env.FAKE_ROOT+'/gone','yes'); console.error('Error: No such object: disposable'); process.exit(1);
  }
  console.log(JSON.stringify(f.containers));
 }
 else if (a[0] === 'top') {
  if (a[1] === f.topGone) {fs.writeFileSync(process.env.FAKE_ROOT+'/top-gone','yes');console.error('No such container: '+a[1]);process.exit(1);}
  console.log('PID COMMAND\\n' + (a[1] === 'coprocessor-tfhe-worker' && f.child ? f.child + ' tfhe_worker' : ''));
 }
 else process.exit(2);
} else if (tool === 'pgrep') {
 if (f.pgrepError) { console.error('permission denied'); process.exit(2); }
 const kind = a.at(-1).replace('_worker','');
 const pids = f.hosts?.[kind] ?? f.containers.filter(c => c.Config.Cmd[0] === '/' + kind + '_worker').map(c => c.State.Pid);
 if (!pids.length) process.exit(1); console.log(pids.join('\\n'));
} else if (tool === 'systemctl') {
 const unit = a.find(x => x.startsWith('fhevm-gpu-'));
 const kind = unit.split('-').at(-2); const pid = f.gpuMissing === kind || f.gpuMissing === 'all' ? 0 : 200 + ['tfhe','sns','zkproof'].indexOf(kind);
 console.log('ActiveState=' + (pid ? 'active' : 'inactive') + '\\nMainPID=' + pid);
} else if (tool === 'cat') {
 const pid = Number(a[0].split('/')[2]);
 if (pid < 200 || pid > 202) {console.error('EACCES'); process.exit(1);}
 if (a[0].endsWith('/stat')) console.log(pid + ' (worker) S 1');
 else if (a[0].endsWith('/cmdline')) console.log('--database-url=postgres://db/coprocessor');
 else console.log('');
} else process.exit(2);
`;
    for (const name of ["docker", "pgrep", "systemctl", "cat"]) writeFileSync(path.join(bin, name), tool, {mode:0o755});
    const module = path.join(import.meta.dir, "flow/queue-ownership.ts");
    const result = Bun.spawnSync([process.execPath, "-e", `import {assertOneWorkerPerQueue} from ${JSON.stringify(module)}; await assertOneWorkerPerQueue({scenario:{kind:${JSON.stringify(fixture.blueGreen ? "blue-green" : "coprocessor-consensus")},topology:{count:1,threshold:1}}},${JSON.stringify({allowGpu:true,requireEveryRole:!fixture.quiescent && !fixture.managed,managedWriterContainers:fixture.managed,requireQuiescent:fixture.quiescent})});`], {env:{...process.env, FHEVM_STATE_DIR:dir, FAKE_ROOT:dir, PATH:`${bin}:${process.env.PATH}`}});
    return {code:result.exitCode, error:result.stderr.toString()};
  } finally {rmSync(dir, {recursive:true, force:true});}
};

test("ordinary and version-gated blue/green owners are recognized, without accepting GCS names on an ordinary stack", () => {
  expect(exercise({containers:fleet()}).code).toBe(0);
  const containers = [...fleet(), ...kinds.map((kind, index) => container(`coprocessor-gcs-${kind}-worker`, 110 + index, kind))];
  expect(exercise({containers, blueGreen:true}).code).toBe(0);
  // After cutover BCS retires while the planned green service names remain.
  expect(exercise({containers:containers.slice(3), blueGreen:true}).code).toBe(0);
  const pausedGreen = containers.slice(3).map(c => ({...c, State:{...c.State, Status:"paused"}}));
  expect(exercise({containers:pausedGreen, blueGreen:true}).error).toContain("expected one running owner");
  expect(exercise({containers}).error).toContain("coprocessor-gcs");
});
test("rogue Docker workers in every role are rejected but isolated queues are permitted", () => {
  for (const kind of kinds) {
    // No pgrep result for the extra container: its command still identifies it.
    const hosts = Object.fromEntries(kinds.map((name, index) => [name, [100 + index]]));
    expect(exercise({containers:[...fleet(), container("rogue", 999, kind)], hosts}).error).toContain("/rogue pid=999");
    expect(exercise({containers:[...fleet(), container("other-db", 998, "postgres", "isolated"), container("rogue", 999, kind, "isolated")], hosts}).code).toBe(0);
  }
});
test("disjoint networks do not exempt workers targeting the host-published stack DB", () => {
  for (const [network, host] of [["host", "127.0.0.1"], ["isolated", "172.17.0.1"], ["isolated", "host.docker.internal"]]) {
    const rogue = container("rogue", 999, "tfhe", network);
    rogue.Config.Env = [`DATABASE_URL=postgres://postgres:postgres@${host}:5432/coprocessor`];
    expect(exercise({containers:[...fleet(), rogue]}).error).toContain("/rogue pid=999");
  }
});
test("an independently identified DB address is isolated, while unresolved aliases fail closed", () => {
  const rogue = container("rogue", 999, "tfhe", "isolated");
  expect(exercise({containers:[...fleet(), rogue]}).error).toContain("/rogue pid=999");
  const db = container("other-db", 998, "postgres", "isolated");
  rogue.Config.Env = ["DATABASE_URL=postgres://172.29.0.2:5432/coprocessor"];
  expect(exercise({containers:[...fleet(), db, rogue]}).code).toBe(0);
  rogue.Config.Env = ["DATABASE_URL=postgres://172.29.0.2:5433/coprocessor"];
  expect(exercise({containers:[...fleet(), db, rogue]}).error).toContain("/rogue pid=999");
});
test("SQLx host, port and database query overrides cannot conceal the owned queue", () => {
  const rogue = container("rogue", 999, "tfhe", "isolated");
  const db = container("other-db", 998, "postgres", "isolated");
  for (const url of [
    "postgres://127.0.0.1:5432/unrelated?dbname=coprocessor",
    "postgres://db:5432/coprocessor?host=127.0.0.1",
    "postgres://db:5432/coprocessor?host=db&host=127.0.0.1",
    "postgres://127.0.0.1:6543/unrelated?host=127.0.0.1&port=5432&dbname=coprocessor",
  ]) {
    rogue.Config.Env = [`DATABASE_URL=${url}`];
    expect(exercise({containers:[...fleet(), db, rogue]}).error).toContain("/rogue pid=999");
  }
  rogue.Config.Env = ["DATABASE_URL=postgres://127.0.0.1:6543/coprocessor?host=db&port=5432"];
  expect(exercise({containers:[...fleet(), db, rogue]}).code).toBe(0);
});
test("an extra-host override prevents treating a DB alias as proof of isolation", () => {
  const rogue = {...container("rogue", 999, "tfhe", "isolated"), HostConfig:{ExtraHosts:["db:host-gateway"]}};
  const db = container("other-db", 998, "postgres", "isolated");
  expect(exercise({containers:[...fleet(), db, rogue]}).error).toContain("/rogue pid=999");
  rogue.HostConfig.ExtraHosts = ["DB=172.17.0.1"];
  expect(exercise({containers:[...fleet(), db, rogue]}).error).toContain("/rogue pid=999");
  rogue.HostConfig.ExtraHosts = ["unrelated:host-gateway"];
  expect(exercise({containers:[...fleet(), db, rogue]}).code).toBe(0);
});
test("wrapper child processes are attributed through Docker, without reading root-only process environments", () => {
  expect(exercise({containers:fleet(), hosts:{tfhe:[199], sns:[101], zkproof:[102]}, child:199}).code).toBe(0);
});
test("GPU ownership covers all roles, rejects a displaced CPU owner and missing GPU owners", () => {
  const db = container("coprocessor-and-kms-db", 50, "postgres");
  const hosts = {tfhe:[200], sns:[201], zkproof:[202]};
  expect(exercise({containers:[db], gpu:true, hosts}).code).toBe(0);
  expect(exercise({containers:[db, fleet()[1]], gpu:true, hosts}).error).toContain("both Docker and GPU");
  expect(exercise({containers:[db], gpu:true, hosts, gpuMissing:"sns"}).error).toContain("expected one running owner");
});
test("unregistered host workers and unreadable process enumeration fail closed", () => {
  expect(exercise({containers:fleet(), hosts:{tfhe:[100,999], sns:[101], zkproof:[102]}}).error).toContain("host worker pid=999");
  expect(exercise({containers:fleet(), pgrepError:true}).error).toContain("Cannot enumerate host");
});
test("a stale GPU marker cannot relabel a healthy Docker fleet", () => {
  expect(exercise({containers:fleet(), gpu:true, gpuMissing:'all'}).error).toContain('GPU session ownership is recorded');
});
test("Docker discovery retries a disappeared container but rejects daemon errors", () => {
  expect(exercise({containers:fleet(), inspectGone:true}).code).toBe(0);
  expect(exercise({containers:fleet(), inspectError:true}).error).toContain('Could not inspect Docker queue owners');
});

test("SQL quiescence rejects restarting managed owners even before they have a PID", () => {
  const worker = container("coprocessor-tfhe-worker", 0);
  worker.State.Status = "restarting";
  expect(exercise({containers:[worker], quiescent:true, managed:["coprocessor-tfhe-worker"]}).error).toContain("managed writer is still restarting");
});
test("SQL quiescence rejects unmanaged host listeners and permits a fully stopped GPU fleet", () => {
  const db = container("coprocessor-and-kms-db", 50, "postgres");
  expect(exercise({containers:[db], managed:[], hosts:{host_listener_p:[999]}, quiescent:true}).error).toContain("host worker pid=999");
  expect(exercise({containers:[db], managed:[], hosts:{}, gpu:true, gpuMissing:"all", quiescent:true}).code).toBe(0);
});

test("wrapper attribution retries a complete snapshot after an unrelated container disappears", () => {
  const hosts={tfhe:[199],sns:[101],zkproof:[102]};
  expect(exercise({containers:[...fleet(),container("disposable",999,"postgres")],hosts,child:199,topGone:"disposable"}).code).toBe(0);
  expect(exercise({containers:fleet(),hosts,child:199,topGone:"coprocessor-tfhe-worker"}).code).toBe(1);
});
test("green Docker owners cannot coexist with GPU owners even on blue-green", () => {
  const db=container("coprocessor-and-kms-db",50,"postgres");
  const green=container("coprocessor-gcs-tfhe-worker",111);
  expect(exercise({containers:[db,green],hosts:{tfhe:[200],sns:[201],zkproof:[202]},gpu:true,blueGreen:true}).error).toContain("both GCS Docker and GPU");
});
