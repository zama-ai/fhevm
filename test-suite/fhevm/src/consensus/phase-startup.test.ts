import {expect, test} from "bun:test";
import {randomUUID} from "node:crypto";
import {existsSync, mkdtempSync, readFileSync, rmSync, writeFileSync} from "node:fs";
import path from "node:path";
const source = readFileSync(path.resolve(import.meta.dir, "../../scripts/lib/container-phase.cjs"), "utf8");
const live = (pid: number) => {
  try { return !["Z", "X"].includes(readFileSync(`/proc/${pid}/stat`, "utf8").split(") ")[1].split(" ")[0]); }
  catch { return false; }
};
const until = async (predicate: () => boolean) => {
  const deadline = Date.now() + 4000;
  while (!predicate()) { if (Date.now() > deadline) throw new Error("startup boundary not reached"); await Bun.sleep(10); }
};
// Stop the actual supervisor at filesystem boundaries without rewriting its
// implementation. Container PID namespaces have no group0 kernel threads to
// accidentally mask a cancellation that ignores the still-live supervisor.
for (const boundary of ["before-check", "after-check", "after-spawn"] as const) {
  test.skipIf(process.platform !== "linux")(`cancellation fences ${boundary} startup and atomic group publication`, async () => {
    const dir = mkdtempSync("/tmp/phase-startup-");
    const token = `startup_${randomUUID()}`;
    const record = `/tmp/fhevm-consensus-phases/${token}.json`;
    const preload = path.join(dir, "preload.cjs");
    writeFileSync(preload, `
      const fs=require('node:fs'), exists=fs.existsSync, rename=fs.renameSync, write=fs.writeFileSync;
      const pause=()=>{write(process.env.REVIEW_DIR+'/paused','1');process.kill(process.pid,'SIGSTOP');};
      fs.existsSync=function(file){const result=exists.apply(this,arguments);
        if(process.env.REVIEW_BOUNDARY==='after-check'&&String(file).endsWith(process.env.REVIEW_TOKEN+'.cancelled')&&!result)pause();
        return result;
      };
      fs.renameSync=function(from,to){
        if(String(to).endsWith(process.env.REVIEW_TOKEN+'.json')){
          const record=JSON.parse(fs.readFileSync(from,'utf8'));
          if(record.group>0)write(process.env.REVIEW_DIR+'/group',String(record.group));
          if((process.env.REVIEW_BOUNDARY==='before-check'&&record.group===0)||
             (process.env.REVIEW_BOUNDARY==='after-spawn'&&record.group>0))pause();
        }
        return rename.apply(this,arguments);
      };
    `);
    const child = Bun.spawn(["node", "-r", preload, "-e", source, "run", token, String(Date.now()+30000), "node", "-e",
      `require('fs').writeFileSync(${JSON.stringify(path.join(dir,"work"))},String(process.pid));process.on('SIGTERM',()=>{});setInterval(()=>{},1000);`], {
      env:{...process.env, REVIEW_DIR:dir, REVIEW_TOKEN:token, REVIEW_BOUNDARY:boundary}, stdout:"ignore", stderr:"ignore",
    });
    const cancel = () => Bun.spawn(["node", "-e", source, "cancel", token], {stdout:"ignore", stderr:"ignore"});
    try {
      await until(()=>existsSync(path.join(dir,"paused")));
      if (boundary === "before-check") {
        expect(existsSync(record)).toBe(false);
        expect(await cancel().exited).toBe(0);
        child.kill("SIGCONT");
        expect(await child.exited).toBe(143);
      } else {
        expect(JSON.parse(readFileSync(record,"utf8")).group).toBe(0);
        if (boundary === "after-spawn") {
          await until(()=>existsSync(path.join(dir,"work")));
          child.kill("SIGKILL");
          await child.exited;
        }
        const cancellation=cancel();
        if (boundary === "after-check") {
          await Bun.sleep(150);
          expect(cancellation.exitCode).toBeNull();
        }
        expect(await cancellation.exited).toBe(0);
        expect(live(child.pid)).toBe(false);
        if (boundary === "after-spawn") expect(live(Number(readFileSync(path.join(dir,"work"),"utf8")))).toBe(false);
      }
      if (boundary !== "after-spawn") expect(existsSync(path.join(dir,"work"))).toBe(false);
    } finally {
      child.kill("SIGKILL");
      await cancel().exited;
      if (existsSync(path.join(dir,"group"))) {
        try { process.kill(-Number(readFileSync(path.join(dir,"group"),"utf8")), "SIGKILL"); } catch {}
      }
      for (const suffix of ["json","cancelled"]) rmSync(`/tmp/fhevm-consensus-phases/${token}.${suffix}`,{force:true});
      rmSync(dir,{recursive:true,force:true});
    }
  }, 10000);
}
