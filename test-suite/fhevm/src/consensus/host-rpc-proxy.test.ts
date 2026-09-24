import { expect, test } from "bun:test";
import { createRequire } from "node:module";
import { createServer } from "node:http";
import { pollerRoute } from "./host-rpc-route";
const { rpcProxy } = createRequire(import.meta.url)("../../scripts/lib/host-rpc-proxy.cjs");
const token = "a".repeat(32);
test("isolated proxy observes each transport fault and forwards the original request after release", async () => {
  let mode = "healthy";
  const events: any[] = [], forwarded: any[] = [];
  const upstream = createServer((request, response) => {
    let body = "";
    request.on("data", chunk => { body += chunk; });
    request.on("end", () => { forwarded.push(JSON.parse(body)); response.setHeader("content-type", "application/json"); response.end(JSON.stringify({ jsonrpc: "2.0", id: 7, result: [] })); });
  });
  await new Promise<void>(resolve => upstream.listen(0, "127.0.0.1", resolve));
  const proxy = rpcProxy({ upstream: `http://127.0.0.1:${(upstream.address() as any).port}`, token, control: () => ({ mode, until: Date.now() + 60_000 }), observe: (event: unknown) => events.push(event) });
  await new Promise<void>(resolve => proxy.listen(0, "127.0.0.1", resolve));
  const url = `http://127.0.0.1:${proxy.address().port}/${token}`;
  const request = { jsonrpc: "2.0", id: 7, method: "eth_getLogs", params: [{ fromBlock: "0x1", toBlock: "0x4" }] };
  const send = (value = request) => fetch(url, { method: "POST", body: JSON.stringify(value), signal: AbortSignal.timeout(2_000) });
  try {
    for (mode of ["429", "503", "408"]) { expect((await send()).status).toBe(Number(mode)); expect(forwarded).toHaveLength(0); }
    mode = "reset";
    await expect(send()).rejects.toThrow();
    expect(forwarded).toHaveLength(0);
    const healthyMethod = { ...request, method: "eth_chainId" };
    expect((await send(healthyMethod)).status).toBe(200);
    expect(forwarded).toEqual([healthyMethod]);
    mode = "healthy";
    expect((await send()).status).toBe(200);
    expect(forwarded[1]).toEqual(request);
    expect(new Set(events.map(event => event.mode))).toEqual(new Set(["429", "503", "408", "reset", "healthy"]));
    expect(events.every(event => event.method === request.method && JSON.stringify(event.params) === JSON.stringify(request.params))).toBe(true);
    expect((await fetch(url + "unowned", { method: "POST" })).status).toBe(404);
  } finally { proxy.closeAllConnections(); proxy.close(); upstream.closeAllConnections(); upstream.close(); }
});
test("routing changes only the unique explicit RPC argument", () => {
  const source = ["--batch-size=4", "--url", "http://host-node:8545", "--database-url=secret"];
  expect(pollerRoute(source, "http://proxy:1234/token")).toEqual({ original: "http://host-node:8545", command: ["--batch-size=4", "--database-url=secret", "--url=http://proxy:1234/token"] });
  for (const command of [[], ["--url"], ["--url=ws://host"], ["--url=http://one", "--url=http://two"]]) expect(() => pollerRoute(command)).toThrow();
});

test("silent responses require a client abort; releasing the proxy cannot manufacture it", async () => {
  // The deployed proxy runs on Node. Exercise its real socket-close behavior;
  // Bun's HTTP compatibility layer does not expose the same abort events.
  const child = Bun.spawn(["node", "-e", `
    const assert = require("node:assert/strict");
    const { rpcProxy } = require(process.argv[1]);
    (async () => {
      let mode = "stall";
      const events = [], token = "a".repeat(32);
      const proxy = rpcProxy({ upstream: "http://127.0.0.1:1", token,
        control: () => ({ mode, until: Date.now() + 60000 }), observe: e => events.push(e) });
      await new Promise(resolve => proxy.listen(0, "127.0.0.1", resolve));
      const send = ms => fetch("http://127.0.0.1:" + proxy.address().port + "/" + token, {
        method: "POST", body: JSON.stringify({ jsonrpc: "2.0", id: 1, method: "eth_getLogs", params: [] }),
        signal: AbortSignal.timeout(ms) });
      const pause = () => new Promise(resolve => setTimeout(resolve, 10));
      try {
        await assert.rejects(send(150));
        for (let i=0; i<50 && !events.some(e => e.mode === "stall-aborted"); i++) await pause();
        assert.equal(events.filter(e => e.mode === "stall-aborted").length, 1);
        assert.ok(events.find(e => e.mode === "stall-aborted").elapsedMs > 50);
        const released = assert.rejects(send(2000));
        for (let i=0; i<50 && events.filter(e => e.mode === "stall").length < 2; i++) await pause();
        assert.equal(events.filter(e => e.mode === "stall").length, 2);
        mode = "healthy";
        await released;
        assert.equal(events.filter(e => e.mode === "stall-aborted").length, 1);
      } finally { mode = "healthy"; proxy.closeAllConnections(); proxy.close(); }
    })().catch(error => { console.error(error); process.exitCode = 1; });
  `, new URL("../../scripts/lib/host-rpc-proxy.cjs", import.meta.url).pathname], {
    stdout: "pipe", stderr: "pipe", timeout: 5_000,
  });
  const [status, errors] = await Promise.all([child.exited, new Response(child.stderr).text()]);
  expect(errors).toBe("");
  expect(status).toBe(0);
});

test("HTTPS control requires a real trust failure and recovers without changing the route", async () => {
  const { mkdtemp, rm } = await import("node:fs/promises");
  const directory = await mkdtemp("/tmp/host-rpc-tls-unit-");
  try {
    for (const name of ["valid", "invalid"]) {
      const child = Bun.spawn(["openssl", "req", "-x509", "-newkey", "rsa:2048", "-nodes", "-days", "1",
        "-subj", "/CN=localhost", "-addext", "subjectAltName=DNS:localhost",
        "-addext", "basicConstraints=critical,CA:FALSE", "-addext", "extendedKeyUsage=serverAuth",
        "-keyout", `${directory}/${name}.key`, "-out", `${directory}/${name}.crt`], { stdout: "ignore", stderr: "pipe" });
      if (await child.exited) throw new Error(await new Response(child.stderr).text());
    }
    const child = Bun.spawn(["node", "-e", `
      const assert=require('node:assert/strict'), fs=require('node:fs'), http=require('node:http'), https=require('node:https');
      const {rpcProxy}=require(process.argv[1]); const dir=process.argv[2];
      (async()=>{
        const upstream=http.createServer((req,res)=>res.end('{}'));
        await new Promise(r=>upstream.listen(0,'127.0.0.1',r));
        let mode='healthy'; const events=[];
        const pair=name=>({key:fs.readFileSync(dir+'/'+name+'.key'),cert:fs.readFileSync(dir+'/'+name+'.crt')});
        const proxy=rpcProxy({upstream:'http://127.0.0.1:'+upstream.address().port,token:'a'.repeat(32),tls:{valid:pair('valid'),invalid:pair('invalid')},control:()=>({mode,until:Date.now()+60000}),observe:e=>events.push(e)});
        await new Promise(r=>proxy.listen(0,'127.0.0.1',r));
        const send=()=>new Promise((resolve,reject)=>{
          const req=https.request({hostname:'127.0.0.1',servername:'localhost',port:proxy.address().port,path:'/'+'a'.repeat(32),method:'POST',ca:pair('valid').cert,agent:false},res=>{res.resume();res.on('end',()=>resolve(res.statusCode));});
          req.on('error',reject);req.end(JSON.stringify({method:'eth_getLogs',id:1,params:[]}));
        });
        try {
          assert.equal(await send(),200);
          mode='tls-trust'; proxy.refreshTrust();
          await assert.rejects(send(),e=>['DEPTH_ZERO_SELF_SIGNED_CERT','CERT_SIGNATURE_FAILURE'].includes(e.code));
          for(let i=0;i<50&&!events.some(e=>e.mode==='tls-trust');i++)await new Promise(r=>setTimeout(r,10));
          assert.ok(events.some(e=>e.mode==='tls-trust'));
          assert.equal(events.filter(e=>e.mode==='healthy').length,1);
          mode='healthy';proxy.refreshTrust();assert.equal(await send(),200);
          assert.equal(events.filter(e=>e.mode==='healthy').length,2);
        } finally {proxy.closeAllConnections();proxy.close();upstream.closeAllConnections();upstream.close();}
      })().catch(e=>{console.error(e);process.exitCode=1});
    `, new URL("../../scripts/lib/host-rpc-proxy.cjs", import.meta.url).pathname, directory], { stdout: "pipe", stderr: "pipe", timeout: 10_000 });
    const [status, errors] = await Promise.all([child.exited, new Response(child.stderr).text()]);
    expect(errors).toBe("");
    expect(status).toBe(0);
  } finally { await rm(directory, { recursive: true, force: true }); }
}, 20_000);
