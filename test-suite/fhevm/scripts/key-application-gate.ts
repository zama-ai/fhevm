#!/usr/bin/env bun
import { keyApplicationGateSql, KEY_GATE_WAITERS, DROP_KEY_GATE } from "../../e2e/test/consensus/keyApplicationGate";
const [command, key] = process.argv.slice(2);
if (command === "install") console.log(keyApplicationGateSql(key ?? ""));
else if (command === "waiters") console.log(KEY_GATE_WAITERS);
else if (command === "drop") console.log(DROP_KEY_GATE);
else throw new Error("unknown key gate SQL operation");
