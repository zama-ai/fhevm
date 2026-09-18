import { expect, test } from "bun:test";
import { mkdtempSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

const cli = path.resolve(import.meta.dir, "..");
const owner = (name: string, pid: number, command: string, status = "running") => ({
  Name: `/${name}`, State: {Pid: pid, Status: status},
  Config: {Cmd: [command], Env: ["DATABASE_URL=postgres://db/coprocessor"]},
  NetworkSettings: {Networks: {stack: {NetworkID: "fixture"}}},
});
function runFixture(mode: string) {
  const dir = mkdtempSync(path.join(tmpdir(), "writer-transaction-"));
  try {
    const bin = path.join(dir, "bin"); mkdirSync(bin);
    const green = mode === "green";
    const names = ["coprocessor-tfhe-worker", "coprocessor-host-listener", ...(green ? ["coprocessor-gcs-tfhe-worker", "coprocessor-gcs-host-listener-poller"] : [])];
    const containers = [owner("coprocessor-and-kms-db", 90, "/postgres"),
      owner(names[0]!, 100, "/tfhe_worker", mode === "paused" ? "paused" : "running"),
      owner(names[1]!, 101, "/host_listener", mode === "stopped" ? "exited" : "running"),
      ...(green ? [owner(names[2]!, 102, "/tfhe_worker"), owner(names[3]!, 103, "/host_listener_poller")] : []),
      ...(mode === "rogue" ? [owner("unmanaged-listener", 999, "/host_listener_poller")] : []),
    ];
    writeFileSync(path.join(dir, "fixture.json"), JSON.stringify({containers, mode}));
    writeFileSync(path.join(dir, "trace"), "");
    const tool = `#!${process.execPath}
const fs = await import('node:fs'); const root=process.env.FIXTURE;
const f=JSON.parse(fs.readFileSync(root+'/fixture.json','utf8')); const a=process.argv.slice(2);
const name=process.argv[1].split('/').pop();
fs.appendFileSync(root+'/trace',name+' '+a.join(' ')+'\\n');
if(name==='pgrep') {process.exit(1);}
if(name==='docker') {
 if(a[0]==='ps') {
  if(f.mode==='late-rogue' && f.containers.some(c=>c.State.Status==='exited')) {
   f.containers.push(${JSON.stringify(owner("late-listener", 999, "/gw_listener"))}); f.mode='late-added';
   fs.writeFileSync(root+'/fixture.json',JSON.stringify(f));
  }
  console.log(f.containers.filter(c=>c.State.Status!=='exited').map(c=>c.Name.slice(1)).join(' '));
 } else if(a[0]==='inspect') {
  if(a[1]==='-f') {
   const c=f.containers.find(c=>c.Name.slice(1)===a.at(-1));
   if(!c) {console.error('No such container');process.exit(1);}
   console.log(c.State.Status);
  } else console.log(JSON.stringify(f.containers.filter(c=>a.slice(1).includes(c.Name.slice(1)))));
 } else if(a[0]==='stop'||a[0]==='start') {
  const c=f.containers.find(c=>c.Name.slice(1)===a[1]); if(!c)process.exit(2);
  c.State.Status=a[0]==='stop'?'exited':'running'; fs.writeFileSync(root+'/fixture.json',JSON.stringify(f));
 } else process.exit(2);
} else process.exit(2);
`;
    for (const name of ["docker", "pgrep"]) writeFileSync(path.join(bin, name), tool, {mode:0o755});
    const result = Bun.spawnSync([process.execPath, "-e", `
import {withQuiescedWriters} from '${cli}/src/flow/writer-ownership.ts';
await withQuiescedWriters({scenario:{kind:'${green ? "blue-green" : "coprocessor-consensus"}',topology:{count:1,threshold:1}}},${JSON.stringify(names)},async()=>console.log('SQL_PERMITTED'));
`], {env:{...process.env, FHEVM_STATE_DIR:dir, FIXTURE:dir, PATH:`${bin}:${process.env.PATH}`}});
    return {status:result.exitCode, output:result.stdout.toString(), error:result.stderr.toString(),
      trace:readFileSync(path.join(dir,"trace"),"utf8"), state:JSON.parse(readFileSync(path.join(dir,"fixture.json"),"utf8")).containers as typeof containers};
  } finally {rmSync(dir,{recursive:true,force:true});}
}

test("DB mutation refuses unmanaged listener writers before stopping any managed owner", () => {
  const result=runFixture("rogue");
  expect(result.status).toBe(1); expect(result.error).toContain("unmanaged-listener");
  expect(result.trace).not.toContain("docker stop"); expect(result.output).not.toContain("SQL_PERMITTED");
});
test("a writer appearing after managed stop prevents SQL and restores every originally running owner", () => {
  const result=runFixture("late-rogue");
  expect(result.status).toBe(1); expect(result.error).toContain("late-listener");
  expect(result.output).not.toContain("SQL_PERMITTED");
  expect(result.state.filter(c=>c.State.Status!=="running")).toHaveLength(0);
  expect(result.trace).toContain("docker start coprocessor-tfhe-worker");
  expect(result.trace).toContain("docker start coprocessor-host-listener");
  expect(result.trace).not.toContain("docker stop late-listener");
});
test("both managed blue and green writer families are quiesced and restored", () => {
  const result=runFixture("green");
  expect(result.status,result.error).toBe(0); expect(result.output).toContain("SQL_PERMITTED");
  for(const name of ["coprocessor-tfhe-worker","coprocessor-host-listener","coprocessor-gcs-tfhe-worker","coprocessor-gcs-host-listener-poller"]) {
    expect(result.trace).toContain(`docker stop ${name}`); expect(result.trace).toContain(`docker start ${name}`);
  }
});
test("a previously stopped writer stays stopped, while a paused owner refuses before mutation", () => {
  const stopped=runFixture("stopped");
  expect(stopped.status,stopped.error).toBe(0);
  expect(stopped.trace).not.toContain("docker start coprocessor-host-listener");
  const paused=runFixture("paused");
  expect(paused.status).toBe(1); expect(paused.error).toContain("while it is paused");
  expect(paused.trace).not.toContain("docker stop");
});

test("DB revert includes every blue/green writer and every extra-chain listener", async () => {
  const {coprocessorRuntimeContainers}=await import("./commands/test");
  const names=coprocessorRuntimeContainers(2,[{key:"ethereum"},{key:"second"}] as never,true);
  expect(names).toContain("coprocessor-gcs-upgrade-controller");
  expect(names).toContain("coprocessor1-gcs-consensus-detector");
  expect(names.filter(name=>name.includes("gcs-host-listener")).length).toBe(10);
});

test("destructive DB profile refuses external targets but accepts every managed operator database", async () => {
  const {assertDbRevertTarget}=await import("./commands/test");
  const scenario={kind:"coprocessor-consensus",topology:{count:3,threshold:3}} as never;
  const base={postgresContainer:"coprocessor-and-kms-db",postgresDb:"coprocessor",postgresHost:undefined};
  for(const postgresDb of ["coprocessor","coprocessor_1","coprocessor_2"]) {
    expect(()=>assertDbRevertTarget({scenario},{...base,postgresDb})).not.toThrow();
  }
  for(const override of [{postgresContainer:"external-db"},{postgresHost:"external:5432"},{postgresDb:"coprocessor_3"},{postgresDb:"kms"}]) {
    expect(()=>assertDbRevertTarget({scenario},{...base,...override})).toThrow("active managed coprocessor fleet");
  }
  const runner=readFileSync(path.join(cli,"src/commands/test.ts"),"utf8");
  const profile=runner.slice(runner.indexOf("const runDbStateRevert = async"));
  expect(profile.indexOf("assertDbRevertTarget(state, postgres)")).toBeLessThan(profile.indexOf("await runNamedE2e"));
});
