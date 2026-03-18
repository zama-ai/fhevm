import { Command } from "@effect/cli";
import { compatVersionArg, forceModernRelayerOption } from "../options";
import { compatResolveEnv } from "./compat-resolve-env";

export const compatResolveEnvCommand = Command.make(
  "compat-resolve-env",
  {
    forceModernRelayer: forceModernRelayerOption,
    versions: compatVersionArg,
  },
  ({ forceModernRelayer, versions }) => compatResolveEnv(versions, { forceModernRelayer }),
).pipe(
  Command.withDescription("Normalize workflow-facing stack-era and relayer env vars from selected refs."),
);
