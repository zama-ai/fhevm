import { BuildError, ContainerStartError, PreflightError } from "../errors";
import {
  GROUP_BUILD_COMPONENTS,
  GROUP_BUILD_SERVICES,
  PROJECT,
  composePath,
  dockerArgs,
} from "../layout";
import { loadMergedComposeDoc, resolvedComposeEnv, serviceNameList, type ComposeDoc } from "../generate/compose";
import { stackSpecForState } from "../stack-spec/stack-spec";
import { saveState } from "../state/state";
import type { BuiltImage, State, StepName } from "../types";
import { exists, remove, readEnvFileIfExists } from "../utils/fs";
import { composeEnv, run, runStreaming } from "../utils/process";
import { multiChainComposeEntries } from "./artifacts";

/** Logs elapsed time for one stack subtask. */
const timed = async <T>(label: string, task: () => Promise<T>) => {
  const started = Date.now();
  const result = await task();
  console.log(`${label} done (${Math.round((Date.now() - started) / 1000)}s)`);
  return result;
};

/** Lists containers belonging to the current compose project. */
export const projectContainers = async (all = false) => {
  const ps = await run(
    ["docker", "ps", ...(all ? ["-a"] : []), "--filter", `label=com.docker.compose.project=${PROJECT}`, "--format", "{{.Names}}"],
    { allowFailure: true },
  );
  if (ps.code !== 0) {
    throw new PreflightError(ps.stderr.trim() || "docker ps failed");
  }
  return ps.stdout.split(/\r?\n/).map((line) => line.trim()).filter(Boolean);
};

/** Collects image refs for selected services from a compose document. */
const imageRefsFromDoc = (doc: ComposeDoc, services: string[]) => {
  const selected = services.length ? services : Object.keys(doc.services);
  return [
    ...new Set(
      selected
        .map((name) => doc.services[name]?.image)
        .filter((value): value is string => typeof value === "string" && value.length > 0),
    ),
  ];
};

/** Extracts the coprocessor instance index encoded in a service name. */
const coprocessorInstanceIndex = (service: string) => {
  const match = /^coprocessor(?:(\d+))?-/.exec(service);
  if (!match) {
    return undefined;
  }
  return match[1] ? Number(match[1]) : 0;
};

/** Reads the immutable image id for a local image reference. */
export const inspectImageId = async (ref: string) => {
  const result = await run(["docker", "image", "inspect", ref, "--format", "{{.Id}}"], { allowFailure: true });
  return result.code === 0 ? result.stdout.trim() : "";
};

/** Persists the current set of successfully built local images. */
const saveBuiltImages = async (
  state: State,
  refs: Array<{ ref: string; group: BuiltImage["group"]; instanceIndex?: number }>,
) => {
  const current = new Map((state.builtImages ?? []).map((item) => [item.ref, item] as const));
  for (const entry of refs) {
    const id = await inspectImageId(entry.ref);
    if (!id) {
      continue;
    }
    current.set(entry.ref, {
      ref: entry.ref,
      id,
      group: entry.group,
      instanceIndex: entry.instanceIndex,
    });
  }
  state.builtImages = [...current.values()].sort((a, b) => a.ref.localeCompare(b.ref));
  await saveState(state);
};

/** Checks whether a set of image refs still matches the last recorded local build ids. */
const refsAlreadyBuilt = async (state: State, refs: string[]) =>
  (await Promise.all(
    refs.map(async (ref) => {
      const id = await inspectImageId(ref);
      return !!id && (state.builtImages ?? []).some((image) => image.ref === ref && image.id === id);
    }),
  )).every(Boolean);

/** Starts one compose component, optionally limiting it to selected services. */
export const composeUp = async (
  component: string,
  services: string[] = [],
  options: { noDeps?: boolean; env?: Record<string, string> } = {},
) => {
  try {
    await runStreaming(
      [...dockerArgs(component), "up", "-d", ...(options.noDeps ? ["--no-deps"] : []), ...services],
      { env: await composeEnv(component, options.env) },
    );
  } catch (error) {
    throw new ContainerStartError(component, error instanceof Error ? error.message : String(error));
  }
};

/** Stops one compose component and returns whether teardown succeeded cleanly. */
export const composeDown = async (component: string) => {
  try {
    const code = await runStreaming([...dockerArgs(component), "down", "-v"], {
      env: await composeEnv(component),
      allowFailure: true,
    });
    if (code !== 0) {
      console.log(`[warn] compose down failed for ${component} (${code})`);
      return false;
    }
    return true;
  } catch (error) {
    console.log(`[warn] compose down failed for ${component}: ${error instanceof Error ? error.message : String(error)}`);
    return false;
  }
};

/** Removes lingering compose-owned containers, volumes, or networks. */
export const removeProjectResources = async (kind: "container" | "volume" | "network", format: string) => {
  const listed = await run(
    ["docker", kind, "ls", ...(kind === "container" ? ["-a"] : []), "--filter", `label=com.docker.compose.project=${PROJECT}`, "--format", format],
    { allowFailure: true },
  );
  if (listed.code !== 0) {
    throw new PreflightError((listed.stderr || listed.stdout).trim() || `docker ${kind} ls failed`);
  }
  const names = listed.stdout.split(/\r?\n/).map((line) => line.trim()).filter(Boolean);
  if (!names.length) {
    return;
  }
  const removed = await run(
    kind === "container"
      ? ["docker", "rm", "-fv", ...names]
      : kind === "volume"
        ? ["docker", "volume", "rm", "-f", ...names]
        : ["docker", "network", "rm", ...names],
    { allowFailure: true },
  );
  if (removed.code !== 0) {
    throw new PreflightError((removed.stderr || removed.stdout).trim() || `docker ${kind} rm failed`);
  }
};

/** Builds one compose component for the selected services. */
const composeBuild = async (component: string, services: string[], env?: Record<string, string>) => {
  try {
    await runStreaming([...dockerArgs(component), "build", ...services], {
      env: await composeEnv(component, env),
    });
  } catch (error) {
    throw new ContainerStartError(component, error instanceof Error ? error.message : String(error));
  }
};

/** Builds any locally overridden images required before a compose-up step. */
export const maybeBuild = async (component: string, state: State, options: { force?: boolean } = {}) => {
  try {
    if (component === "coprocessor") {
      const doc = await loadMergedComposeDoc(component);
      const services = Object.entries(doc.services)
        .filter(([, service]) => !!service.build)
        .map(([name]) => name);
      if (!services.length) {
        return;
      }
      const refs = imageRefsFromDoc(doc, services);
      if (!options.force && (await refsAlreadyBuilt(state, refs))) {
        return;
      }
      console.log("[build] coprocessor");
      for (const ref of refs) {
        await run(["docker", "image", "rm", "-f", ref], { allowFailure: true });
      }
      for (const service of services) {
        await timed(`[build] ${service}`, () => composeBuild(component, [service]));
      }
      await saveBuiltImages(
        state,
        refs.map((ref) => ({
          ref,
          group: "coprocessor" as const,
          instanceIndex: coprocessorInstanceIndex(
            services.find((service) => doc.services[service]?.image === ref) ?? "",
          ),
        })),
      );
      return;
    }

    for (const override of state.overrides) {
      if (!GROUP_BUILD_COMPONENTS[override.group].includes(component)) {
        continue;
      }
      const doc = await loadMergedComposeDoc(component);
      const available = new Set(Object.keys(doc.services));
      const candidates = override.services?.length ? override.services : GROUP_BUILD_SERVICES[override.group];
      const services = candidates.filter((service) => available.has(service));
      if (!services.length) {
        continue;
      }
      console.log(`[build] ${override.group} (${component})`);
      const refs = imageRefsFromDoc(doc, services);
      if (!options.force && (await refsAlreadyBuilt(state, refs))) {
        continue;
      }
      for (const ref of refs) {
        await run(["docker", "image", "rm", "-f", ref], { allowFailure: true });
      }
      const seen = new Set<string>();
      const deduped = services.filter((service) => {
        const image = doc.services[service]?.image;
        if (typeof image !== "string" || seen.has(image)) {
          return false;
        }
        seen.add(image);
        return true;
      });
      const buildBatches = override.group === "coprocessor" ? deduped.map((service) => [service]) : [deduped];
      for (const batch of buildBatches) {
        await timed(`[build] ${batch.join(",")}`, () => composeBuild(component, batch));
      }
      await saveBuiltImages(
        state,
        imageRefsFromDoc(doc, services).map((ref) => ({ ref, group: override.group })),
      );
    }
  } catch (error) {
    if (error instanceof ContainerStartError) {
      throw new BuildError(error.component, error.stderr);
    }
    throw new BuildError(component, error instanceof Error ? error.message : String(error));
  }
};

/** Builds then starts a compose component as one pipeline step. */
export const stepComposeUp = async (
  component: string,
  state: State,
  services?: string[],
  options?: { noDeps?: boolean; env?: Record<string, string> },
) => {
  await maybeBuild(component, state);
  await composeUp(component, services, options);
};

/** Maps a multi-chain compose name to the component whose env it needs. */
const multiChainEnvComponent = (name: string) =>
  name.startsWith("coprocessor-") ? "coprocessor" : name;

/** Starts one generated multi-chain compose file. */
export const multiChainComposeUp = async (
  name: string,
  services?: string[],
) => {
  const file = composePath(name);
  const component = multiChainEnvComponent(name);
  try {
    await runStreaming(
      ["docker", "compose", "-p", PROJECT, "-f", file, "up", "-d", "--no-deps", ...(services ?? [])],
      { env: await composeEnv(component) },
    );
  } catch (error) {
    throw new ContainerStartError(name, error instanceof Error ? error.message : String(error));
  }
};

/** Stops one generated multi-chain compose file. */
export const multiChainComposeDown = async (name: string) => {
  const file = composePath(name);
  const component = multiChainEnvComponent(name);
  try {
    const code = await runStreaming(
      ["docker", "compose", "-p", PROJECT, "-f", file, "down", "-v"],
      { env: await composeEnv(component).catch(() => ({ COMPOSE_IGNORE_ORPHANS: "true" })), allowFailure: true },
    );
    if (code !== 0) {
      console.log(`[warn] compose down failed for ${name} (${code})`);
      return false;
    }
    return true;
  } catch (error) {
    console.log(`[warn] compose down failed for ${name}: ${error instanceof Error ? error.message : String(error)}`);
    return false;
  }
};

/** Resets runtime state by stopping components from the requested step onward. */
export const resetAfterStep = async (
  step: StepName,
  stepNames: readonly StepName[],
  componentByStep: Record<StepName, string[]>,
  stateStepIndex: (step: StepName) => number,
  state: Pick<State, "scenario">,
) => {
  const start = stateStepIndex(step);
  const failed: string[] = [];
  const toTearDown = multiChainComposeEntries(state)
    .filter(([, parentStep]) => stateStepIndex(parentStep) >= start)
    .map(([name]) => name);
  if (toTearDown.length) {
    const results = await Promise.all(
      toTearDown.map(async (name) => ({ name, ok: await multiChainComposeDown(name) })),
    );
    for (const { name, ok } of results) {
      if (!ok) failed.push(name);
    }
  }
  for (let index = stepNames.length - 1; index >= start; index -= 1) {
    for (const component of componentByStep[stepNames[index]]) {
      const ok = await composeDown(component);
      if (!ok) {
        failed.push(component);
      }
    }
  }
  if (failed.length) {
    throw new PreflightError(`Failed to stop components while resetting from ${step}: ${failed.join(", ")}`);
  }
};
