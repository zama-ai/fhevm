import { STATE_FILE, hostChainRuntimes } from "../src/layout";
import type { State } from "../src/types";
import { consensusTopology, assertListenerRoute, assertConsumerRoute } from "../src/consensus/topology";

const state = await Bun.file(STATE_FILE).json() as State;
const topology = consensusTopology(state, {
  scenario: process.env.CONSENSUS_SCENARIO,
  operators: process.argv[2] ?? process.env.CONSENSUS_OPERATORS,
  threshold: process.env.CONSENSUS_THRESHOLD,
});
if (topology.scenario === "three-of-three-fork" && process.env.FORK_OPERATOR_INDEX !== undefined && process.env.FORK_OPERATOR_INDEX !== "2") {
  throw new Error("managed fork topology routes operator 2 to the fork");
}
for (let index = 0; index < topology.count; index++) {
  const prefix = index === 0 ? "coprocessor" : `coprocessor${index}`;
  const chains = hostChainRuntimes(state.scenario.hostChains);
  for (const chain of chains) for (const role of ["host-listener", "host-listener-poller", ...(chain.isDefault ? ["host-listener-consumer"] : [])]) {
    const containerName = `${prefix}-${role}${chain.suffix}`;
    const child = Bun.spawn(["docker", "inspect", containerName], { stdout: "pipe", stderr: "pipe", timeout: 10_000 });
    const [output, status] = await Promise.all([new Response(child.stdout).text(), child.exited]);
    if (status !== 0) throw new Error(`cannot inspect ${containerName}`);
    const [container] = JSON.parse(output);
    if (role === "host-listener-consumer") assertConsumerRoute(container.Config.Cmd, container.State.Running, index, topology.scenario);
    else assertListenerRoute(container.Config.Cmd, container.State.Running, index, topology.scenario, chain.node);
  }
}
// Reuse the in-container E2E oracle, with that package's ethers dependency.
// This happens before any runner pauses a service or changes database state.
if (!state.discovery) throw new Error("active stack has no discovered Gateway");
const gateway = state.discovery.gateway.GATEWAY_CONFIG_ADDRESS;
if (!gateway) throw new Error("active stack has no GatewayConfig address");
const probe = Bun.spawn(["docker", "exec", "-e", "TS_NODE_TRANSPILE_ONLY=true",
  process.env.TEST_CONTAINER ?? "fhevm-test-suite-e2e-debug", "node", "-r", "ts-node/register", "-e",
  `const {assertGatewayTopology}=require('./test/consensus/helpers.ts');
   assertGatewayTopology(...JSON.parse(process.argv[1])).then(()=>process.exit(0), error=>{console.error(error);process.exit(1)});`,
  JSON.stringify([state.discovery.endpoints.gateway.http, gateway, topology.count, topology.threshold])],
  { stdout: "pipe", stderr: "pipe", timeout: 30_000 });
const [error, status] = await Promise.all([new Response(probe.stderr).text(), probe.exited]);
if (status !== 0) throw new Error(`cannot verify live Gateway topology: ${error}`);
// Values are restricted above to known scenario identifiers and integer constants.
console.log(`CONSENSUS_SCENARIO=${topology.scenario}\nCONSENSUS_OPERATORS=${topology.count}\nCONSENSUS_THRESHOLD=${topology.threshold}`);
