import path from "node:path";
import { createHash } from "node:crypto";
import { mkdir, mkdtemp, rm } from "node:fs/promises";
import { run } from "../../src/utils/process";

/** Released 0.14 binaries use STACK_VERSION as their protocol identity. */
export async function probeSoftwareCandidate(stateDir: string, targetTag: string, count: number) {
  if (!/^v[0-9A-Za-z.+-]+$/.test(targetTag)) throw new Error("an explicit release tag is required");
  const root = path.join(stateDir, "rollout", "software-candidates");
  await mkdir(root, { recursive: true });
  const evidence: { container: string; before: string; after: string; protocol: string; beforeHash: string; afterHash: string }[] = [];
  for (let operator = 0; operator < count; operator++) {
    for (const role of ["host-listener", "tfhe-worker", "zkproof-worker", "sns-worker", "transaction-sender"]) {
      const container = `coprocessor${operator || ""}-${role}`;
      const before = JSON.parse((await run(["docker", "inspect", container])).stdout)[0];
      const source = before.Config.Image as string;
      if (!source.includes(":") || source.includes("@")) throw new Error("software baseline must use an explicit release tag");
      const target = `${source.slice(0, source.lastIndexOf(":"))}:${targetTag}`;
      const binary = `/usr/local/bin/${role.replaceAll("-", "_")}`;
      const originalProtocol = (await run(["docker", "exec", container, binary, "--stack-version"])).stdout.trim();
      await run(["docker", "pull", target]);
      const after = JSON.parse((await run(["docker", "image", "inspect", target])).stdout)[0].Id as string;
      const candidateProtocol = (await run(["docker", "run", "--rm", "--network", "none", after, binary, "--stack-version"])).stdout.trim();
      if (!/^v?0\.14\.0$/.test(originalProtocol) || candidateProtocol !== originalProtocol) {
        throw new Error(`${role} is not a compatible software-only transition (${originalProtocol} -> ${candidateProtocol})`);
      }
      const directory = await mkdtemp(path.join(root, "binary-"));
      let candidate = "";
      try {
        candidate = (await run(["docker", "create", "--network", "none", after, binary, "--stack-version"])).stdout.trim();
        await run(["docker", "cp", `${container}:${binary}`, path.join(directory, "before")]);
        await run(["docker", "cp", `${candidate}:${binary}`, path.join(directory, "after")]);
        const hash = async (name: string) => createHash("sha256").update(await Bun.file(path.join(directory, name)).arrayBuffer().then(value => Buffer.from(value))).digest("hex");
        evidence.push({ container, before: before.Image, after, protocol: originalProtocol, beforeHash: await hash("before"), afterHash: await hash("after") });
      } finally {
        if (candidate) await run(["docker", "rm", candidate]);
        await rm(directory, { recursive: true });
      }
    }
  }
  if (!evidence.some(item => item.beforeHash !== item.afterHash)) throw new Error("software-only campaign selected identical binaries");
  await Bun.write(path.join(root, "preflight.json"), JSON.stringify(evidence, null, 2));
  return evidence;
}

export async function assertSoftwareCandidateRunning(evidence: Awaited<ReturnType<typeof probeSoftwareCandidate>>) {
  for (const item of evidence) {
    const row = JSON.parse((await run(["docker", "inspect", item.container])).stdout)[0];
    if (!row.State.Running || row.Image !== item.after) throw new Error(`${item.container} did not adopt its observed candidate image`);
  }
}
