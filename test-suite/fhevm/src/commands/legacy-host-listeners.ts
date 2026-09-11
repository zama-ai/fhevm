import { PreflightError } from "../errors";
import type { ComposeDoc } from "../generate/compose";
import type { ContainerControl } from "./consumer-only-recovery";

export type LegacyHostListenerMode = "listener" | "poller";

/** Require a fallback for every consumer, including secondary host chains. */
export const legacyHostListenerContainers = (docs: ComposeDoc[], mode: LegacyHostListenerMode) => {
  const consumers: string[] = [];
  const listeners: string[] = [];
  const pollers: string[] = [];
  for (const doc of docs) {
    for (const [service, config] of Object.entries(doc.services)) {
      const executable = Array.isArray(config.command) ? config.command[0] : undefined;
      const name = typeof config.container_name === "string" ? config.container_name : service;
      if (executable === "host_listener_consumer") consumers.push(name);
      if (executable === "host_listener") listeners.push(name);
      if (executable === "host_listener_poller") pollers.push(name);
    }
  }
  if (!consumers.length) throw new PreflightError("Legacy fallback tests require a consumer stack");
  for (const consumer of consumers) {
    const listener = consumer.replace("host-listener-consumer", "host-listener");
    if (!listeners.includes(listener) || !pollers.includes(listener.replace("host-listener", "host-listener-poller"))) {
      throw new PreflightError(`Missing legacy fallback for ${consumer}`);
    }
  }
  const selected = mode === "listener" ? listeners : pollers;
  if (selected.length !== consumers.length) {
    throw new PreflightError("Legacy fallback topology does not match the consumer topology");
  }
  return { consumers, selected, legacy: [...listeners, ...pollers] };
};

/** Test one fallback exclusively, then restore consumer ingestion even on failure. */
export const withLegacyHostListeners = async (
  docs: ComposeDoc[],
  mode: LegacyHostListenerMode,
  control: ContainerControl,
  test: () => Promise<void>,
) => {
  const { consumers, selected, legacy } = legacyHostListenerContainers(docs, mode);
  const assertRunning = async (names: string[], expected: boolean) => {
    for (const name of names) {
      if (await control.running(name) !== expected) {
        throw new PreflightError(`Unexpected ${expected ? "stopped" : "running"} host ingestion: ${name}`);
      }
    }
  };
  await assertRunning(consumers, true);
  await assertRunning(legacy, false);
  const stopRunning = async (names: string[]) => {
    const running = [];
    for (const name of names) if (await control.running(name)) running.push(name);
    if (running.length) await control.stop(running);
  };
  try {
    await control.stop(consumers);
    await assertRunning(consumers, false);
    await control.start(selected);
    const assertFallback = async () => {
      await assertRunning(selected, true);
      await assertRunning([...consumers, ...legacy.filter((name) => !selected.includes(name))], false);
    };
    await assertFallback();
    console.log(`[legacy-${mode}] Exclusive host ingestion: ${selected.join(", ")}`);
    await test();
    await assertFallback();
  } finally {
    // Do not restart consumers if a fallback cannot be stopped: fail visibly
    // instead of allowing two ingestion paths to hide a broken fallback test.
    await stopRunning(legacy);
    await assertRunning(legacy, false);
    await control.start(consumers);
    await assertRunning(consumers, true);
    console.log(`[legacy-${mode}] Consumer-only ingestion restored`);
  }
};
