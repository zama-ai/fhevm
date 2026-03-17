import { Context, Effect, Layer } from "effect";
import fs from "node:fs/promises";
import YAML from "yaml";
import { CommandRunner } from "./CommandRunner";
import { ContainerRunner } from "./ContainerRunner";
import { BuildError, ContainerStartError } from "../errors";
import type { BuiltImage, State } from "../types";
import { GROUP_BUILD_COMPONENTS, GROUP_BUILD_SERVICES, effectiveComposePath } from "../layout";

type ComposeDoc = Record<string, unknown> & {
  services: Record<string, Record<string, unknown>>;
};

export class ImageBuilder extends Context.Tag("ImageBuilder")<
  ImageBuilder,
  {
    readonly maybeBuild: (
      component: string,
      state: State,
      saveState: (s: State) => Effect.Effect<void>,
    ) => Effect.Effect<void, BuildError>;
    readonly inspectImageId: (ref: string) => Effect.Effect<string>;
    readonly removeImage: (ref: string) => Effect.Effect<void>;
  }
>() {
  static Live = Layer.effect(
    ImageBuilder,
    Effect.gen(function* () {
      const cmd = yield* CommandRunner;
      const containers = yield* ContainerRunner;

      const imageRefsFromDoc = (doc: ComposeDoc, services: string[]): string[] => {
        const selected = services.length ? services : Object.keys(doc.services);
        return [
          ...new Set(
            selected
              .map((name) => doc.services[name]?.image)
              .filter(
                (value): value is string => typeof value === "string" && value.length > 0,
              ),
          ),
        ];
      };

      const coprocessorInstanceIndex = (service: string) => {
        const match = /^coprocessor(?:(\d+))?-/.exec(service);
        if (!match) {
          return undefined;
        }
        return match[1] ? Number(match[1]) : 0;
      };

      const saveBuiltImages = (
        state: State,
        saveState: (state: State) => Effect.Effect<void>,
        refs: Array<{ ref: string; group: BuiltImage["group"]; instanceIndex?: number }>,
      ) =>
        Effect.gen(function* () {
          const current = new Map(
            (state.builtImages ?? []).map((item) => [item.ref, item] as const),
          );
          for (const entry of refs) {
            const id = yield* inspectId(entry.ref);
            if (!id) {
              continue;
            }
            current.set(entry.ref, {
              ref: entry.ref,
              id,
              group: entry.group,
              instanceIndex: entry.instanceIndex,
            } satisfies BuiltImage);
          }
          state.builtImages = [...current.values()].sort((a, b) =>
            a.ref.localeCompare(b.ref),
          );
          yield* saveState(state);
        });

      const inspectId = (ref: string) =>
        cmd
          .run(["docker", "image", "inspect", ref, "--format", "{{.Id}}"], {
            allowFailure: true,
          })
          .pipe(
            Effect.map((r) => (r.code === 0 ? r.stdout.trim() : "")),
            Effect.catchAll(() => Effect.succeed("")),
          );

      return {
        maybeBuild: (component, state, saveState) =>
          Effect.gen(function* () {
            if (component === "coprocessor") {
              const doc = YAML.parse(
                yield* Effect.promise(() => fs.readFile(effectiveComposePath(component), "utf8")),
              ) as ComposeDoc;
              const services = Object.entries(doc.services)
                .filter(([, service]) => !!service.build)
                .map(([name]) => name);
              if (!services.length) {
                return;
              }
              yield* Effect.log("[build] coprocessor");
              const refs = imageRefsFromDoc(doc, services);
              for (const ref of refs) {
                yield* cmd.run(["docker", "image", "rm", "-f", ref], {
                  allowFailure: true,
                });
              }
              for (const service of services) {
                yield* containers.composeBuild(component, [service]);
              }
              yield* saveBuiltImages(
                state,
                saveState,
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
              if (!GROUP_BUILD_COMPONENTS[override.group].includes(component)) continue;

              // Parse compose file once for the entire override
              const doc = YAML.parse(
                yield* Effect.promise(() => fs.readFile(effectiveComposePath(component), "utf8")),
              ) as ComposeDoc;
              const available = new Set(Object.keys(doc.services));
              const candidates = override.services?.length
                ? override.services
                : GROUP_BUILD_SERVICES[override.group];
              const services = candidates.filter((s) => available.has(s));
              if (!services.length) continue;

              // Deduplicate services by image tag to avoid buildx conflicts
              const seen = new Set<string>();
              const deduped = services.filter((s) => {
                const img = doc.services[s]?.image;
                if (typeof img !== "string" || seen.has(img)) return false;
                seen.add(img);
                return true;
              });

              yield* Effect.log(`[build] ${override.group} (${component})`);

              // Remove existing images to force rebuild
              const refs = imageRefsFromDoc(doc, deduped);
              for (const ref of refs) {
                yield* cmd.run(["docker", "image", "rm", "-f", ref], {
                  allowFailure: true,
                });
              }

              // Build in batches (coprocessor builds one service at a time)
              const buildBatches =
                override.group === "coprocessor"
                  ? deduped.map((s) => [s])
                  : [deduped];
              for (const batch of buildBatches) {
                yield* containers.composeBuild(component, batch);
              }

              yield* saveBuiltImages(
                state,
                saveState,
                imageRefsFromDoc(doc, services).map((ref) => ({
                  ref,
                  group: override.group,
                })),
              );
            }
          }).pipe(
            Effect.mapError((e) => {
              if (e instanceof ContainerStartError) {
                return new BuildError({ component: e.component, stderr: e.stderr });
              }
              return new BuildError({ component, stderr: String(e) });
            }),
          ),

        inspectImageId: (ref) => inspectId(ref),

        removeImage: (ref) =>
          cmd
            .run(["docker", "image", "rm", ref], { allowFailure: true })
            .pipe(
              Effect.asVoid,
              Effect.catchAll(() => Effect.void),
            ),
      };
    }),
  );
}
