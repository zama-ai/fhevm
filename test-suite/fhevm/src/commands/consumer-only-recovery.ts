import { PreflightError } from "../errors";
import type { ComposeDoc } from "../generate/compose";

export type ContainerControl = {
  running: (name: string) => Promise<boolean>;
  stop: (names: string[]) => Promise<void>;
  start: (names: string[]) => Promise<void>;
};

/** Select from generated Compose, including every operator and host chain. */
export const recoveryContainers = (doc: ComposeDoc, extraChains: ComposeDoc[] = []) => {
  const legacy: string[] = [];
  const consumers: string[] = [];
  for (const [service, config] of Object.entries(doc.services)) {
    const executable = Array.isArray(config.command) ? config.command[0] : undefined;
    const name = typeof config.container_name === "string" ? config.container_name : service;
    if (executable === "host_listener" || executable === "host_listener_poller") legacy.push(name);
    if (executable === "host_listener_consumer") consumers.push(name);
  }
  if (!consumers.length) {
    throw new PreflightError("Consumer-only drift requires consumer services in the generated stack");
  }
  for (const name of legacy) {
    const consumer = name.replace(/host-listener(?:-poller)?/, "host-listener-consumer");
    if (!consumers.includes(consumer)) {
      throw new PreflightError(`Missing host consumer for ${name}`);
    }
  }
  // Drift traffic targets the default host chain. Extra chains currently have
  // legacy ingestion only; stop that too, without requiring new topology here.
  for (const extra of extraChains) {
    for (const [service, config] of Object.entries(extra.services)) {
      const executable = Array.isArray(config.command) ? config.command[0] : undefined;
      const name = typeof config.container_name === "string" ? config.container_name : service;
      if (executable === "host_listener" || executable === "host_listener_poller") legacy.push(name);
      if (executable === "host_listener_consumer") consumers.push(name);
    }
  }
  return { legacy: [...new Set(legacy)], consumers: [...new Set(consumers)] };
};

/** Restore only services that were running before the test, including on failure. */
export const withConsumerOnlyRecovery = async (
  doc: ComposeDoc,
  control: ContainerControl,
  test: () => Promise<void>,
  extraChains: ComposeDoc[] = [],
) => {
  const { legacy, consumers } = recoveryContainers(doc, extraChains);
  for (const name of consumers) {
    if (!(await control.running(name))) throw new PreflightError(`Host consumer is not running: ${name}`);
  }
  console.log(`[drift-consumer-recovery] Active host consumers: ${consumers.join(", ")}`);
  const restore: string[] = [];
  for (const name of legacy) if (await control.running(name)) restore.push(name);
  try {
    if (restore.length) {
      console.log(`[drift-consumer-recovery] Stopping legacy ingestion: ${restore.join(", ")}`);
      await control.stop(restore);
    }
    for (const name of legacy) {
      if (await control.running(name)) throw new PreflightError(`Legacy ingestion is still running: ${name}`);
    }
    console.log("[drift-consumer-recovery] All legacy listeners and pollers are stopped; running drift recovery with host consumers");
    await test();
  } finally {
    if (restore.length) {
      console.log(`[drift-consumer-recovery] Restoring legacy ingestion: ${restore.join(", ")}`);
      await control.start(restore);
      console.log("[drift-consumer-recovery] Previously running legacy services restarted");
    }
  }
};
