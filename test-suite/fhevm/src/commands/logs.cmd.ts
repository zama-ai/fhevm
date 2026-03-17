import { Command } from "@effect/cli";
import { Option } from "effect";
import { noFollowOption, serviceArg } from "../options";
import { logs } from "./logs";

export const logsCommand = Command.make(
  "logs",
  { noFollow: noFollowOption, service: serviceArg },
  ({ noFollow, service }) =>
    logs(Option.getOrUndefined(service), { follow: !noFollow }),
).pipe(
  Command.withDescription("Show logs for a stack container or a well-known alias."),
);
