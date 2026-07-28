import {
  doctorDemo,
  downDemo,
  logsDemo,
  reseedDemo,
  reseedThroughSupervisor,
  serveDemo,
  statusDemo,
  upDemo,
} from "./lifecycle";

const [command, ...args] = process.argv.slice(2);

const usage = (exitCode: number): never => {
  console.log(
    "usage: bun run demo <doctor|up|serve|status|logs|reseed|down> [native-process|owned-container|all] [--no-follow|--direct]",
  );
  process.exit(exitCode);
};

try {
  if (command === "--help" || command === "-h" || command === "help") {
    usage(0);
  } else if (command === "doctor") {
    const result = await doctorDemo();
    if (result.errors.length > 0) process.exitCode = 1;
  } else if (command === "up") {
    await upDemo();
  } else if (command === "serve") {
    await serveDemo();
  } else if (command === "status") {
    if (!(await statusDemo())) process.exitCode = 1;
  } else if (command === "logs") {
    await logsDemo(
      args.find((arg) => !arg.startsWith("--")) ?? "all",
      !args.includes("--no-follow"),
    );
  } else if (command === "reseed") {
    if (args.includes("--direct")) await reseedDemo();
    else await reseedThroughSupervisor();
  } else if (command === "down") {
    await downDemo();
  } else {
    usage(2);
  }
} catch (error) {
  console.error(error instanceof Error ? error.message : String(error));
  process.exitCode = 1;
}
