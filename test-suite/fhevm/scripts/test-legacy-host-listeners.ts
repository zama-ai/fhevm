import YAML from "yaml";
import { withLegacyHostListeners } from "../src/commands/legacy-host-listeners";
import { dockerInspect, waitForContainer } from "../src/flow/readiness";
import { composeUp, multiChainComposeUp } from "../src/flow/runtime-compose";
import { loadMergedComposeDoc, type ComposeDoc } from "../src/generate/compose";
import { consumerOnlyHostListeners } from "../src/host-listener-mode";
import { composePath, hostChainRuntimes } from "../src/layout";
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
  const profiles = mode === "listener"
    ? ["input-proof", "input-proof-compute-decrypt", "user-decryption", "delegated-user-decryption", "erc20", "public-decrypt-http-ebool", "public-decrypt-http-mixed", "multi-chain-isolation"]
    : ["erc20", "multi-chain-isolation"];
  for (const profile of profiles) await runStreaming(["./fhevm-cli", "test", profile]);
});
