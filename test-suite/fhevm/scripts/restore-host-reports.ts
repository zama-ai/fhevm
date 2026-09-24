#!/usr/bin/env bun
import { envPath } from "../src/layout";
import { readEnvFile } from "../src/utils/fs";
import { hostReportOriginals } from "../src/consensus/host-report-journal";

const file = process.argv[2];
if (!file) throw new Error("host report recovery journal required");
const originals = hostReportOriginals(await Bun.file(file).json());
const env = await readEnvFile(envPath("minio"));
if (!env.MINIO_ROOT_USER || !env.MINIO_ROOT_PASSWORD) throw new Error("managed MinIO credentials missing");
const alias = new URL("http://127.0.0.1:9000");
alias.username = env.MINIO_ROOT_USER; alias.password = env.MINIO_ROOT_PASSWORD;
for (const row of originals) {
  const key = `${row.bucket}/state_hash/chain=${row.chain}/block=${row.block}.bin`;
  const child = Bun.spawn(["docker", "exec", "-i", "-e", "MC_HOST_consensus", "fhevm-minio", "mc", "--quiet",
    "pipe", "--attr", `block-hash=${row.blockHash}`, `consensus/${key}`], {
    stdin: "pipe", stdout: "pipe", stderr: "pipe", timeout: 60_000,
    env: { ...process.env, MC_HOST_consensus: alias.href },
  });
  child.stdin.write(Buffer.from(row.original, "hex"));
  await child.stdin.end();
  const [status] = await Promise.all([child.exited, new Response(child.stdout).text(), new Response(child.stderr).text()]);
  if (status !== 0) throw new Error("host report restore failed; retain journal and discard stack until repaired");
  const response = await fetch(`http://127.0.0.1:9000/${key}`, { signal: AbortSignal.timeout(10_000) });
  if (!response.ok || response.headers.get("x-amz-meta-block-hash") !== row.blockHash ||
      Buffer.from(await response.arrayBuffer()).toString("hex") !== row.original) {
    throw new Error("restored host commitment or metadata differs");
  }
}
console.log(`Restored and read back ${originals.length} host commitments.`);
