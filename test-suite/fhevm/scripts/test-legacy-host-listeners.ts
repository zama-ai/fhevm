import YAML from "yaml";
import { legacyHostListenerContainers, withLegacyHostListeners } from "../src/commands/legacy-host-listeners";
import { waitForPollerCatchup } from "../src/commands/poller-readiness";
import { dockerInspect, waitForContainer } from "../src/flow/readiness";
import { composeUp, multiChainComposeUp } from "../src/flow/runtime-compose";
import { loadMergedComposeDoc, type ComposeDoc } from "../src/generate/compose";
import { consumerOnlyHostListeners } from "../src/host-listener-mode";
import { COPROCESSOR_DB_CONTAINER, DEFAULT_POSTGRES_USER, composePath, coprocessorDatabaseName, hostChainRuntimes } from "../src/layout";
import { loadState } from "../src/state/state";
import { run, runStreaming } from "../src/utils/process";

const mode = process.argv[2];
if (mode !== "listener" && mode !== "poller") throw new Error("Expected listener or poller");
const state = await loadState();
if (!state || !consumerOnlyHostListeners(state.scenario)) {
  throw new Error("Start a consumer-only stack before running legacy fallback tests");
}
const components: Array<{ name: string; doc: ComposeDoc }> = [
  { name: "coprocessor", doc: await loadMergedComposeDoc("coprocessor") },
];
for (const chain of hostChainRuntimes(state.scenario.hostChains).filter((chain) => !chain.isDefault)) {
  components.push({ name: chain.copro, doc: YAML.parse(await Bun.file(composePath(chain.copro)).text()) });
}
await withLegacyHostListeners(components.map(({ doc }) => doc), mode, {
  running: async (name) => (await dockerInspect(name))[0]?.State.Status === "running",
  stop: async (names) => { await run(["docker", "stop", ...names]); },
  start: async (names) => {
    for (const { name, doc } of components) {
      const services = Object.entries(doc.services)
        .filter(([service, config]) => names.includes(String(config.container_name ?? service)))
        .map(([service]) => service);
      if (!services.length) continue;
      if (name === "coprocessor") await composeUp(name, services, { noDeps: true });
      else await multiChainComposeUp(name, services);
    }
    for (const name of names) await waitForContainer(name, "running");
  },
}, async () => {
  try {
    if (mode === "poller") {
      const targets = [];
      const databases = new Map<string, { database: string; chainId: string }>();
      for (const chain of hostChainRuntimes(state.scenario.hostChains)) {
        const response = await fetch(`http://127.0.0.1:${chain.rpcPort}`, {
          method: "POST",
          headers: { "content-type": "application/json" },
          body: JSON.stringify({ jsonrpc: "2.0", id: 1, method: "eth_blockNumber", params: [] }),
          signal: AbortSignal.timeout(10_000),
        });
        const head = await response.json() as { result?: string };
        if (!response.ok || !head.result || !/^0x[0-9a-f]+$/i.test(head.result)) {
          throw new Error(`Cannot read host head for poller readiness: ${chain.key}`);
        }
        // Freeze the target: following a moving head can keep readiness pending.
        for (let index = 0; index < state.scenario.topology.count; index++) {
          const prefix = index === 0 ? "coprocessor" : `coprocessor${index}`;
          const name = `${prefix}-host-listener-poller${chain.suffix}`;
          targets.push({ name, block: BigInt(head.result) });
          databases.set(name, { database: coprocessorDatabaseName(index), chainId: BigInt(chain.chainId).toString() });
        }
      }
      await waitForPollerCatchup(targets, {
        running: async (name) => (await dockerInspect(name))[0]?.State.Status === "running",
        progress: async (name) => {
          const { database, chainId } = databases.get(name)!;
          const result = await run([
            "docker", "exec", process.env.POSTGRES_CONTAINER ?? COPROCESSOR_DB_CONTAINER,
            "psql", "-U", process.env.POSTGRES_USER ?? DEFAULT_POSTGRES_USER, "-d", database,
            "-v", "ON_ERROR_STOP=1", "-tAc",
            `SELECT last_caught_up_block FROM host_listener_poller_state WHERE chain_id = ${chainId}`,
          ], { timeoutMs: 10_000 });
          const value = result.stdout.trim();
          return value ? BigInt(value) : null;
        },
        sleep: (ms) => Bun.sleep(ms),
        now: () => Date.now(),
      });
    }
    const profiles = mode === "listener"
      ? ["input-proof", "input-proof-compute-decrypt", "user-decryption", "delegated-user-decryption", "erc20", "public-decrypt-http-ebool", "public-decrypt-http-mixed", "multi-chain-isolation"]
      : ["erc20", "multi-chain-isolation"];
    for (const profile of profiles) await runStreaming(["./fhevm-cli", "test", profile]);
  } catch (error) {
    // Preserve the fallback logs before the finally block restores consumers.
    for (const name of legacyHostListenerContainers(components.map(({ doc }) => doc), mode).selected) {
      console.log(`::group::${name} before fallback cleanup`);
      await runStreaming(["docker", "logs", "--tail", "200", name], { allowFailure: true });
      console.log("::endgroup::");
    }
    throw error;
  }
});
