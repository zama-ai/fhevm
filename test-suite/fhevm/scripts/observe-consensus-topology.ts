import { STATE_FILE } from "../src/layout";
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
  for (const role of ["host-listener", "host-listener-poller", "host-listener-consumer"]) {
    const child = Bun.spawn(["docker", "inspect", `${prefix}-${role}`], { stdout: "pipe", stderr: "pipe", timeout: 10_000 });
    const [output, status] = await Promise.all([new Response(child.stdout).text(), child.exited]);
    if (status !== 0) throw new Error(`cannot inspect ${prefix}-${role}`);
    const [container] = JSON.parse(output);
    const check = role === "host-listener-consumer" ? assertConsumerRoute : assertListenerRoute;
    check(container.Config.Cmd, container.State.Running, index, topology.scenario);
  }
}
// Values are restricted above to known scenario identifiers and integer constants.
console.log(`CONSENSUS_SCENARIO=${topology.scenario}\nCONSENSUS_OPERATORS=${topology.count}\nCONSENSUS_THRESHOLD=${topology.threshold}`);
