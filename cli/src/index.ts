import { defineCommand, runMain } from "citty";

const main = defineCommand({
  meta: {
    name: "fhevm-cli",
    version: "0.1.0",
    description: "fhEVM local development stack manager",
  },
  subCommands: {
    up: () => import("./commands/up").then((module) => module.default),
    down: () => import("./commands/down").then((module) => module.default),
    clean: () => import("./commands/clean").then((module) => module.default),
    status: () => import("./commands/status").then((module) => module.default),
    logs: () => import("./commands/logs").then((module) => module.default),
    restart: () => import("./commands/restart").then((module) => module.default),
    test: () => import("./commands/test").then((module) => module.default),
    pause: () => import("./commands/pause").then((module) => module.default),
    unpause: () => import("./commands/unpause").then((module) => module.default),
    doctor: () => import("./commands/doctor").then((module) => module.default),
  },
});

await runMain(main);
