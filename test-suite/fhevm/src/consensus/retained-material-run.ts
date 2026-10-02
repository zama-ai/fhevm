import path from "node:path";
import { chmod, mkdir } from "node:fs/promises";
import { envPath, STATE_DIR, TEST_SUITE_CONTAINER } from "../layout";
import type { State } from "../types";
import { readEnvFile } from "../utils/fs";
import { run, runWithHeartbeat } from "../utils/process";

/** Host-side snapshots survive replacement of the E2E container during rollout. */
export async function runRetainedMaterial(state: State, phase: "seed" | "verify" | "retired-sns" | "retired-tfhe" | "retired-check" | "retired-recover", label: string, gpu = false): Promise<void> {
  if (!/^[a-z0-9-]+$/.test(label)) throw new Error("invalid retained fixture label");
  const env = await readEnvFile(envPath("test-suite"));
  const copro = await readEnvFile(envPath("coprocessor"));
  const root = path.join(STATE_DIR, "runtime", "retained-material");
  await mkdir(root, { recursive: true, mode: 0o700 });
  for (const [index, chain] of state.scenario.hostChains.entries()) {
    const remote = `/tmp/retained-${label}-${chain.chainId}.json`;
    const snapshot = path.join(root, `${label}-${chain.chainId}.json`);
    if (phase !== "seed") {
      await run(["docker", "exec", "-i", TEST_SUITE_CONTAINER, "sh", "-c", 'umask 077; cat > "$1"', "sh", remote],
        { input: await Bun.file(snapshot).text() });
    } else if (await Bun.file(snapshot).exists()) throw new Error(`refusing to replace retained baseline ${snapshot}`);
    const required = (key: string, values = env) => {
      if (!values[key]) throw new Error(`retained fixture requires ${key}`);
      return values[key]!;
    };
    const overrides: Record<string, string> = {
      RETAINED_REQUIRE_GPU: gpu ? "1" : "0", RETAINED_PHASE: phase, RETAINED_STATE_FILE: remote,
      COPROCESSOR_COUNT: String(state.scenario.topology.count),
      GATEWAY_RPC_URL: required("GATEWAY_URL", copro),
      GATEWAY_CONFIG_ADDRESS: required("GATEWAY_CONFIG_ADDRESS", copro),
    };
    for (const [name, suffix] of [
      ["RPC_URL", "RPC_URL"], ["CHAIN_ID_HOST", "CHAIN_ID"],
      ["ACL_CONTRACT_ADDRESS", "ACL_CONTRACT_ADDRESS"],
      ["KMS_VERIFIER_CONTRACT_ADDRESS", "KMS_VERIFIER_CONTRACT_ADDRESS"],
      ["INPUT_VERIFIER_CONTRACT_ADDRESS", "INPUT_VERIFIER_CONTRACT_ADDRESS"],
      ["FHEVM_EXECUTOR_CONTRACT_ADDRESS", "FHEVM_EXECUTOR_CONTRACT_ADDRESS"],
      ["PROTOCOL_CONFIG_CONTRACT_ADDRESS", "PROTOCOL_CONFIG_CONTRACT_ADDRESS"],
    ]) overrides[name!] = required(index === 0 ? name! : `HOST_CHAIN_${index}_${suffix}`);
    if (["retired-check", "retired-recover"].includes(phase)) {
      for (const lane of ["sns", "tfhe"]) await run(["docker", "exec", "-i", TEST_SUITE_CONTAINER, "sh", "-c", 'cat > "$1"', "sh", `${remote}.retired-${lane}.json`],
        { input: await Bun.file(`${snapshot}.retired-${lane}.json`).text() });
    }
    await runWithHeartbeat(["docker", "exec", ...Object.entries(overrides).flatMap(([name, value]) => ["-e", `${name}=${value}`]),
      TEST_SUITE_CONTAINER, "npx", "hardhat", "test", "test/consensus/retainedMaterial.ts", "--no-compile", "--network", "staging"],
    `retained material ${phase} on ${chain.key}`);
    if (["retired-sns", "retired-tfhe"].includes(phase)) {
      const content = (await run(["docker", "exec", TEST_SUITE_CONTAINER, "cat", `${remote}.${phase}.json`])).stdout;
      await Bun.write(`${snapshot}.${phase}.json`, content);
    }
    if (phase === "seed") {
      const content = (await run(["docker", "exec", TEST_SUITE_CONTAINER, "cat", remote])).stdout;
      const parsed = JSON.parse(content);
      if (parsed.chain !== chain.chainId || !parsed.seedTransaction) throw new Error("retained fixture returned wrong chain or no receipt");
      await Bun.write(snapshot, content);
      await chmod(snapshot, 0o600);
    } else if (phase === "verify") {
      // Preserve the original immutable baseline and all later consumption receipts.
      const receipts = (await run(["docker", "exec", TEST_SUITE_CONTAINER, "sh", "-c", 'for receipt in "$1".verified-*.json; do cat "$receipt" || exit; printf "\\n"; done', "sh", remote])).stdout;
      await Bun.write(path.join(root, `${label}-${chain.chainId}-verified-${Date.now()}.jsonl`), receipts);
    }
  }
}
