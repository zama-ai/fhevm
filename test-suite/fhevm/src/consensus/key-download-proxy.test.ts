import { expect, test } from "bun:test";
import { mkdtempSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

test("the download proxy client survives the ordinary host-command timeout", async () => {
  const directory = mkdtempSync(path.join(tmpdir(), "key-proxy-lifetime-"));
  try {
    const scripts = path.resolve(import.meta.dir, "../../scripts");
    const launch = readFileSync(path.join(scripts, "hold-key-download.sh"), "utf8")
      .split("\n").find(line => line.includes('docker exec "$TEST_CONTAINER" node "$remote/proxy.cjs"'));
    expect(launch).toBeDefined();
    mkdirSync(path.join(directory, "bin"));
    writeFileSync(path.join(directory, "bin/docker"), "#!/bin/bash\nsleep 2\necho proxy-shutdown\n", { mode: 0o755 });
    const child = Bun.spawn(["bash", "-c", `set -euo pipefail
source "$SCRIPT_DIR/lib/host-command.sh"
TEST_CONTAINER=isolated; remote=/isolated; MIGRATION_KEY_HEX=fixture; wrong_args=()
${launch}
proxy_pid=$!
wait "$proxy_pid"
`], { env: { ...process.env, SCRIPT_DIR: scripts, PATH: `${directory}/bin:${process.env.PATH}`,
      HC_COMMAND_TIMEOUT_SECONDS: "1", CASE_DEADLINE_EPOCH: "", HC_CLEANUP_DEADLINE_EPOCH: "" },
      stdout: "pipe", stderr: "pipe", timeout: 5000 });
    const [status, errors] = await Promise.all([child.exited, new Response(child.stderr).text()]);
    expect(status, errors).toBe(0);
    expect(errors).toContain("proxy-shutdown");
  } finally { rmSync(directory, { recursive: true, force: true }); }
}, 6000);

test("the download proxy client is its own process group and cleanup cancels the whole group", async () => {
  const directory = mkdtempSync(path.join(tmpdir(), "key-proxy-group-"));
  try {
    const scripts = path.resolve(import.meta.dir, "../../scripts");
    const source = readFileSync(path.join(scripts, "hold-key-download.sh"), "utf8").split("\n");
    const launch = source.find(line => line.includes('docker exec "$TEST_CONTAINER" node "$remote/proxy.cjs"'));
    const cancel = source.find(line => line.includes('kill -- "-$proxy_pid"'));
    expect(launch).toBeDefined();
    expect(cancel, "cleanup must cancel the client's process group, not only the job's shell").toBeDefined();
    mkdirSync(path.join(directory, "bin"));
    // A client that ignores the shutdown control: it only dies when killed.
    writeFileSync(path.join(directory, "bin/docker"), "#!/bin/bash\nexec sleep 600\n", { mode: 0o755 });
    const child = Bun.spawn(["bash", "-c", `set -euo pipefail
source "$SCRIPT_DIR/lib/host-command.sh"
TEST_CONTAINER=isolated; remote=/isolated; MIGRATION_KEY_HEX=group-test-$$; wrong_args=()
${launch}
proxy_pid=$!
for ((attempt=0;attempt<50;attempt++)); do
  [[ "$(ps -o pgid= -p "$proxy_pid" | tr -d ' ')" == "$proxy_pid" ]] && break; sleep 0.1
done
[[ "$(ps -o pgid= -p "$proxy_pid" | tr -d ' ')" == "$proxy_pid" ]] || { echo not-own-group >&2; exit 3; }
until pgrep -f "sleep 600" >/dev/null; do sleep 0.1; done
members_before="$(ps -o pid= -g "$proxy_pid" | wc -l)"
kill -- "-$proxy_pid"
wait "$proxy_pid" || true
for ((attempt=0;attempt<30;attempt++)); do
  [[ -z "$(ps -o pid= -g "$proxy_pid" 2>/dev/null)" ]] && break; sleep 0.1
done
echo "members_before=$members_before members_after=$(ps -o pid= -g "$proxy_pid" 2>/dev/null | wc -l)"
`], { env: { ...process.env, SCRIPT_DIR: scripts, PATH: `${directory}/bin:${process.env.PATH}`,
      HC_COMMAND_TIMEOUT_SECONDS: "1", CASE_DEADLINE_EPOCH: "", HC_CLEANUP_DEADLINE_EPOCH: "" },
      stdout: "pipe", stderr: "pipe", timeout: 10_000 });
    const [status, output, errors] = await Promise.all([child.exited, new Response(child.stdout).text(), new Response(child.stderr).text()]);
    expect(status, errors).toBe(0);
    // timeout plus the docker client were in the group; nothing survives the cancel.
    expect(output).toMatch(/members_before=[2-9] members_after=0/);
  } finally { rmSync(directory, { recursive: true, force: true }); }
}, 12_000);

test("key download faults select one real GET and recovery returns the original body", async () => {
  const child = Bun.spawn(["node", "-e", `
    const assert=require('node:assert/strict'), http=require('node:http'), fs=require('node:fs'), os=require('node:os');
    const {keyProxy}=require(process.argv[1]);
    (async()=>{
      const root=fs.mkdtempSync(os.tmpdir()+'/key-proxy-test-'); const wrong=root+'/wrong'; fs.writeFileSync(wrong,'different key fixture');
      const original=Buffer.alloc(16384,7), events=[], key='ab'.repeat(32);let mode='healthy';
      const upstream=http.createServer((req,res)=>res.end(original));
      await new Promise(r=>upstream.listen(0,'127.0.0.1',r));
      const proxy=keyProxy({upstream:'http://127.0.0.1:'+upstream.address().port,key,wrongKey:wrong,control:()=>({mode,until:Date.now()+60000}),observe:e=>events.push(e)});
      await new Promise(r=>proxy.listen(0,'127.0.0.1',r));
      const base='http://127.0.0.1:'+proxy.address().port, path='/bucket/prefix/CompressedXofKeySet/'+key;
      const read=async(path,signal)=>Buffer.from(await (await fetch(base+path,{signal})).arrayBuffer());
      try {
        assert.deepEqual(await read(path),original);
        for(mode of ['wrong-digest','malformed','wrong-key']) {
          assert.notDeepEqual(await read(path),original);
          assert.deepEqual(await read('/bucket/PublicKey/'+key),original);
          assert.deepEqual(await read('/bucket/CompressedXofKeySet/'+'cd'.repeat(32)),original);
        }
        assert.equal(events.filter(e=>e.mode==='wrong-key').length,1);
        mode='interrupt';await assert.rejects(read(path,AbortSignal.timeout(150)));
        for(let i=0;i<50&&!events.some(e=>e.mode==='interrupted-close');i++)await new Promise(r=>setTimeout(r,10));
        const partial=events.find(e=>e.mode==='interrupt');assert.ok(partial.bytes>0&&partial.bytes<partial.total);
        assert.ok(events.some(e=>e.mode==='interrupted-close'));
        mode='healthy';assert.deepEqual(await read(path),original);
        assert.equal(events.filter(e=>e.mode==='healthy').length,2);
      }finally{proxy.closeAllConnections();proxy.close();upstream.closeAllConnections();upstream.close();fs.rmSync(root,{recursive:true});}
    })().catch(e=>{console.error(e);process.exitCode=1});
  `, new URL("../../scripts/lib/key-download-proxy.cjs", import.meta.url).pathname], { stdout: "pipe", stderr: "pipe", timeout: 10_000 });
  const [status, errors] = await Promise.all([child.exited, new Response(child.stderr).text()]);
  expect(errors).toBe("");
  expect(status).toBe(0);
});
