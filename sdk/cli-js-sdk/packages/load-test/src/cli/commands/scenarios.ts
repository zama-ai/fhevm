import type { Command } from "@commander-js/extra-typings";

export const registerScenarioCommands = (program: Command): void => {
  const scenarios = program.command("scenario").description("Inspect resolved scenarios");

  scenarios.command("list").description("List built-in scenarios").action(async () => {
    const [{ BUILTIN_SCENARIOS, createBuiltinScenario }, { logger }] = await Promise.all([
      import("../../scenario/builtin"), import("../../shared/logger"),
    ]);
    for (const name of BUILTIN_SCENARIOS) {
      const scenario = createBuiltinScenario(name);
      logger.info(`${name}: ${scenario.description}`);
    }
  });

  scenarios.command("show <ref>")
    .description("Print the resolved scenario JSON (built-in name or file path)")
    .action(async (ref) => {
      const { loadScenario } = await import("../../scenario/load");
      console.log(JSON.stringify(await loadScenario(ref), null, 2));
    });
};
