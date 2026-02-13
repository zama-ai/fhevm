import chalk from "chalk";

export function info(msg: string): void {
  console.log(`${chalk.green("[INFO]")} ${msg}`);
}

export function warn(msg: string): void {
  console.log(`${chalk.yellow("[WARN]")} ${msg}`);
}

export function error(msg: string): void {
  console.error(`${chalk.red("[ERROR]")} ${msg}`);
}

export function step(id: string, msg: string): void {
  console.log(`${chalk.blueBright(`[STEP:${id}]`)} ${msg}`);
}

export function success(msg: string): void {
  console.log(`${chalk.green("[SUCCESS]")} ${msg}`);
}

export const log = { info, warn, error, step, success };
