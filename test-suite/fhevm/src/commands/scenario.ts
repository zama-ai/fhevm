import { Effect } from "effect";

import { listScenarioSummaries } from "../scenario";

export const listScenarios = () =>
  Effect.gen(function* () {
    const scenarios = yield* listScenarioSummaries();
    if (!scenarios.length) {
      yield* Effect.log("No bundled scenarios found.");
      return;
    }
    for (const scenario of scenarios) {
      const header =
        scenario.name && scenario.name !== scenario.key
          ? `${scenario.key} - ${scenario.name}`
          : scenario.key;
      yield* Effect.log(header);
      if (scenario.description) {
        yield* Effect.log(`  ${scenario.description}`);
      }
    }
  });
