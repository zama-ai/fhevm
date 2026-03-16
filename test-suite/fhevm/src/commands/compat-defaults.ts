/**
 * commands/compat-defaults.ts — The `compat-defaults` command handler.
 *
 * Outputs the COMPAT_MATRIX external defaults and anchors as JSON.
 * CI workflows read this to avoid hardcoding version pins.
 */
import { Effect } from "effect";

import { COMPAT_MATRIX } from "../compat";

export const compatDefaults = Effect.sync(() => {
  console.log(
    JSON.stringify({
      externalDefaults: COMPAT_MATRIX.externalDefaults,
      anchors: COMPAT_MATRIX.anchors,
    }),
  );
});
