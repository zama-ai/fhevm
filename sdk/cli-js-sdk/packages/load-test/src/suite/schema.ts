import { z } from "zod";

import { flowKindSchema } from "../scenario/schema";
import { artifactSlugSchema } from "../shared/paths";

/**
 * A suite is an ordered list of scenario runs executed sequentially (load
 * tests must never overlap — they contend for the same relayer capacity).
 * The suite runner derives pool requirements from the resolved scenarios and
 * prepares any deficit before the first run.
 */

export const suiteEntrySchema = z.object({
  /** Built-in scenario name or path to a scenario JSON file. */
  scenario: z.string().min(1),
  /** Built-in parameter overrides (same knobs as `run` CLI flags). */
  params: z
    .object({
      rps: z.number().positive().optional(),
      durationSec: z.number().positive().optional(),
      count: z.number().int().positive().optional(),
      flow: flowKindSchema.optional(),
    })
    .default({}),
  /** Report/baseline key; defaults to the resolved scenario name. */
  label: artifactSlugSchema.optional(),
});

export const suiteSchema = z.object({
  name: artifactSlugSchema,
  description: z.string().default(""),
  entries: z.array(suiteEntrySchema).min(1),
  /** Idle seconds between scenario runs, letting the relayer queue drain. */
  pauseSec: z.number().min(0).default(30),
});

export type SuiteEntry = z.infer<typeof suiteEntrySchema>;
export type Suite = z.infer<typeof suiteSchema>;
