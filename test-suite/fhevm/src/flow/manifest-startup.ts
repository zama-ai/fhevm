import { MANIFEST_INJECTION_PATH } from "../manifest-drift";
import type { State } from "../types";

export function delaysManifestDetectors(state: State): boolean {
  return state.scenario.kind === "coprocessor-consensus" && state.scenario.instances.some(instance =>
    instance.args["consensus-detector"]?.includes(`--dangerous-drift-injection=${MANIFEST_INJECTION_PATH}`));
}

/** Give the listeners a catch-up window before choosing the first manifest range. */
export async function startManifestRuntime(services: string[], operations: {
  start: (services: string[]) => Promise<void>;
  waitForListener: (service: string) => Promise<unknown>;
  sleep: (milliseconds: number) => Promise<unknown>;
}) {
  const detectors = services.filter(name => name.endsWith("-consensus-detector"));
  if (!detectors.length) {
    await operations.start(services);
    return;
  }
  const workers = services.filter(name => !detectors.includes(name));
  const listeners = workers.filter(name => /-host-listener(?:-poller|-consumer)?$/.test(name));
  if (!listeners.length) throw new Error("manifest scenario requires host listeners before consensus detectors");
  await operations.start(workers);
  await Promise.all(listeners.map(operations.waitForListener));
  console.log("[manifest-lifecycle] listeners running; waiting 15 seconds before starting consensus detectors");
  await operations.sleep(15_000);
  await operations.start(detectors);
}
